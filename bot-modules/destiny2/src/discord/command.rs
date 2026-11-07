use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use bungie_api::BungieClient;
use secrecy::ExposeSecret;
use serenity::all::{CreateCommand, GuildId};
use zayden_core::EmojiCacheData;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use super::ParentToken;
use crate::commands::Command as Destiny2Command;

pub(super) struct Destiny2<D> {
    pub(super) bungie_client: Arc<BungieClient>,
    pub(super) parent_token: Arc<dyn ParentToken>,
    pub(super) _data: PhantomData<fn() -> D>,
}

#[async_trait]
impl<D> ModuleCommand for Destiny2<D>
where
    D: EmojiCacheData + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("destiny2")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Destiny2Command::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let parent_token = self.parent_token.get(&cx.app.db).await?;

        Destiny2Command::run::<D>(
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
