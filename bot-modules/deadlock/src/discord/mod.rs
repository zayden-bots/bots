mod autocomplete;
mod command;

use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::FullEvent;
use tracing::debug;
use zayden_core::error::HandlerError;
use zayden_core::{
    CronResult,
    EventCtx,
    EventKind,
    Module,
    ModuleCtx,
    ModuleError,
    OverlapError,
    RegistryBuilder,
    as_i64,
};

use crate::DeadlockApi;
use crate::cron::{
    DeadlockBriefingCron,
    DeadlockCacheGcCron,
    DeadlockLeaderboardCron,
    DeadlockMatchCron,
    DeadlockPatchCron,
    DeadlockRankCron,
};

pub struct DeadlockModule {
    api: Arc<DeadlockApi>,
    zayden_id: u64,
}

impl DeadlockModule {
    pub fn from_config(cx: &ModuleCtx<'_>) -> Result<Self, ModuleError> {
        let api = Arc::new(
            DeadlockApi::new(
                cx.app.http.clone(),
                cx.app.db.clone(),
                cx.config.deadlock_api_key.as_ref(),
            )
            .map_err(|e| ModuleError::init("deadlock", e))?,
        );
        Ok(Self { api, zayden_id: cx.app.zayden_id })
    }
}

#[async_trait]
impl Module for DeadlockModule {
    fn id(&self) -> &'static str {
        "deadlock"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Deadlock { api: Arc::clone(&self.api) })
            .add_autocomplete(autocomplete::Deadlock { api: Arc::clone(&self.api) });
        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![
            DeadlockPatchCron::cron_job(Arc::clone(&self.api), self.zayden_id),
            DeadlockMatchCron::cron_job(Arc::clone(&self.api), self.zayden_id),
            DeadlockRankCron::cron_job(Arc::clone(&self.api), self.zayden_id),
            DeadlockLeaderboardCron::cron_job(Arc::clone(&self.api), self.zayden_id),
            DeadlockBriefingCron::cron_job(),
            DeadlockCacheGcCron::cron_job(),
        ]
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::GuildMemberRemoval]
    }

    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        let FullEvent::GuildMemberRemoval { guild_id, user, .. } = ev else {
            return Ok(());
        };

        let removed = crate::db::guild_members::remove(
            &cx.app.db,
            as_i64(guild_id.get()),
            as_i64(user.id.get()),
        )
        .await?;

        if removed {
            debug!(%guild_id, user_id = %user.id, "Reset Deadlock opt-ins after the member left");
        }

        Ok(())
    }
}
