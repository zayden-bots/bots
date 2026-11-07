use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::commands::{LinkCommand, UnlinkCommand};

pub(super) struct Link {
    pub(super) dashboard_url: Arc<str>,
}

#[async_trait]
impl ModuleCommand for Link {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("link")
    }

    fn definition(&self) -> CreateCommand<'static> {
        LinkCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        LinkCommand::run(cx.ctx, cx.interaction, &cx.app.db, &self.dashboard_url)
            .await?;
        Ok(())
    }
}

pub(super) struct Unlink;

#[async_trait]
impl ModuleCommand for Unlink {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("unlink")
    }

    fn definition(&self) -> CreateCommand<'static> {
        UnlinkCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        UnlinkCommand::run(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
