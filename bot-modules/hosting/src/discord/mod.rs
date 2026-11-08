mod components;
mod slash_commands;

use std::sync::Arc;

use async_trait::async_trait;
use components::{HostingCancel, HostingConfirm, HostingSpec};
use secrecy::ExposeSecret;
use slash_commands::Server;
use zayden_app::entitlement::EntitlementService;
use zayden_core::error::HandlerError;
use zayden_core::{CronResult, Module, ModuleCtx, OverlapError, RegistryBuilder};

use crate::cron::{HostingDeleteCron, HostingExpireCron, HostingReminderCron};
use crate::{HostingError, HostingRuntime};

#[derive(Clone)]
pub(super) struct Runtime(Option<Arc<HostingRuntime>>);

impl Runtime {
    pub(super) fn get(&self) -> Result<Arc<HostingRuntime>, HandlerError> {
        self.0
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| HandlerError::from_respond(HostingError::Disabled))
    }
}

pub struct HostingModule {
    runtime: Runtime,
    entitlements: Arc<EntitlementService>,
}

impl HostingModule {
    #[must_use]
    pub fn from_config(cx: &ModuleCtx<'_>) -> Self {
        // Hosting needs the panel's admin-scoped key, so its client is built
        // here and never handed to the dashboard.
        let hosting =
            cx.config.pelican.as_ref().zip(cx.config.hosting.as_ref()).map(
                |(pelican, hosting)| {
                    let runtime = Arc::new(HostingRuntime::new(
                        cx.app.http.clone(),
                        &pelican.base_url,
                        pelican.api_key.expose_secret(),
                        hosting.clone(),
                    ));
                    HostingRuntime::spawn_paid_listener(
                        Arc::clone(&runtime),
                        cx.app.db.clone(),
                        cx.app.subscribe(),
                    );
                    runtime
                },
            );

        Self {
            runtime: Runtime(hosting),
            entitlements: Arc::clone(&cx.app.entitlements),
        }
    }
}

#[async_trait]
impl Module for HostingModule {
    fn id(&self) -> &'static str {
        "hosting"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Server { runtime: self.runtime.clone() })
            .add_autocomplete(Server { runtime: self.runtime.clone() })
            .add_modal(HostingSpec { runtime: self.runtime.clone() })?
            .add_component(HostingConfirm { runtime: self.runtime.clone() })?
            .add_component(HostingCancel)?;

        Ok(())
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        // Hosting is optional; without a catalog and panel credentials there
        // are no servers for these sweeps to act on.
        self.runtime.0.as_ref().map_or_else(Vec::new, |runtime| {
            vec![
                HostingReminderCron::cron_job(Arc::clone(runtime)),
                HostingExpireCron::cron_job(
                    Arc::clone(runtime),
                    Arc::clone(&self.entitlements),
                ),
                HostingDeleteCron::cron_job(Arc::clone(runtime)),
            ]
        })
    }
}
