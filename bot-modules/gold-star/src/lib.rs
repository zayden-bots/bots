pub mod commands;
mod discord;
pub use discord::GoldStarModule;
pub mod error;
pub mod manager;

pub use commands::{GiveStar, Stars};
pub use error::GoldStarError;
use error::Result;
pub use manager::GoldStarRow;
