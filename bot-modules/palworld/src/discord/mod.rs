mod autocomplete;
mod command;
mod modal;

use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

use crate::client::PalworldClient;

pub struct PalworldModule {
    client: Arc<PalworldClient>,
}

impl PalworldModule {
    #[must_use]
    pub const fn new(client: Arc<PalworldClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Module for PalworldModule {
    fn id(&self) -> &'static str {
        "palworld"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Palworld { client: Arc::clone(&self.client) })
            .add_autocomplete(autocomplete::Palworld {
                client: Arc::clone(&self.client),
            })
            .add_modal(modal::PalworldUploadModal {
                client: Arc::clone(&self.client),
            })?;
        Ok(())
    }
}
