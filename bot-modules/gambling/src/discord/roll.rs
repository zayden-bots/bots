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

pub(super) struct Roll<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for Roll<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("roll")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_roll()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::roll::<D>(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
