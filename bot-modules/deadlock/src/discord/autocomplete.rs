use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::AutocompleteCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleAutocomplete;

use crate::DeadlockApi;

pub(super) struct Deadlock {
    pub(super) api: Arc<DeadlockApi>,
}

#[async_trait]
impl ModuleAutocomplete for Deadlock {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("deadlock")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        crate::autocomplete::run(cx, &self.api).await?;
        Ok(())
    }
}
