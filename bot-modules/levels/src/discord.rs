use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::{CreateCommand, FullEvent, UserId};
use sqlx::PgPool;
use zayden_core::{
    ComponentCtx,
    EventCtx,
    EventKind,
    HandlerError,
    IdMatch,
    InvocationCtx,
    Module,
    ModuleCommand,
    ModuleComponent,
    OverlapError,
    RegistryBuilder,
};

#[async_trait]
pub trait LevelUpReward: Send + Sync {
    async fn on_level_up(
        &self,
        pool: &PgPool,
        user: UserId,
        level: i32,
    ) -> Result<(), HandlerError>;
}

pub struct LevelsModule {
    reward: Option<Arc<dyn LevelUpReward>>,
}

impl LevelsModule {
    #[must_use]
    pub fn new(reward: Option<Arc<dyn LevelUpReward>>) -> Self {
        Self { reward }
    }
}

#[async_trait]
impl Module for LevelsModule {
    fn id(&self) -> &'static str {
        "levels"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Levels)
            .add_command(Rank)
            .add_command(Xp)
            .add_component(Levels)?;
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::Message]
    }

    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        let FullEvent::Message { new_message, .. } = ev else {
            return Ok(());
        };
        if new_message.author.bot() {
            return Ok(());
        }
        if let Some(level) = crate::message_create(new_message, &cx.app.db).await?
            && let Some(reward) = &self.reward
        {
            reward.on_level_up(&cx.app.db, new_message.author.id, level).await?;
        }
        Ok(())
    }
}

struct Levels;

#[async_trait]
impl ModuleCommand for Levels {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("levels")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Levels::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        crate::Levels::run(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for Levels {
    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("levels"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::Levels::run_components(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct Rank;

#[async_trait]
impl ModuleCommand for Rank {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("rank")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Rank::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        crate::Rank::rank(&cx.ctx.http, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}

struct Xp;

#[async_trait]
impl ModuleCommand for Xp {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("xp")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Xp::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        crate::Xp::xp(&cx.ctx.http, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
