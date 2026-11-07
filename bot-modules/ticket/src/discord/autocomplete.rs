use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::AutocompleteCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleAutocomplete;

use crate::{Ticket, WikiIndex};

pub(super) struct TicketAutocomplete {
    pub(super) wiki_index: Arc<WikiIndex>,
}

#[async_trait]
impl ModuleAutocomplete for TicketAutocomplete {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("ticket")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        Ticket::faq_autocomplete(
            &cx.ctx.http,
            cx.interaction,
            &cx.app,
            &self.wiki_index,
        )
        .await?;

        Ok(())
    }
}
