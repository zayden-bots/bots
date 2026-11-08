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
use zayden_core::{
    CronResult,
    EventCtx,
    EventKind,
    Module,
    ModuleCtx,
    ModuleError,
    OverlapError,
    RegistryBuilder,
    StartCtx,
    require,
};

use crate::{
    SupportIdleCloseCron,
    SupportIdleCron,
    SupportIdleGcCron,
    SupportIdleStaleCron,
    WikiIndex,
};

pub struct TicketModule {
    wiki_index: Arc<WikiIndex>,
}

impl TicketModule {
    pub fn from_config(cx: &ModuleCtx<'_>) -> Result<Self, ModuleError> {
        require("ticket", "AI_PROVIDER_KEY", cx.config.ai_provider_key.as_ref())?;
        let wiki_index = Arc::new(WikiIndex::new(cx.app.http.clone()));
        WikiIndex::spawn_invalidator(&wiki_index, cx.app.subscribe());
        Ok(Self { wiki_index })
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

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![
            SupportIdleCron::cron_job(),
            SupportIdleCloseCron::cron_job(),
            SupportIdleStaleCron::cron_job(),
            SupportIdleGcCron::cron_job(),
        ]
    }

    fn on_start(&self, cx: &StartCtx<'_>) {
        let http = Arc::clone(&cx.ctx.http);
        let app = Arc::clone(&cx.app);
        let guilds =
            cx.ready.guilds.iter().map(|guild| guild.id).collect::<Vec<_>>();
        tokio::spawn(async move {
            crate::archive::rebuild(&http, &app, &guilds).await;
        });
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
