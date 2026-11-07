use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::{AutocompleteCtx, InvocationCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleAutocomplete, ModuleCommand};

use super::RuntimeHandle;
use crate::autocomplete;

pub(super) struct Jellyfin {
    pub(super) runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleCommand for Jellyfin {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("jellyfin")
    }

    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Jellyfin::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        crate::Jellyfin::run(cx, &runtime).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleAutocomplete for Jellyfin {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("jellyfin")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        autocomplete::run(cx, &runtime).await?;
        Ok(())
    }
}
