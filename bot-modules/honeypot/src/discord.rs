use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use moderation::{InfractionKind, NewInfraction};
use serenity::all::{CreateCommand, Message, UserId};
use zayden_app::state::AppState;
use zayden_core::{
    EventCtx,
    EventKind,
    HandlerError,
    InvocationCtx,
    Module,
    ModuleCommand,
    OverlapError,
    RegistryBuilder,
    Screen,
    as_i64,
};

use crate::{BAN_REASON, Honeypot, HoneypotHit, HoneypotOutcome, message_create};

const HONEYPOT_POINTS: i32 = 0;
const HONEYPOT_MODERATOR: &str = "Zayden (Honeypot)";

pub struct HoneypotModule;

impl HoneypotModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for HoneypotModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for HoneypotModule {
    fn id(&self) -> &'static str {
        "honeypot"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(HoneypotCmd);
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::Message]
    }

    async fn screen_message(
        &self,
        cx: &EventCtx<'_>,
        msg: &Message,
    ) -> Result<Screen, HandlerError> {
        let Some(guild_id) = msg.guild_id else {
            return Ok(Screen::Pass);
        };

        let settings = cx.app.settings.honeypot.get(as_i64(guild_id.get())).await?;

        if let Some(hit) = message_create(cx.ctx, msg, &settings).await? {
            record_hit(&cx.app, &hit).await?;
            return Ok(Screen::Consumed);
        }

        Ok(Screen::Pass)
    }
}

struct HoneypotCmd;

#[async_trait]
impl ModuleCommand for HoneypotCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("honeypot")
    }

    fn module(&self) -> Option<&'static str> {
        Some("honeypot")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Honeypot::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Honeypot::run(cx).await?;
        Ok(())
    }
}

async fn record_hit(app: &Arc<AppState>, hit: &HoneypotHit) -> sqlx::Result<()> {
    let kind = match hit.outcome {
        HoneypotOutcome::SoftBanned => InfractionKind::SoftBan,
        HoneypotOutcome::BanStanding => InfractionKind::Ban,
    };

    NewInfraction {
        guild_id: hit.guild_id,
        target_id: hit.user_id,
        target_username: &hit.username,
        kind,
        moderator_id: UserId::new(app.zayden_id),
        moderator_username: HONEYPOT_MODERATOR,
        points: HONEYPOT_POINTS,
        reason: BAN_REASON,
    }
    .record(&app.db)
    .await
}
