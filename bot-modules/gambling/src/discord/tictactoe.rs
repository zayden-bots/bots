use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{
    ComponentCtx,
    HandlerError,
    IdMatch,
    InvocationCtx,
    ModuleCommand,
    ModuleComponent,
};

use crate::{Commands, GameCache};

pub(super) struct TicTacToe(pub(super) Arc<GameCache>);

#[async_trait]
impl ModuleCommand for TicTacToe {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("tictactoe")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_tictactoe()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::tictactoe(cx.ctx, cx.interaction, options, &cx.app.db, &self.0)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for TicTacToe {
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("ttt"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::components::TicTacToe::run_component(
            cx.ctx,
            cx.interaction,
            &cx.app.db,
        )
        .await?;
        Ok(())
    }
}
