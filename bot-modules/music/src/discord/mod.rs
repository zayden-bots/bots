mod command;
mod components;

use std::sync::Arc;

use async_trait::async_trait;
use command::Music;
use components::{ControlPanel, QueuePager};
use serenity::all::FullEvent;
use songbird::Songbird;
use zayden_core::error::HandlerError;
use zayden_core::{EventCtx, EventKind, Module, OverlapError, RegistryBuilder};

use crate::{MusicManager, TrackResolver};

pub struct MusicModule {
    songbird: Arc<Songbird>,
    music: Arc<MusicManager>,
    resolver: Arc<dyn TrackResolver>,
}

impl MusicModule {
    #[must_use]
    pub const fn new(
        songbird: Arc<Songbird>,
        music: Arc<MusicManager>,
        resolver: Arc<dyn TrackResolver>,
    ) -> Self {
        Self { songbird, music, resolver }
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
        &[EventKind::VoiceStateUpdate]
    }

    async fn on_event(
        &self,
        _cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        let FullEvent::VoiceStateUpdate { new, .. } = ev else {
            return Ok(());
        };

        self.music.occupancy().update(new);
        Ok(())
    }
}
