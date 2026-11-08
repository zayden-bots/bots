mod commands;
mod components;

use std::sync::Arc;

use async_trait::async_trait;
use commands::Voice;
use components::{
    VoiceBitrate,
    VoiceBitrateModal,
    VoiceClaim,
    VoiceDelete,
    VoiceKick,
    VoiceKickMenu,
    VoiceLimit,
    VoiceLimitModal,
    VoicePassword,
    VoicePasswordModal,
    VoicePrivacy,
    VoicePrivacyMenu,
    VoiceRegion,
    VoiceRegionMenu,
    VoiceRename,
    VoiceRenameModal,
    VoiceTransfer,
    VoiceTransferMenu,
    VoiceTrust,
    VoiceTrustMenu,
};
use serenity::all::FullEvent;
use zayden_core::error::HandlerError;
use zayden_core::{EventCtx, EventKind, Module, OverlapError, RegistryBuilder};

use crate::VoiceStateCache;
use crate::events::voice_state_update::{channel_creator, channel_deleter};

pub struct TempVoiceModule {
    voice_states: Arc<VoiceStateCache>,
}

impl TempVoiceModule {
    #[must_use]
    pub fn new() -> Self {
        Self { voice_states: Arc::new(VoiceStateCache::new()) }
    }
}

impl Default for TempVoiceModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for TempVoiceModule {
    fn id(&self) -> &'static str {
        "temp_voice"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        let vs = &self.voice_states;

        b.add_command(Voice { voice_states: Arc::clone(vs) })
            .add_component(VoiceClaim { voice_states: Arc::clone(vs) })?
            .add_component(VoiceDelete { voice_states: Arc::clone(vs) })?
            .add_component(VoiceRename { voice_states: Arc::clone(vs) })?
            .add_component(VoiceLimit { voice_states: Arc::clone(vs) })?
            .add_component(VoiceBitrate { voice_states: Arc::clone(vs) })?
            .add_component(VoicePassword { voice_states: Arc::clone(vs) })?
            .add_component(VoicePrivacy { voice_states: Arc::clone(vs) })?
            .add_component(VoiceRegion { voice_states: Arc::clone(vs) })?
            .add_component(VoiceTransfer { voice_states: Arc::clone(vs) })?
            .add_component(VoiceTrust { voice_states: Arc::clone(vs) })?
            .add_component(VoiceKick { voice_states: Arc::clone(vs) })?
            .add_component(VoicePrivacyMenu { voice_states: Arc::clone(vs) })?
            .add_component(VoiceRegionMenu { voice_states: Arc::clone(vs) })?
            .add_component(VoiceTransferMenu { voice_states: Arc::clone(vs) })?
            .add_component(VoiceTrustMenu { voice_states: Arc::clone(vs) })?
            .add_component(VoiceKickMenu { voice_states: Arc::clone(vs) })?
            .add_modal(VoiceRenameModal { voice_states: Arc::clone(vs) })?
            .add_modal(VoiceLimitModal { voice_states: Arc::clone(vs) })?
            .add_modal(VoiceBitrateModal { voice_states: Arc::clone(vs) })?
            .add_modal(VoicePasswordModal { voice_states: Arc::clone(vs) })?;

        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::VoiceStateUpdate, EventKind::GuildCreate]
    }

    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        if let FullEvent::GuildCreate { guild, .. } = ev {
            self.voice_states.guild_create(guild);
            return Ok(());
        }

        let FullEvent::VoiceStateUpdate { new, .. } = ev else {
            return Ok(());
        };

        let old = self.voice_states.update(new)?;

        futures::try_join!(
            channel_creator(&cx.ctx.http, &cx.app.db, new),
            channel_deleter(cx.ctx, &cx.app.db, &self.voice_states, old.as_ref()),
        )?;

        Ok(())
    }
}
