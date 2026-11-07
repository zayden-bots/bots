use std::borrow::Cow;
use std::marker::PhantomData;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{
    ComponentCtx,
    EmojiCacheData,
    GuildMembersCache,
    HandlerError,
    IdMatch,
    InvocationCtx,
    ModuleCommand,
    ModuleComponent,
};

use crate::{Commands, GamblingData};

pub(super) struct Leaderboard<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for Leaderboard<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("leaderboard")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_leaderboard()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::leaderboard::<D>(cx.ctx, cx.interaction, options, &cx.app.db)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<D> ModuleComponent for Leaderboard<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("leaderboard"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::Leaderboard::run_component::<D>(cx.ctx, cx.interaction, &cx.app.db)
            .await?;
        Ok(())
    }
}
