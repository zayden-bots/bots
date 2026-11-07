use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::AutocompleteCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleAutocomplete;

use crate::client::PalworldClient;

pub(super) struct Palworld {
    pub(super) client: Arc<PalworldClient>,
}

#[async_trait]
impl ModuleAutocomplete for Palworld {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("palworld")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        crate::autocomplete::run(cx, &self.client).await?;
        Ok(())
    }
}
