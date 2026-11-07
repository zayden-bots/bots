mod autocomplete;
mod cache_listener;
mod command;

use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;
use bungie_api::BungieClient;
pub use cache_listener::spawn_cache_listener;
use sqlx::PgPool;
use zayden_core::error::HandlerError;
use zayden_core::{EmojiCacheData, Module, OverlapError, RegistryBuilder};

#[async_trait]
pub trait ParentToken: Send + Sync {
    async fn get(&self, db: &PgPool) -> Result<String, HandlerError>;
}

pub struct Destiny2Module<D> {
    bungie_client: Arc<BungieClient>,
    parent_token: Arc<dyn ParentToken>,
    _data: PhantomData<fn() -> D>,
}

impl<D> Destiny2Module<D> {
    #[must_use]
    pub fn new(
        bungie_client: Arc<BungieClient>,
        parent_token: Arc<dyn ParentToken>,
    ) -> Self {
        Self { bungie_client, parent_token, _data: PhantomData }
    }
}

#[async_trait]
impl<D> Module for Destiny2Module<D>
where
    D: EmojiCacheData + Send + Sync + 'static,
{
    fn id(&self) -> &'static str {
        "destiny2"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(command::Destiny2::<D> {
            bungie_client: Arc::clone(&self.bungie_client),
            parent_token: Arc::clone(&self.parent_token),
            _data: PhantomData,
        })
        .add_autocomplete(autocomplete::Destiny2 {
            bungie_client: Arc::clone(&self.bungie_client),
        });
        Ok(())
    }
}
