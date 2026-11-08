use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Lotto;

#[async_trait]
impl ModuleCommand for Lotto {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("lotto")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_lotto()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::lotto(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
