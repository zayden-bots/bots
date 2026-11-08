use std::sync::Arc;

use async_trait::async_trait;
use secrecy::ExposeSecret;
use zayden_app::config::BotConfig;
use zayden_core::ctx::StartCtx;
use zayden_core::{CronResult, Module, OverlapError, RegistryBuilder};

use crate::listener::spawn_patreon_listener;
use crate::{PatreonApp, PatreonPollCron};

#[must_use]
pub fn app_from_config(config: &BotConfig) -> Option<Arc<PatreonApp>> {
    config.patreon.as_ref().map(|p| {
        Arc::new(PatreonApp {
            client_id: p.client_id.clone(),
            client_secret: p.client_secret.expose_secret().to_owned(),
            redirect_uri: p.redirect_uri.clone(),
        })
    })
}

pub struct PatreonModule {
    http: reqwest::Client,
    app: Option<Arc<PatreonApp>>,
}

impl PatreonModule {
    #[must_use]
    pub const fn new(http: reqwest::Client, app: Option<Arc<PatreonApp>>) -> Self {
        Self { http, app }
    }
}

#[async_trait]
impl Module for PatreonModule {
    fn id(&self) -> &'static str {
        "patreon"
    }

    fn register(&self, _b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        self.app.as_ref().map_or_else(Vec::new, |app| {
            vec![PatreonPollCron::cron_job(self.http.clone(), Arc::clone(app))]
        })
    }

    fn on_start(&self, cx: &StartCtx<'_>) {
        if cx.primary {
            spawn_patreon_listener(cx.ctx.clone(), Arc::clone(&cx.app));
        }
    }
}
