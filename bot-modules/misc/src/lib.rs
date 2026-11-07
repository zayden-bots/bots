use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

mod custom_msg;
mod random;

use custom_msg::CustomMsg;
use random::Random;

pub struct MiscModule;

impl MiscModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for MiscModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for MiscModule {
    fn id(&self) -> &'static str {
        "misc"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Random).add_command(CustomMsg);
        Ok(())
    }
}
