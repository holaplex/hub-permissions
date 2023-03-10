use hub_core::prelude::*;
use ory_keto_client::{
    apis::{configuration::Configuration, relationship_api::create_relationship},
    models::{CreateRelationshipBody, SubjectSet},
};

use crate::{
    proto::{
        credential_events, organization_events,
        treasury_events::{self, DropCreated},
        CredentialEventKey, Member, OAuth2Client, OrganizationEventKey, Project, TreasuryEventKey,
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
                process_project_created_event(keto, k, payload).await
            },
            Some(organization_events::Event::MemberAdded(payload)) => {
                process_member_added_event(keto, payload).await
            },
            Some(_) | None => Ok(()),
        },

        Services::Treasuries(k, e) => match e.event {
            Some(treasury_events::Event::DropCreated(drop)) => {
                process_drop_created_event(keto, k, drop).await
            },
            Some(_) | None => Ok(()),
        },
        Services::Credentials(key, payload) => match payload.event {
            Some(credential_events::Event::Oauth2ClientCreated(c)) => {
                process_oauth2_client_created_event(keto, key, c).await
            },
            None => Ok(()),
        },
        _ => Ok(()),
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
                relation: "session".to_string(),
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
            relation: Some("owners".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.id.to_string(),
                namespace: "User".to_string(),
                relation: "session".to_string(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}

async fn process_project_created_event(
    keto: Configuration,
    key: OrganizationEventKey,
    payload: Project,
) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Project".to_string()),
            object: Some(payload.id.to_string()),
            relation: Some("parents".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: key.id.to_string(),
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
    payload: DropCreated,
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

async fn process_member_added_event(keto: Configuration, payload: Member) -> Result<()> {
    let relation = create_relationship(
        &keto,
        Some(&CreateRelationshipBody {
            namespace: Some("Organization".to_string()),
            object: Some(payload.organization_id.to_string()),
            relation: Some("editors".to_string()),
            subject_id: None,
            subject_set: Some(Box::new(SubjectSet {
                object: payload.user_id.to_string(),
                namespace: "User".to_string(),
                relation: String::default(),
            })),
        }),
    )
    .await?;

    info!("relation created {:?}", relation);

    Ok(())
}
