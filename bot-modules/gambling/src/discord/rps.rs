use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::{Commands, GameCache};

pub(super) struct RockPaperScissors(pub(super) Arc<GameCache>);

#[async_trait]
impl ModuleCommand for RockPaperScissors {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("rps")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_rps()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::rps(cx.ctx, cx.interaction, options, &cx.app.db, &self.0).await?;
        Ok(())
    }
}
