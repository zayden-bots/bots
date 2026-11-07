use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::DeadlockApi;
use crate::commands::Command as DeadlockCommand;

pub(super) struct Deadlock {
    pub(super) api: Arc<DeadlockApi>,
}

#[async_trait]
impl ModuleCommand for Deadlock {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("deadlock")
    }

    fn module(&self) -> Option<&'static str> {
        Some("deadlock")
    }

    fn definition(&self) -> CreateCommand<'static> {
        DeadlockCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        DeadlockCommand::run(cx, &self.api).await?;
        Ok(())
    }
}
