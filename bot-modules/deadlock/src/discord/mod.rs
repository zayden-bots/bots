mod autocomplete;
mod command;

use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::FullEvent;
use tracing::debug;
use zayden_core::error::HandlerError;
use zayden_core::{
    EventCtx,
    EventKind,
    Module,
    OverlapError,
    RegistryBuilder,
    as_i64,
};

use crate::DeadlockApi;

pub struct DeadlockModule {
    api: Arc<DeadlockApi>,
}

impl DeadlockModule {
    #[must_use]
    pub const fn new(api: Arc<DeadlockApi>) -> Self {
        Self { api }
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
