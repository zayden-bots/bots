use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use bungie_api::BungieClient;
use secrecy::ExposeSecret;
use serenity::all::{CreateCommand, GuildId};
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use super::ParentToken;
use crate::commands::Command as Destiny2Command;

pub(super) struct Destiny2 {
    pub(super) bungie_client: Arc<BungieClient>,
    pub(super) parent_token: Arc<dyn ParentToken>,
}

#[async_trait]
impl ModuleCommand for Destiny2 {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("destiny2")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Destiny2Command::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let parent_token = self.parent_token.get(&cx.app.db).await?;

        Destiny2Command::run(
            cx.ctx,
            cx.interaction,
            &cx.app.db,
            &self.bungie_client,
            cx.app.google_api_key.expose_secret(),
            &parent_token,
            GuildId::new(cx.app.zayden_guild),
        )
        .await?;

        Ok(())
    }
}
