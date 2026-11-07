use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::{CreateCommand, GuildId};
use zayden_core::{CommandScope, HandlerError, InvocationCtx, ModuleCommand};

pub(super) struct RaidReport {
    pub(super) guild: GuildId,
}

#[async_trait]
impl ModuleCommand for RaidReport {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("raidreport")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::RaidReport::register()
    }

    fn scope(&self) -> CommandScope {
        CommandScope::Guilds(Cow::Owned(vec![self.guild]))
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::RaidReport::run(cx.ctx, cx.interaction).await?;
        Ok(())
    }
}
