use std::borrow::Cow;

use async_trait::async_trait;
use zayden_core::ctx::{ComponentCtx, ModalCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleComponent, ModuleModal};
use zayden_core::scope::IdMatch;

use super::Runtime;
use crate::components;
use crate::modal::{CANCEL_PREFIX, CONFIRM_PREFIX, MODAL_PREFIX};

async fn submit(cx: &ModalCtx<'_>, runtime: &Runtime) -> Result<(), HandlerError> {
    let runtime = runtime.get()?;

    components::spec_submitted(
        &cx.ctx.http,
        cx.interaction,
        &runtime,
        &cx.app.db,
        &cx.app.entitlements,
    )
    .await?;

    Ok(())
}

async fn confirm(
    cx: &ComponentCtx<'_>,
    runtime: &Runtime,
) -> Result<(), HandlerError> {
    let runtime = runtime.get()?;

    components::confirm(&cx.ctx.http, cx.interaction, &runtime, &cx.app.db).await?;

    Ok(())
}

pub(super) struct HostingSpec {
    pub(super) runtime: Runtime,
}

#[async_trait]
impl ModuleModal for HostingSpec {
    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(MODAL_PREFIX))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        submit(cx, &self.runtime).await
    }
}

pub(super) struct HostingConfirm {
    pub(super) runtime: Runtime,
}

#[async_trait]
impl ModuleComponent for HostingConfirm {
    fn module(&self) -> Option<&'static str> {
        Some("hosting")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(CONFIRM_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        confirm(cx, &self.runtime).await
    }
}

pub(super) struct HostingCancel;

#[async_trait]
impl ModuleComponent for HostingCancel {
    fn module(&self) -> Option<&'static str> {
        Some("hosting")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(CANCEL_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        components::cancel(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}
