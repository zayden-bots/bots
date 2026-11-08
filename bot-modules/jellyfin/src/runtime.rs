use std::sync::Arc;

use reqwest::Client;
use secrecy::ExposeSecret;
use zayden_app::config::JellyfinConfig;
use zayden_core::{ModuleCtx, ModuleError};

use crate::cache::JellyfinCaches;
use crate::error::Result;
use crate::transport::{JellyfinClient, PlaybackClient, SeerClient};

pub fn runtime_from_config(
    cx: &ModuleCtx<'_>,
) -> std::result::Result<Option<Arc<JellyfinRuntime>>, ModuleError> {
    cx.config
        .jellyfin
        .as_ref()
        .map(|c| JellyfinRuntime::new(cx.app.http.clone(), c))
        .transpose()
        .map_err(|e| ModuleError::init("jellyfin", e))
}

#[derive(Debug)]
pub struct JellyfinRuntime {
    pub jellyfin: JellyfinClient,
    pub seer: SeerClient,
    pub playback: PlaybackClient,
    pub caches: JellyfinCaches,
    dddie_api_key: Option<String>,
}

impl JellyfinRuntime {
    pub fn new(http: Client, config: &JellyfinConfig) -> Result<Arc<Self>> {
        let jellyfin = JellyfinClient::new(
            http.clone(),
            config.internal_url.clone(),
            config.public_url.clone(),
            config.api_key.expose_secret().to_owned(),
            config.movie_library_id.clone(),
            config.show_library_id.clone(),
        );

        let playback = PlaybackClient::new(
            http.clone(),
            config.internal_url.clone(),
            config.api_key.expose_secret().to_owned(),
        );

        let seer = SeerClient::new(
            http,
            &config.seer_base_url,
            config.seer_api_key.expose_secret().to_owned(),
            config.region.clone(),
        )?;

        Ok(Arc::new(Self {
            jellyfin,
            seer,
            playback,
            caches: JellyfinCaches::new(),
            dddie_api_key: config
                .dddie_api_key
                .as_ref()
                .map(|key| key.expose_secret().to_owned()),
        }))
    }

    #[must_use]
    pub fn dddie_api_key(&self) -> Option<&str> {
        self.dddie_api_key.as_deref()
    }
}
