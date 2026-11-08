use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Craft;

#[async_trait]
impl ModuleCommand for Craft {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("craft")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_craft()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::craft(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
