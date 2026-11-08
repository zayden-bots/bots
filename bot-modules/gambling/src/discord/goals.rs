use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Goals;

#[async_trait]
impl ModuleCommand for Goals {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("goals")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_goals()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::goals(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
