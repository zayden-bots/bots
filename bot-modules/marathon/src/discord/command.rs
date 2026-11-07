use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::client::MarathonClient;
use crate::commands::Command as MarathonCommand;

pub(super) struct Marathon {
    pub(super) client: Arc<MarathonClient>,
}

#[async_trait]
impl ModuleCommand for Marathon {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("marathon")
    }

    fn module(&self) -> Option<&'static str> {
        Some("marathon")
    }

    fn definition(&self) -> CreateCommand<'static> {
        MarathonCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        MarathonCommand::run(cx, &self.client).await?;
        Ok(())
    }
}
