use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{HandlerError, InvocationCtx, ModuleCommand};

use crate::Commands;

pub(super) struct Profile;

#[async_trait]
impl ModuleCommand for Profile {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("profile")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_profile()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::profile(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}
