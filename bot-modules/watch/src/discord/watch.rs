use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use jellyfin::RuntimeHandle;
use jellyfin::runtime::JellyfinRuntime;
use serenity::all::CreateCommand;
use zayden_core::ctx::{AutocompleteCtx, InvocationCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleAutocomplete, ModuleCommand};

async fn dispatch(
    cx: &InvocationCtx<'_>,
    runtime: &RuntimeHandle,
) -> Result<(), HandlerError> {
    let runtime = runtime.get()?;

    // The module cannot name `BotState`, so it hands back the cron work it
    // wants registered and this layer applies it.
    let scheduled = crate::Watch::run(cx, &runtime).await?;
    apply(cx.ctx, &runtime, scheduled).await;

    Ok(())
}

pub(super) async fn apply(
    ctx: &serenity::all::Context,
    runtime: &Arc<JellyfinRuntime>,
    scheduled: crate::party::Scheduled,
) {
    scheduled.apply(ctx, runtime).await;
}

pub(super) struct Watch {
    pub(super) runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleCommand for Watch {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("watch")
    }

    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Watch::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        // Delegating to a plain `async fn` rather than writing the body inline:
        // inside `async_trait` the compiler cannot prove the higher-ranked
        // lifetime bound for a dispatch this wide (rust-lang/rust#100013).
        dispatch(cx, &self.runtime).await
    }
}

#[async_trait]
impl ModuleAutocomplete for Watch {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("watch")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        crate::autocomplete::run(cx, &runtime).await?;
        Ok(())
    }
}
