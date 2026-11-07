use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use bungie_api::BungieClient;
use secrecy::ExposeSecret;
use zayden_core::ctx::AutocompleteCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleAutocomplete;

use crate::commands::Command as Destiny2Command;

pub(super) struct Destiny2 {
    pub(super) bungie_client: Arc<BungieClient>,
}

#[async_trait]
impl ModuleAutocomplete for Destiny2 {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("destiny2")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        let Some(option) = cx.interaction.data.autocomplete() else {
            return Ok(());
        };

        Destiny2Command::autocomplete(
            cx.ctx,
            cx.interaction,
            option,
            &cx.app.db,
            &self.bungie_client,
            cx.app.google_api_key.expose_secret(),
        )
        .await?;

        Ok(())
    }
}
