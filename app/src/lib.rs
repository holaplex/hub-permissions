#![deny(clippy::disallowed_methods, clippy::suspicious, clippy::style)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]

pub mod events;

use hub_core::{clap, consumer::RecvError, prelude::*};

#[allow(clippy::pedantic)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/organization.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/customer.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/treasury.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/credential.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/webhook.proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/nfts.proto.rs"));
}

#[derive(Debug)]
pub enum Services {
    Organizations(proto::OrganizationEventKey, proto::OrganizationEvents),
    Customers(proto::CustomerEventKey, proto::CustomerEvents),
    Treasuries(proto::TreasuryEventKey, proto::TreasuryEvents),
    Credentials(proto::CredentialEventKey, proto::CredentialEvents),
    Webhooks(proto::WebhookEventKey, proto::WebhookEvents),
    Nfts(proto::NftEventKey, proto::NftEvents),
}

impl hub_core::consumer::MessageGroup for Services {
    const REQUESTED_TOPICS: &'static [&'static str] = &[
        "hub-orgs",
        "hub-customers",
        "hub-treasuries",
        "hub-credentials",
        "hub-webhooks",
        "hub-nfts",
    ];

    fn from_message<M: hub_core::consumer::Message>(msg: &M) -> Result<Self, RecvError> {
        let topic = msg.topic();
        let key = msg.key().ok_or(RecvError::MissingKey)?;
        let val = msg.payload().ok_or(RecvError::MissingPayload)?;
        info!(topic, ?key, ?val);

        match topic {
            "hub-orgs" => {
                let key = proto::OrganizationEventKey::decode(key)?;
                let val = proto::OrganizationEvents::decode(val)?;

                Ok(Services::Organizations(key, val))
            },
            "hub-customers" => {
                let key = proto::CustomerEventKey::decode(key)?;
                let val = proto::CustomerEvents::decode(val)?;

                Ok(Services::Customers(key, val))
            },
            "hub-treasuries" => {
                let key = proto::TreasuryEventKey::decode(key)?;
                let val = proto::TreasuryEvents::decode(val)?;

                Ok(Services::Treasuries(key, val))
            },
            "hub-credentials" => {
                let key = proto::CredentialEventKey::decode(key)?;
                let val = proto::CredentialEvents::decode(val)?;

                Ok(Services::Credentials(key, val))
            },
            "hub-webhooks" => {
                let key = proto::WebhookEventKey::decode(key)?;
                let val = proto::WebhookEvents::decode(val)?;

                Ok(Services::Webhooks(key, val))
            },
            "hub-nfts" => {
                let key = proto::NftEventKey::decode(key)?;
                let val = proto::NftEvents::decode(val)?;

                Ok(Services::Nfts(key, val))
            },
            t => Err(RecvError::BadTopic(t.into())),
        }
    }
}

#[derive(Debug, clap::Args)]
#[command(version, author, about)]
pub struct Args {
    #[arg(short, long, env)]
    pub keto_write_url: String,
}
