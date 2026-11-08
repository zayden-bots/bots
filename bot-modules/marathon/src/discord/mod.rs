mod autocomplete;
mod command;

use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use zayden_core::{CronResult, Module, ModuleCtx, OverlapError, RegistryBuilder};

use crate::client::MarathonClient;
use crate::cron::{MarathonAnnounceCron, MarathonNewsCron};

pub struct MarathonModule {
    client: Arc<MarathonClient>,
    http: reqwest::Client,
    bungie_api_key: Option<SecretString>,
}

impl MarathonModule {
    #[must_use]
    pub fn from_config(cx: &ModuleCtx<'_>) -> Self {
        let client = Arc::new(MarathonClient::new(
            cx.app.http.clone(),
            cx.config.flaresolverr_url.clone(),
        ));
        Self {
            client,
            http: cx.app.http.clone(),
            bungie_api_key: cx.config.bungie_api_key.clone(),
        }
    }
}

#[async_trait]
impl Module for MarathonModule {
    fn id(&self) -> &'static str {
        "marathon"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Marathon { client: Arc::clone(&self.client) })
            .add_autocomplete(autocomplete::Marathon {
                client: Arc::clone(&self.client),
            });
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![
            MarathonAnnounceCron::cron_job(Arc::clone(&self.client)),
            MarathonNewsCron::cron_job(
                self.http.clone(),
                self.bungie_api_key.as_ref().map(|k| k.expose_secret().to_owned()),
            ),
        ]
    }
}
