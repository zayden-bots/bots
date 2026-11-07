use std::borrow::Cow;
use std::marker::PhantomData;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use tracing::debug;
use zayden_core::{
    ComponentCtx,
    EmojiCacheData,
    GuildMembersCache,
    HandlerError,
    IdMatch,
    InvocationCtx,
    ModuleCommand,
    ModuleComponent,
    message_metadata,
};

use crate::{Commands, GamblingData};

pub(super) struct HigherLower<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for HigherLower<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("higherorlower")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_higher_lower()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::higher_lower::<D>(cx.ctx, cx.interaction, options, &cx.app.db)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<D> ModuleComponent for HigherLower<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("hol"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let metadata = message_metadata(&cx.interaction.message)?;

        if cx.interaction.user != metadata.user {
            debug!(
                user_id = %cx.interaction.user.id,
                owner_id = %metadata.user.id,
                "user does not own this higher/lower message; ignoring"
            );
            return Ok(());
        }

        crate::components::HigherLower::run_components::<D>(
            cx.ctx,
            cx.interaction,
            &cx.app.db,
        )
        .await?;
        Ok(())
    }
}
