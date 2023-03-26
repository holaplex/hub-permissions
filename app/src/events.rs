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
        credential_events, customer_events, organization_events, treasury_events, webhook_events,
        CredentialEventKey, Customer, CustomerEventKey, Member, OAuth2Client, OrganizationEventKey,
        Project, TreasuryEventKey, Webhook, WebhookEventKey,
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
        Services::Organizations(k, e) => match e.event {
            Some(organization_events::Event::OrganizationCreated(_)) => {
                process_org_created_event(keto, k).await
            },
            Some(organization_events::Event::ProjectCreated(payload)) => {
                process_project_created_event(keto, payload).await
            },
            Some(organization_events::Event::MemberAdded(payload)) => {
                process_member_added_event(keto, k, payload).await
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
