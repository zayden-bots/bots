use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Work;

#[async_trait]
impl ModuleCommand for Work {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("work")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_work()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::work(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
