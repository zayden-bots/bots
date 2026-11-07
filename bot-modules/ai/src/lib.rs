pub mod chat;
mod discord;
pub mod error;
pub mod openai;
pub mod persona;
pub mod response;

pub use discord::AiModule;
pub use error::AiError as Error;
