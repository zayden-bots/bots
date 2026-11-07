use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::{CreateCommand, GuildId};
use zayden_core::{CommandScope, HandlerError, InvocationCtx, ModuleCommand};

pub(super) struct Goof {
    pub(super) guild: GuildId,
}

#[async_trait]
impl ModuleCommand for Goof {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("goof")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Goof::register()
    }

    fn scope(&self) -> CommandScope {
        CommandScope::Guilds(Cow::Owned(vec![self.guild]))
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::Goof::run(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
