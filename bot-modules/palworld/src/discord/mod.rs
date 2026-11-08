mod autocomplete;
mod command;
mod modal;

use std::sync::Arc;

use async_trait::async_trait;
use secrecy::ExposeSecret;
use zayden_core::{
    CronResult,
    Module,
    ModuleCtx,
    OverlapError,
    RegistryBuilder,
    StartCtx,
};

use crate::client::PalworldClient;
use crate::cron::{
    PalworldSaveRefreshCron,
    PalworldUploadSweepCron,
    PalworldWarmCron,
};
use crate::transport::Pelican;

pub struct PalworldModule {
    client: Arc<PalworldClient>,
}

impl PalworldModule {
    #[must_use]
    pub fn from_config(cx: &ModuleCtx<'_>) -> Self {
        let pelican = cx.config.pelican.clone().and_then(|p| {
            let save = p.save?;
            Some(Pelican::new(
                cx.app.http.clone(),
                p.base_url,
                p.api_key.expose_secret().to_owned(),
                save.server_id,
                save.save_path,
            ))
        });

        let client = Arc::new(PalworldClient::new(
            cx.app.http.clone(),
            cx.config.flaresolverr_url.clone(),
            cx.config.palworld_paldex_url.clone(),
            cx.config.palworld_palcalc_url.clone(),
            cx.config.palworld_save_dir.clone(),
            cx.config.palworld_uploads_dir.clone(),
            pelican,
        ));
        Self { client }
    }
}

#[async_trait]
impl Module for PalworldModule {
    fn id(&self) -> &'static str {
        "palworld"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Palworld { client: Arc::clone(&self.client) })
            .add_autocomplete(autocomplete::Palworld {
                client: Arc::clone(&self.client),
            })
            .add_modal(modal::PalworldUploadModal {
                client: Arc::clone(&self.client),
            })?;
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![
            PalworldUploadSweepCron::cron_job(),
            PalworldSaveRefreshCron::cron_job(Arc::clone(&self.client)),
            PalworldWarmCron::cron_job(Arc::clone(&self.client)),
        ]
    }

    fn on_start(&self, _cx: &StartCtx<'_>) {
        let client = Arc::clone(&self.client);
        tokio::spawn(async move { client.warm().await });
    }
}
