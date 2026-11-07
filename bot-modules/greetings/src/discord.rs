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

use crate::{register as register_command, run};

pub struct GreetingsModule;

impl GreetingsModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for GreetingsModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for GreetingsModule {
    fn id(&self) -> &'static str {
        "greetings"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(GoodCmd);
        Ok(())
    }
}

struct GoodCmd;

#[async_trait]
impl ModuleCommand for GoodCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("good")
    }

    fn module(&self) -> Option<&'static str> {
        Some("greetings")
    }

    fn definition(&self) -> CreateCommand<'static> {
        register_command()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        run(cx, &cx.app.settings.greetings).await?;
        Ok(())
    }
}
