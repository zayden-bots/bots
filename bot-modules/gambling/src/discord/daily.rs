use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Daily;

#[async_trait]
impl ModuleCommand for Daily {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("daily")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_daily()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::daily(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
