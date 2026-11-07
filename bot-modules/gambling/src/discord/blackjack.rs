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

use crate::components::BlackjackCustomId;
use crate::{Commands, GamblingData};

pub(super) struct Blackjack<D>(pub(super) PhantomData<fn() -> D>);

#[async_trait]
impl<D> ModuleCommand for Blackjack<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("blackjack")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_blackjack()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::blackjack::<D>(cx.ctx, cx.interaction, options, &cx.app.db)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<D> ModuleComponent for Blackjack<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("blackjack"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let metadata = message_metadata(&cx.interaction.message)?;

        if cx.interaction.user != metadata.user {
            debug!(
                user_id = %cx.interaction.user.id,
                owner_id = %metadata.user.id,
                "user does not own this blackjack message; ignoring"
            );
            return Ok(());
        }

        match cx.interaction.data.custom_id.parse::<BlackjackCustomId>()? {
            BlackjackCustomId::Hit => {
                crate::components::Blackjack::hit::<D>(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                )
                .await?;
            },
            BlackjackCustomId::Stand => {
                crate::components::Blackjack::stand::<D>(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                )
                .await?;
            },
            BlackjackCustomId::Double => {
                crate::components::Blackjack::double::<D>(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                )
                .await?;
            },
            BlackjackCustomId::Split => {
                crate::components::Blackjack::split::<D>(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                )
                .await?;
            },
            BlackjackCustomId::Surrender => {
                crate::components::Blackjack::surrender::<D>(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                )
                .await?;
            },
            BlackjackCustomId::Hand { .. } | BlackjackCustomId::Dealer { .. } => {
                debug!(
                    custom_id = %cx.interaction.data.custom_id,
                    "ignoring click on a blackjack status badge"
                );
            },
        }

        Ok(())
    }
}
