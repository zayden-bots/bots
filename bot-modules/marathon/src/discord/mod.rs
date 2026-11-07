mod autocomplete;
mod command;

use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

use crate::client::MarathonClient;

pub struct MarathonModule {
    client: Arc<MarathonClient>,
}

impl MarathonModule {
    #[must_use]
    pub const fn new(client: Arc<MarathonClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Module for MarathonModule {
    fn id(&self) -> &'static str {
        "marathon"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Marathon { client: Arc::clone(&self.client) })
            .add_autocomplete(autocomplete::Marathon {
                client: Arc::clone(&self.client),
            });
        Ok(())
    }
}
