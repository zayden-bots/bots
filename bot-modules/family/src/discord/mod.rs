use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

mod command;
mod components;

use command::Family;
use components::{AdoptAccept, AdoptDecline, MarryAccept, MarryDecline};

pub struct FamilyModule;

impl FamilyModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for FamilyModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for FamilyModule {
    fn id(&self) -> &'static str {
        "family"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Family)
            .add_component(MarryAccept)?
            .add_component(MarryDecline)?
            .add_component(AdoptAccept)?
            .add_component(AdoptDecline)?;
        Ok(())
    }
}
