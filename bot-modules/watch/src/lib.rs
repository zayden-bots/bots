pub mod autocomplete;
pub mod commands;
pub mod components;
mod discord;
pub mod discovery;
pub mod embeds;
pub mod error;
pub mod events;
pub mod party;

pub use commands::Watch;
pub use discord::WatchModule;
pub use error::{Result, WatchError};
pub use party::JellyfinPartyReaperCron;
