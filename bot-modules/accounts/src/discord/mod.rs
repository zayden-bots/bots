use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

mod command;
mod components;

use command::{Link, Unlink};
use components::{UnlinkCancel, UnlinkConfirm};

pub struct AccountsModule {
    dashboard_url: Arc<str>,
}

impl AccountsModule {
    #[must_use]
    pub const fn new(dashboard_url: Arc<str>) -> Self {
        Self { dashboard_url }
    }
}

#[async_trait]
impl Module for AccountsModule {
    fn id(&self) -> &'static str {
        "accounts"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Link { dashboard_url: Arc::clone(&self.dashboard_url) })
            .add_command(Unlink)
            .add_component(UnlinkConfirm)?
            .add_component(UnlinkCancel)?;

        Ok(())
    }
}
