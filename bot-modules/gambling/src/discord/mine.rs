use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Mine;

#[async_trait]
impl ModuleCommand for Mine {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("mine")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_mine()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::mine(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
