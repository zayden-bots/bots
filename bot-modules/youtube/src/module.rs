use std::sync::Arc;

use async_trait::async_trait;
use secrecy::ExposeSecret;
use zayden_app::config::BotConfig;
use zayden_core::ctx::StartCtx;
use zayden_core::{CronResult, Module, OverlapError, RegistryBuilder};

use crate::listener::spawn_youtube_listener;
use crate::{YoutubeLeaseCron, YoutubePollCron, YoutubeRuntime};

#[must_use]
pub fn runtime_from_config(config: &BotConfig) -> Option<YoutubeRuntime> {
    let youtube = config.youtube.as_ref()?;
    let Some(google_api_key) = config.google_api_key.as_ref() else {
        tracing::warn!(
            "youtube is configured but GOOGLE_API_KEY is not set; youtube is disabled"
        );
        return None;
    };
    Some(YoutubeRuntime {
        api_key: Arc::from(google_api_key.expose_secret()),
        webhook_uri: Arc::from(youtube.webhook_uri.as_str()),
    })
}

pub struct YoutubeModule {
    http: reqwest::Client,
    runtime: Option<YoutubeRuntime>,
}

impl YoutubeModule {
    #[must_use]
    pub const fn new(
        http: reqwest::Client,
        runtime: Option<YoutubeRuntime>,
    ) -> Self {
        Self { http, runtime }
    }
}

#[async_trait]
impl Module for YoutubeModule {
    fn id(&self) -> &'static str {
        "youtube"
    }

    fn register(&self, _b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        self.runtime.as_ref().map_or_else(Vec::new, |runtime| {
            vec![
                YoutubePollCron::cron_job(
                    self.http.clone(),
                    Arc::clone(&runtime.api_key),
                ),
                YoutubeLeaseCron::cron_job(
                    self.http.clone(),
                    Arc::clone(&runtime.webhook_uri),
                ),
            ]
        })
    }

    fn on_start(&self, cx: &StartCtx<'_>) {
        if cx.primary
            && let Some(runtime) = self.runtime.clone()
        {
            spawn_youtube_listener(cx.ctx.clone(), Arc::clone(&cx.app), runtime);
        }
    }
}
