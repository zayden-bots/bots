use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::ctx::{AutocompleteCtx, InvocationCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleAutocomplete, ModuleCommand};

use super::Runtime;
use crate::commands::Hosting;

async fn dispatch(
    cx: &InvocationCtx<'_>,
    runtime: &Runtime,
) -> Result<(), HandlerError> {
    let runtime = runtime.get()?;

    Hosting::run(
        &cx.ctx.http,
        cx.interaction,
        &runtime,
        &cx.app.db,
        &cx.app.entitlements,
        cx.interaction.data.options(),
    )
    .await?;

    Ok(())
}

async fn complete(
    cx: &AutocompleteCtx<'_>,
    runtime: &Runtime,
) -> Result<(), HandlerError> {
    let runtime = runtime.get()?;

    Hosting::autocomplete(&cx.ctx.http, cx.interaction, &runtime).await?;
    Ok(())
}

pub(super) struct Server {
    pub(super) runtime: Runtime,
}

#[async_trait]
impl ModuleCommand for Server {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("server")
    }

    fn module(&self) -> Option<&'static str> {
        Some("hosting")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Hosting::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        // Delegating to a plain `async fn`: inside `async_trait` the compiler
        // cannot prove the higher-ranked lifetime bound for a dispatch this
        // wide (rust-lang/rust#100013).
        dispatch(cx, &self.runtime).await
    }
}

#[async_trait]
impl ModuleAutocomplete for Server {
    fn command(&self) -> Cow<'static, str> {
        Cow::Borrowed("server")
    }

    async fn run(&self, cx: &AutocompleteCtx<'_>) -> Result<(), HandlerError> {
        complete(cx, &self.runtime).await
    }
}
