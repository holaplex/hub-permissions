//!

use holaplex_hub_permissions::{events, Args, Services};
use hub_core::{
    prelude::*,
    tokio::{self, task},
};
use ory_keto_client::{self, apis::configuration::Configuration};

pub fn main() {
    let opts = hub_core::StartConfig {
        service_name: "hub-orgs",
    };

    hub_core::run(opts, |common, args| {
        let Args { keto_write_url } = args;

        let keto = Configuration {
            base_path: keto_write_url,
            user_agent: None,
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };

        common.rt.block_on(async move {
            let cons = common.consumer_cfg.build::<Services>().await?;

            let mut stream = cons.stream();
            loop {
                let keto = keto.clone();
                match stream.next().await {
                    Some(Ok(msg)) => {
                        info!(?msg, "message received");

                        tokio::spawn(async move { events::process(msg, keto).await });
                        task::yield_now().await;
                    },
                    None => (),
                    Some(Err(e)) => {
                        warn!("failed to get message {:?}", e);
                    },
                }
            }
        })
    });
}
