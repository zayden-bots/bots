use std::borrow::Cow;

use async_trait::async_trait;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

pub(super) struct FetchSuggestionsCmd;

#[async_trait]
impl ModuleCommand for FetchSuggestionsCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("fetch_suggestions")
    }

    fn definition(&self) -> serenity::all::CreateCommand<'static> {
        crate::FetchSuggestions::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::FetchSuggestions::run(
            &cx.ctx.http,
            cx.interaction,
            cx.interaction.data.options(),
            &cx.app.db,
        )
        .await?;
        Ok(())
    }
}
