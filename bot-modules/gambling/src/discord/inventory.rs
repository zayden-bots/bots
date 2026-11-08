use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Inventory;

#[async_trait]
impl ModuleCommand for Inventory {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("inventory")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_inventory()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::inventory(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
