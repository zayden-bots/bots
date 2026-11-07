mod autocomplete;
mod components;
mod slash_commands;

use std::sync::Arc;

use async_trait::async_trait;
use autocomplete::TicketAutocomplete;
use components::{
    CreateTicketModal,
    SupportClose,
    SupportFaq,
    SupportSolved,
    SupportStillOpen,
    SupportTicket,
    TicketCreate,
};
use serenity::all::FullEvent;
use slash_commands::TicketCommand;
use tracing::debug;
use zayden_core::error::HandlerError;
use zayden_core::{EventCtx, EventKind, Module, OverlapError, RegistryBuilder};

use crate::WikiIndex;

pub struct TicketModule {
    wiki_index: Arc<WikiIndex>,
}

impl TicketModule {
    #[must_use]
    pub const fn new(wiki_index: Arc<WikiIndex>) -> Self {
        Self { wiki_index }
    }
}

#[async_trait]
impl Module for TicketModule {
    fn id(&self) -> &'static str {
        "ticket"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(TicketCommand)
            .add_autocomplete(TicketAutocomplete {
                wiki_index: Arc::clone(&self.wiki_index),
            })
            .add_component(TicketCreate)?
            .add_component(SupportTicket)?
            .add_component(SupportClose)?
            .add_component(SupportFaq)?
            .add_component(SupportSolved)?
            .add_component(SupportStillOpen)?
            .add_modal(CreateTicketModal)?;
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::Message, EventKind::ThreadCreate, EventKind::ThreadDelete]
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
            FullEvent::Message { new_message: msg, .. } => {
                if msg.author.bot() {
                    if cx
                        .ctx
                        .http
                        .application_id()
                        .is_some_and(|id| id.get() != msg.author.id.get())
                    {
                        crate::idle::events::message(msg, &cx.app).await?;
                    }
                    debug!(author_id = %msg.author.id, "message author is a bot; ignoring");
                    return Ok(());
                }
                tokio::try_join!(
                    async {
                        crate::SupportMessageCommand::run(&cx.ctx.http, msg, &cx.app)
                            .await
                            .map_err(HandlerError::from)
                    },
                    async {
                        crate::idle::events::message(msg, &cx.app)
                            .await
                            .map_err(HandlerError::from)
                    },
                )?;
            },
            FullEvent::ThreadCreate { thread, newly_created, .. } => {
                crate::SupportThreadCreate::run(
                    &cx.ctx.http,
                    thread,
                    *newly_created,
                    &cx.app,
                )
                .await?;
            },
            FullEvent::ThreadDelete { thread, .. } => {
                crate::idle::events::thread_delete(thread, &cx.app).await?;
            },
            _ => {},
        }
        Ok(())
    }
}
