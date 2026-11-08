use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct SendCmd;

#[async_trait]
impl ModuleCommand for SendCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("send")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_send()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::send(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
