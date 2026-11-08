use std::borrow::Cow;

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

use crate::Commands;

pub(super) struct Leaderboard;

#[async_trait]
impl ModuleCommand for Leaderboard {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("leaderboard")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_leaderboard()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::leaderboard(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for Leaderboard {
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("leaderboard"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::Leaderboard::run_component(cx.ctx, cx.interaction, &cx.app.db)
            .await?;
        Ok(())
    }
}
