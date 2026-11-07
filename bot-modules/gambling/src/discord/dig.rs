use std::borrow::Cow;
use std::marker::PhantomData;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{
    EmojiCacheData,
    GuildMembersCache,
    HandlerError,
    InvocationCtx,
    ModuleCommand,
};

use crate::{Commands, GamblingData};

pub(super) struct Dig<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for Dig<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("dig")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_dig()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::dig::<D>(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
