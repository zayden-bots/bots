mod command;

use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use command::Jellyfin;
use zayden_core::ctx::ComponentCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleComponent;
use zayden_core::scope::IdMatch;
use zayden_core::{CronResult, Module, OverlapError, RegistryBuilder};

use crate::components::link_cancel;
use crate::quick_connect::CANCEL_PREFIX;
use crate::{
    JellyfinError,
    JellyfinGameRoundReaperCron,
    JellyfinIndexRefreshCron,
    JellyfinRollupCron,
    JellyfinRuntime,
};

#[derive(Clone)]
pub struct RuntimeHandle(Option<Arc<JellyfinRuntime>>);

impl RuntimeHandle {
    #[must_use]
    pub const fn new(runtime: Option<Arc<JellyfinRuntime>>) -> Self {
        Self(runtime)
    }

    pub fn get(&self) -> Result<Arc<JellyfinRuntime>, HandlerError> {
        self.0
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| HandlerError::from_respond(JellyfinError::NotConfigured))
    }

    #[must_use]
    pub const fn as_option(&self) -> Option<&Arc<JellyfinRuntime>> {
        self.0.as_ref()
    }
}

fn suffix(cx: &ComponentCtx<'_>, prefix: &str) -> String {
    cx.interaction.data.custom_id.strip_prefix(prefix).unwrap_or_default().to_owned()
}

struct LinkCancel {
    runtime: RuntimeHandle,
}

#[async_trait]
impl ModuleComponent for LinkCancel {
    fn module(&self) -> Option<&'static str> {
        Some("jellyfin")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(CANCEL_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let runtime = self.runtime.get()?;
        let suffix = suffix(cx, CANCEL_PREFIX);

        link_cancel::run(cx, &runtime, &suffix).await?;
        Ok(())
    }
}

pub struct JellyfinModule {
    runtime: RuntimeHandle,
}

impl JellyfinModule {
    #[must_use]
    pub const fn new(runtime: Option<Arc<JellyfinRuntime>>) -> Self {
        Self { runtime: RuntimeHandle::new(runtime) }
    }
}

#[async_trait]
impl Module for JellyfinModule {
    fn id(&self) -> &'static str {
        "jellyfin"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Jellyfin { runtime: self.runtime.clone() })
            .add_autocomplete(Jellyfin { runtime: self.runtime.clone() })
            .add_component(LinkCancel { runtime: self.runtime.clone() })?;

        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        self.runtime.as_option().map_or_else(Vec::new, |runtime| {
            vec![
                JellyfinIndexRefreshCron::cron_job(Arc::clone(runtime)),
                JellyfinRollupCron::cron_job(Arc::clone(runtime)),
                JellyfinGameRoundReaperCron::cron_job(),
            ]
        })
    }
}
