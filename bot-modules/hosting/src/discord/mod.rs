mod components;
mod slash_commands;

use std::sync::Arc;

use async_trait::async_trait;
use components::{HostingCancel, HostingConfirm, HostingSpec};
use slash_commands::Server;
use zayden_core::error::HandlerError;
use zayden_core::{Module, OverlapError, RegistryBuilder};

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
}

impl HostingModule {
    #[must_use]
    pub const fn new(runtime: Option<Arc<HostingRuntime>>) -> Self {
        Self { runtime: Runtime(runtime) }
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
}
