mod watch;

use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use jellyfin::RuntimeHandle;
use jellyfin::components::{
    GUESS_MODAL_PREFIX,
    GUESS_OPEN_PREFIX,
    TRIVIA_ANSWER_PREFIX,
    game,
};
use jellyfin::runtime::JellyfinRuntime;
use serenity::all::FullEvent;
use zayden_core::ctx::{ComponentCtx, ModalCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleComponent, ModuleModal};
use zayden_core::scope::IdMatch;
use zayden_core::{
    CronResult,
    EventCtx,
    EventKind,
    Module,
    OverlapError,
    RegistryBuilder,
};

use self::watch::{Watch, apply};
use crate::JellyfinPartyReaperCron;
use crate::components::{
    PARTY_CANCEL_PREFIX,
    PARTY_JOIN_PREFIX,
    PARTY_LEAVE_PREFIX,
    party,
};

fn suffix(cx: &ComponentCtx<'_>, prefix: &str) -> String {
    cx.interaction.data.custom_id.strip_prefix(prefix).unwrap_or_default().to_owned()
}

struct PartyJoin {
    runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleComponent for PartyJoin {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(PARTY_JOIN_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        let suffix = suffix(cx, PARTY_JOIN_PREFIX);

        party::join(cx, &runtime, &suffix).await?;
        Ok(())
    }
}

struct PartyLeave {
    runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleComponent for PartyLeave {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(PARTY_LEAVE_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        let suffix = suffix(cx, PARTY_LEAVE_PREFIX);

        party::leave(cx, &runtime, &suffix).await?;
        Ok(())
    }
}

struct PartyCancel {
    runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleComponent for PartyCancel {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(PARTY_CANCEL_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        let suffix = suffix(cx, PARTY_CANCEL_PREFIX);

        let scheduled = party::cancel(cx, &runtime, &suffix).await?;
        apply(cx.ctx, &runtime, scheduled).await;

        Ok(())
    }
}

struct TriviaAnswer;

#[async_trait]
impl ModuleComponent for TriviaAnswer {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(TRIVIA_ANSWER_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let suffix = cx
            .interaction
            .data
            .custom_id
            .strip_prefix(TRIVIA_ANSWER_PREFIX)
            .unwrap_or_default()
            .to_owned();

        game::trivia_answer(cx, &suffix).await?;
        Ok(())
    }
}

struct GuessOpen;

#[async_trait]
impl ModuleComponent for GuessOpen {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(GUESS_OPEN_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let suffix = cx
            .interaction
            .data
            .custom_id
            .strip_prefix(GUESS_OPEN_PREFIX)
            .unwrap_or_default()
            .to_owned();

        game::open_guess_modal(cx, &suffix).await?;
        Ok(())
    }
}

struct GuessSubmit;

#[async_trait]
impl ModuleModal for GuessSubmit {
    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(GUESS_MODAL_PREFIX))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        let suffix = cx
            .interaction
            .data
            .custom_id
            .strip_prefix(GUESS_MODAL_PREFIX)
            .unwrap_or_default()
            .to_owned();

        game::guess_submit(cx, &suffix).await?;
        Ok(())
    }
}

pub struct WatchModule {
    runtime: RuntimeHandle,
}

impl WatchModule {
    #[must_use]
    pub const fn new(runtime: Option<Arc<JellyfinRuntime>>) -> Self {
        Self { runtime: RuntimeHandle::new(runtime) }
    }
}

#[async_trait]
impl Module for WatchModule {
    fn id(&self) -> &'static str {
        "watch"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Watch { runtime: self.runtime.clone() })
            .add_autocomplete(Watch { runtime: self.runtime.clone() })
            .add_component(PartyJoin { runtime: self.runtime.clone() })?
            .add_component(PartyLeave { runtime: self.runtime.clone() })?
            .add_component(PartyCancel { runtime: self.runtime.clone() })?
            .add_component(TriviaAnswer)?
            .add_component(GuessOpen)?
            .add_modal(GuessSubmit)?;

        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::GuildCreate]
    }

    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        let FullEvent::GuildCreate { guild, .. } = ev else {
            return Ok(());
        };

        // Party provisioning and cleanup are lost with the in-memory cron jobs,
        // so anything the bot was down for has to be caught up here.
        if let Some(runtime) = self.runtime.as_option() {
            crate::events::guild_create(cx.ctx, runtime, &cx.app.db, guild.id).await;
        }

        Ok(())
    }

    // Guest accounts are real accounts on a real media server, so the
    // reaper is registered alongside the refresh jobs rather than being
    // left to the in-memory per-party schedules.
    fn cron_jobs(&self) -> Vec<CronResult> {
        self.runtime.as_option().map_or_else(Vec::new, |runtime| {
            vec![
                JellyfinPartyReaperCron::cron_job(Arc::clone(runtime)),
                JellyfinPartyReaperCron::reconcile_job(Arc::clone(runtime)),
            ]
        })
    }
}
