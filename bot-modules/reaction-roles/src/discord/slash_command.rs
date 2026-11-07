use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

pub(super) struct ReactionRoleCmd;

#[async_trait]
impl ModuleCommand for ReactionRoleCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("reaction_role")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::ReactionRoleCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::ReactionRoleCommand::run(&cx.ctx.http, cx.interaction, &cx.app.db)
            .await?;
        Ok(())
    }
}
