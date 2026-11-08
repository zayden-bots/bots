mod command;
mod components;

use std::sync::Arc;

use async_trait::async_trait;
use command::Music;
use components::{ControlPanel, QueuePager};
use serenity::all::FullEvent;
use songbird::Songbird;
use zayden_core::error::HandlerError;
use zayden_core::{
    EventCtx,
    EventKind,
    Module,
    ModuleCtx,
    ModuleError,
    OverlapError,
    RegistryBuilder,
};

use crate::resolver_config::build_resolver;
use crate::{MusicManager, TrackResolver};

pub struct MusicModule {
    songbird: Arc<Songbird>,
    music: Arc<MusicManager>,
    resolver: Arc<dyn TrackResolver>,
}

impl MusicModule {
    pub async fn from_config(
        cx: &ModuleCtx<'_>,
        songbird: Arc<Songbird>,
    ) -> Result<Self, ModuleError> {
        let music = Arc::new(MusicManager::new());
        let resolver = build_resolver(cx.config).await?;
        Ok(Self { songbird, music, resolver })
    }
}

#[async_trait]
impl Module for MusicModule {
    fn id(&self) -> &'static str {
        "music"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Music {
            songbird: Arc::clone(&self.songbird),
            manager: Arc::clone(&self.music),
            resolver: Arc::clone(&self.resolver),
        })
        .add_component(ControlPanel {
            songbird: Arc::clone(&self.songbird),
            music: Arc::clone(&self.music),
            resolver: Arc::clone(&self.resolver),
        })?
        .add_component(QueuePager {
            songbird: Arc::clone(&self.songbird),
            music: Arc::clone(&self.music),
            resolver: Arc::clone(&self.resolver),
        })?;

        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::VoiceStateUpdate, EventKind::GuildCreate]
    }

    async fn on_event(
        &self,
        _cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        if let FullEvent::VoiceStateUpdate { new, .. } = ev {
            self.music.occupancy().update(new);
        } else if let FullEvent::GuildCreate { guild, .. } = ev {
            self.music.occupancy().guild_create(guild);
        }
        Ok(())
    }
}
