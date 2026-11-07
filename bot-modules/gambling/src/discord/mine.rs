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

pub(super) struct Mine<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for Mine<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("mine")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_mine()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::mine::<D>(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
