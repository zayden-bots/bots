use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::{Commands, GameCache};

pub(super) struct Coinflip(pub(super) Arc<GameCache>);

#[async_trait]
impl ModuleCommand for Coinflip {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("coinflip")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_coinflip()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::coinflip(cx.ctx, cx.interaction, options, &cx.app.db, &self.0)
            .await?;
        Ok(())
    }
}
