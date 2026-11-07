use async_trait::async_trait;
use zayden_core::{Module, OverlapError, RegistryBuilder};

mod infraction;
mod logs;
mod rules;

use infraction::Infraction;
use logs::Logs;
use rules::RulesCommand;

pub struct ModerationModule;

impl ModerationModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ModerationModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for ModerationModule {
    fn id(&self) -> &'static str {
        "moderation"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Infraction).add_command(Logs).add_command(RulesCommand);
        Ok(())
    }
}
