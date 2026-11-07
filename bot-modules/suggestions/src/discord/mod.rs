use async_trait::async_trait;
use serenity::all::FullEvent;
use zayden_core::{
    EventCtx,
    EventKind,
    HandlerError,
    Module,
    OverlapError,
    RegistryBuilder,
};

use crate::Suggestions;

mod components;
mod slash_command;

use slash_command::FetchSuggestionsCmd;

pub struct SuggestionsModule;

impl SuggestionsModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for SuggestionsModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for SuggestionsModule {
    fn id(&self) -> &'static str {
        "suggestions"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(FetchSuggestionsCmd)
            .add_component(components::SuggestionsAccept)?
            .add_component(components::SuggestionsReject)?
            .add_modal(components::SuggestionsAcceptModal)?
            .add_modal(components::SuggestionsRejectModal)?;
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::Message, EventKind::ReactionAdd, EventKind::ReactionRemove]
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "FullEvent is non_exhaustive; unrouted kinds are ignored"
    )]
    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        match ev {
            FullEvent::Message { new_message, .. } => {
                if new_message.author.bot() {
                    return Ok(());
                }
                Suggestions::message(&cx.ctx.http, new_message, &cx.app.db).await?;
            },
            FullEvent::ReactionAdd { add_reaction, .. } => {
                Suggestions::reaction(&cx.ctx.http, add_reaction, &cx.app.db)
                    .await?;
            },
            FullEvent::ReactionRemove { removed_reaction, .. } => {
                Suggestions::reaction(&cx.ctx.http, removed_reaction, &cx.app.db)
                    .await?;
            },
            _ => {},
        }
        Ok(())
    }
}
