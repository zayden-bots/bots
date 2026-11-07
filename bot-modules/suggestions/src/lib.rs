mod components;
mod discord;
pub mod error;
pub mod layout;
pub mod manager;
mod message;
mod modal;
mod reaction;
pub mod slash_command;

pub use discord::SuggestionsModule;
use error::Result;
pub use error::SuggestionsError;
pub use manager::SuggestionsGuildRow;
pub use reaction::{ReviewAction, ReviewThresholds, review_action};
pub use slash_command::FetchSuggestions;

pub struct Suggestions;
