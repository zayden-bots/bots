use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::Ticket;

pub(super) struct TicketCommand;

#[async_trait]
impl ModuleCommand for TicketCommand {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("ticket")
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Ticket::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Ticket::run(
            &cx.ctx.http,
            cx.interaction,
            &cx.app,
            cx.interaction.data.options(),
        )
        .await?;
        Ok(())
    }
}
