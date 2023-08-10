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
        credential_events, customer_events, nft_events, organization_events, solana_nft_events,
        webhook_events, CollectionCreation, CreationStatus, CredentialEventKey, Customer,
        CustomerEventKey, DropCreation, Member, MintCollectionCreation, MintCreation, NftEventKey,
        OAuth2Client, OrganizationEventKey, Project, SolanaMintPayload, SolanaNftEventKey,
        SolanaUpdatedMintPayload, Webhook, WebhookEventKey,
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
                process_member_reactivated_event(keto, key, payload).await
            },
            Some(_) | None => Ok(()),
        },

        Services::Customers(k, e) => match e.event {
            Some(customer_events::Event::Created(payload)) => {
                process_customer_added_event(keto, k, payload).await
            },
            Some(_) | None => Ok(()),
        },

        Services::Treasuries(_k, _e) => Ok(()),
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
            Some(nft_events::Event::DropMinted(payload)) => {
                process_nfts_mint_drop_event(keto, key, payload).await
            },
            Some(nft_events::Event::DropCreated(payload)) => {
                process_drop_created_event(keto, key, payload).await
            },
            Some(nft_events::Event::CollectionCreated(payload)) => {
                process_collection_created_event(keto, key, payload).await
            },
            Some(nft_events::Event::MintedToCollection(payload)) => {
                process_nfts_mint_to_collection_event(keto, key, payload).await
            },
            Some(nft_events::Event::SolanaMintUpdated(payload)) => {
                process_solana_mint_updated(keto, key, payload).await
            },
            Some(_) | None => Ok(()),
        },
        Services::SolanaNfts(key, payload) => match payload.event {
            Some(solana_nft_events::Event::ImportedExternalCollection(_)) => {
                process_solana_collection_imported_event(keto, key).await
            },
            Some(solana_nft_events::Event::ImportedExternalMint(payload)) => {
                process_solana_mint_imported_event(keto, key, payload).await
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
    key: NftEventKey,
    payload: DropCreation,
) -> Result<()> {
    let status =
        CreationStatus::from_i32(payload.status).ok_or(anyhow!("creation status not found"))?;

    if status == CreationStatus::Completed || status == CreationStatus::Failed {
        return Ok(());
    }

    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Drop".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.project_id.to_string(),
                namespace: "Project".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_collection_created_event(
    keto: Configuration,
    key: NftEventKey,
    payload: CollectionCreation,
) -> Result<()> {
    let status =
        CreationStatus::from_i32(payload.status).ok_or(anyhow!("creation status not found"))?;

    if status == CreationStatus::Completed || status == CreationStatus::Failed {
        return Ok(());
    }

    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Collection".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.project_id.to_string(),
                namespace: "Project".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_solana_collection_imported_event(
    keto: Configuration,
    key: SolanaNftEventKey,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Collection".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.project_id.to_string(),
                namespace: "Project".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}
async fn process_solana_mint_imported_event(
    keto: Configuration,
    key: SolanaNftEventKey,
    payload: SolanaMintPayload,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Mint".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.collection_id.to_string(),
                namespace: "Collection".to_string(),
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

    info!("Org User Permission relation created {:?}", relation);
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Member".to_string()),
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
    info!("Org Member Parent relation created {:?}", relation);
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

    info!(
        "User permission relation deleted for user {:?}",
        key.user_id
    );
    Ok(())
}

async fn process_member_reactivated_event(
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

    info!("User Permission relation created {:?}", relation);
    Ok(())
}

async fn process_nfts_mint_drop_event(
    keto: Configuration,
    key: NftEventKey,
    payload: MintCreation,
) -> Result<()> {
    let status =
        CreationStatus::from_i32(payload.status).ok_or(anyhow!("creation status not found"))?;

    if status == CreationStatus::Completed || status == CreationStatus::Failed {
        return Ok(());
    }

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

async fn process_nfts_mint_to_collection_event(
    keto: Configuration,
    key: NftEventKey,
    payload: MintCollectionCreation,
) -> Result<()> {
    let status =
        CreationStatus::from_i32(payload.status).ok_or(anyhow!("creation status not found"))?;

    if status == CreationStatus::Completed || status == CreationStatus::Failed {
        return Ok(());
    }

    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Mint".to_string()),
            object: Some(key.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.collection_id.to_string(),
                namespace: "Collection".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_solana_mint_updated(
    keto: Configuration,
    key: NftEventKey,
    payload: SolanaUpdatedMintPayload,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("UpdateHistories".to_string()),
            object: Some(key.id),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.mint_id,
                namespace: "Mint".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}
