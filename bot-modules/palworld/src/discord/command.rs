use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::client::PalworldClient;
use crate::commands::Command as PalworldCommand;

pub(super) struct Palworld {
    pub(super) client: Arc<PalworldClient>,
}

#[async_trait]
impl ModuleCommand for Palworld {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("palworld")
    }

    fn module(&self) -> Option<&'static str> {
        Some("palworld")
    }

    fn definition(&self) -> CreateCommand<'static> {
        PalworldCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        PalworldCommand::run(cx, &self.client, &cx.app.db).await?;
        Ok(())
    }
}
