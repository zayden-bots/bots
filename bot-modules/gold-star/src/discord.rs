use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{
    HandlerError,
    InvocationCtx,
    Module,
    ModuleCommand,
    OverlapError,
    RegistryBuilder,
};

use crate::{GiveStar, Stars};

pub struct GoldStarModule;

impl GoldStarModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for GoldStarModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for GoldStarModule {
    fn id(&self) -> &'static str {
        "gold_star"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(GiveStarCmd);
        b.add_command(StarsCmd);
        Ok(())
    }
}

struct GiveStarCmd;

#[async_trait]
impl ModuleCommand for GiveStarCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("give_star")
    }

    fn definition(&self) -> CreateCommand<'static> {
        GiveStar::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        GiveStar::run(
            &cx.ctx.http,
            cx.interaction,
            cx.interaction.data.options(),
            &cx.app.db,
        )
        .await?;
        Ok(())
    }
}

struct StarsCmd;

#[async_trait]
impl ModuleCommand for StarsCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("stars")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Stars::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Stars::run(
            &cx.ctx.http,
            cx.interaction,
            cx.interaction.data.options(),
            &cx.app.db,
        )
        .await?;
        Ok(())
    }
}
