use hub_core::prelude::*;
use ory_keto_client::{
    apis::{
        configuration::Configuration,
        relationship_api::{create_relationship, delete_relationships},
    },
    models::{CreateRelationshipBody, SubjectSet},
};

use crate::{
    proto::{
        credential_events, customer_events, nft_events, organization_events, treasury_events,
        webhook_events, CredentialEventKey, Customer, CustomerEventKey, Member, MintTransaction,
        NftEventKey, OAuth2Client, OrganizationEventKey, Project, TreasuryEventKey, Webhook,
        WebhookEventKey,
    },
    Services,
};
/// Res
///
/// # Errors
/// This function fails if ...
#[allow(clippy::too_many_lines)]
pub async fn process(msg: Services, keto: Configuration) -> Result<()> {
    // match topics
    match msg {
        Services::Organizations(key, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(_)) => {
                process_org_created_event(keto, key).await
            },
            Some(organization_events::Event::ProjectCreated(payload)) => {
                process_project_created_event(keto, payload).await
            },
            Some(organization_events::Event::MemberAdded(payload)) => {
                process_member_added_event(keto, key, payload).await
            },
            Some(organization_events::Event::MemberDeactivated(payload)) => {
                process_member_deactivated_event(keto, key, payload).await
            },
            Some(organization_events::Event::MemberReactivated(payload)) => {
                process_member_added_event(keto, key, payload).await
            },
            Some(_) | None => Ok(()),
        },

        Services::Customers(k, e) => match e.event {
            Some(customer_events::Event::Created(payload)) => {
                process_customer_added_event(keto, k, payload).await
            },
            None => Ok(()),
        },

        Services::Treasuries(k, e) => match e.event {
            Some(treasury_events::Event::DropCreated(drop)) => {
                process_drop_created_event(keto, k, drop).await
            },
            Some(_) | None => Ok(()),
        },
        Services::Credentials(key, payload) => match payload.event {
            Some(credential_events::Event::Oauth2ClientCreated(payload)) => {
                process_oauth2_client_created_event(keto, key, payload).await
            },
            Some(credential_events::Event::Oauth2ClientDeleted(payload)) => {
                process_oauth2_client_deleted_event(keto, key, payload).await
            },
            None => Ok(()),
        },
        Services::Webhooks(key, payload) => match payload.event {
            Some(webhook_events::Event::Created(payload)) => {
                process_webhooks_created_event(keto, key, payload).await
            },
            Some(webhook_events::Event::Deleted(payload)) => {
                process_webhook_deleted_event(keto, key, payload).await
            },
            None => Ok(()),
        },
        Services::Nfts(key, payload) => match payload.event {
            Some(nft_events::Event::MintDrop(payload)) => {
                process_nfts_mint_drop_event(keto, key, payload).await
            },
            Some(_) | None => Ok(()),
        },
    }
}

async fn process_org_created_event(keto: Configuration, key: OrganizationEventKey) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Organization".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("owners".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.user_id.to_string(),
                namespace: "User".to_string(),
                relation: String::new(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_oauth2_client_created_event(
    keto: Configuration,
    key: CredentialEventKey,
    payload: OAuth2Client,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Organization".to_string()),
            object: Some(payload.organization.to_string()),
            relation: Some("editors".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.id.to_string(),
                namespace: "User".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Credential".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.organization.to_string(),
                namespace: "Organization".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_oauth2_client_deleted_event(
    keto: Configuration,
    key: CredentialEventKey,
    payload: OAuth2Client,
) -> Result<()> {
    delete_relationships(
        &keto,
        Some("Organization"),
        Some(&payload.organization),
        Some("editors"),
        None,
        Some("User"),
        Some(&key.id),
        Some(""),
    )
    .await?;

    delete_relationships(
        &keto,
        Some("Credential"),
        Some(&key.id),
        Some("parents"),
        None,
        Some("Organization"),
        Some(&payload.organization),
        Some(""),
    )
    .await?;

    Ok(())
}

async fn process_project_created_event(keto: Configuration, payload: Project) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Project".to_string()),
            object: Some(payload.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.organization_id.to_string(),
                namespace: "Organization".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_drop_created_event(
    keto: Configuration,
    key: TreasuryEventKey,
    payload: treasury_events::DropCreated,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Drop".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.project_id.to_string(),
                namespace: "Project".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_member_added_event(
    keto: Configuration,
    key: OrganizationEventKey,
    payload: Member,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Organization".to_string()),
            object: Some(payload.organization_id.to_string()),
            relation: Some("editors".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.user_id.to_string(),
                namespace: "User".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_customer_added_event(
    keto: Configuration,
    key: CustomerEventKey,
    payload: Customer,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Customer".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.project_id.to_string(),
                namespace: "Project".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_webhooks_created_event(
    keto: Configuration,
    key: WebhookEventKey,
    payload: Webhook,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Webhook".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.organization_id.to_string(),
                namespace: "Organization".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_webhook_deleted_event(
    keto: Configuration,
    key: WebhookEventKey,
    payload: Webhook,
) -> Result<()> {
    delete_relationships(
        &keto,
        Some("Webhook"),
        Some(&key.id),
        Some("parents"),
        None,
        Some("Organization"),
        Some(&payload.organization_id),
        Some(""),
    )
    .await
    .map_err(Into::into)
}

async fn process_member_deactivated_event(
    keto: Configuration,
    key: OrganizationEventKey,
    payload: Member,
) -> Result<()> {
    delete_relationships(
        &keto,
        Some("Organization"),
        Some(&payload.organization_id),
        Some("editors"),
        None,
        Some("User"),
        Some(&key.user_id),
        Some(""),
    )
    .await?;

    info!("relation deleted for user {:?}", key.user_id);

    Ok(())
}

async fn process_nfts_mint_drop_event(
    keto: Configuration,
    key: NftEventKey,
    payload: MintTransaction,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Mint".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.drop_id.to_string(),
                namespace: "Drop".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}
