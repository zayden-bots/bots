use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::AutocompleteCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleAutocomplete;

use crate::client::MarathonClient;

pub(super) struct Marathon {
    pub(super) client: Arc<MarathonClient>,
}

#[async_trait]
impl ModuleAutocomplete for Marathon {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("marathon")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        crate::autocomplete::run(cx, &self.client).await?;
        Ok(())
    }
}
