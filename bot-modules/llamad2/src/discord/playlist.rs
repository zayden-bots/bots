use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::{CreateCommand, GuildId};
use zayden_core::{CommandScope, HandlerError, InvocationCtx, ModuleCommand};

pub(super) struct Playlist {
    pub(super) guild: GuildId,
}

#[async_trait]
impl ModuleCommand for Playlist {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("playlist")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Playlist::register()
    }

    fn scope(&self) -> CommandScope {
        CommandScope::Guilds(Cow::Owned(vec![self.guild]))
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::Playlist::run(cx.ctx, cx.interaction).await?;
        Ok(())
    }
}
