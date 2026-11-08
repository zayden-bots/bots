mod autocomplete;
mod cache_listener;
mod command;

use std::sync::Arc;

use async_trait::async_trait;
use bungie_api::{BungieClient, BungieClientBuilder};
pub use cache_listener::spawn_cache_listener;
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use zayden_core::error::HandlerError;
use zayden_core::{
    CronResult,
    Module,
    ModuleCtx,
    ModuleError,
    OverlapError,
    RegistryBuilder,
    StartCtx,
    require,
};

use crate::endgame_analysis::EndgameAnalysisSheetCron;

#[async_trait]
pub trait ParentToken: Send + Sync {
    async fn get(&self, db: &PgPool) -> Result<String, HandlerError>;
}

pub struct Destiny2Module {
    bungie_client: Arc<BungieClient>,
    parent_token: Arc<dyn ParentToken>,
    google_api_key: SecretString,
}

impl Destiny2Module {
    pub fn from_config(
        cx: &ModuleCtx<'_>,
        parent_token: Arc<dyn ParentToken>,
    ) -> Result<Self, ModuleError> {
        let bungie_api_key = require(
            "destiny2",
            "BUNGIE_API_KEY",
            cx.config.bungie_api_key.as_ref(),
        )?;
        let google_api_key = require(
            "destiny2",
            "GOOGLE_API_KEY",
            cx.config.google_api_key.as_ref(),
        )?;
        let bungie_client =
            BungieClientBuilder::new(bungie_api_key.expose_secret().to_owned())
                .build()
                .map_err(|e| ModuleError::init("destiny2", e))?;

        Ok(Self {
            bungie_client: Arc::new(bungie_client),
            parent_token,
            google_api_key: google_api_key.clone(),
        })
    }
}

#[async_trait]
impl Module for Destiny2Module {
    fn id(&self) -> &'static str {
        "destiny2"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Destiny2 {
            bungie_client: Arc::clone(&self.bungie_client),
            parent_token: Arc::clone(&self.parent_token),
        })
        .add_autocomplete(autocomplete::Destiny2 {
            bungie_client: Arc::clone(&self.bungie_client),
        });
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![EndgameAnalysisSheetCron::cron_job(
            Arc::clone(&self.bungie_client),
            self.google_api_key.expose_secret().to_owned(),
        )]
    }

    fn on_start(&self, cx: &StartCtx<'_>) {
        spawn_cache_listener(Arc::clone(&cx.app));
    }
}
