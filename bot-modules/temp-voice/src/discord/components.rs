use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::{ComponentCtx, ModalCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleComponent, ModuleModal};
use zayden_core::scope::IdMatch;

use crate::VoiceStateCache;
use crate::components::{self, Components};

pub(super) struct VoiceClaim {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceClaim {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::CLAIM))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::claim(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceDelete {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceDelete {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::DELETE))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::delete(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceRename {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceRename {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::RENAME))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::rename(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceLimit {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceLimit {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::LIMIT))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::limit(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceBitrate {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceBitrate {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::BITRATE))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::bitrate(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoicePassword {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoicePassword {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::PASSWORD))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::password(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoicePrivacy {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoicePrivacy {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::PRIVACY))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::privacy(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceRegion {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceRegion {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::REGION))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::region(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceTransfer {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceTransfer {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::TRANSFER))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::transfer(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceTrust {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceTrust {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::TRUST))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::trust(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceKick {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceKick {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::KICK))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::kick(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoicePrivacyMenu {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoicePrivacyMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::PRIVACY_MENU))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::privacy_menu(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceRegionMenu {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceRegionMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::REGION_MENU))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::region_menu(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceTransferMenu {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceTransferMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::TRANSFER_MENU))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::transfer_menu(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceTrustMenu {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceTrustMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::TRUST_MENU))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::trust_menu(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceKickMenu {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleComponent for VoiceKickMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::KICK_MENU))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::kick_menu(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceRenameModal {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleModal for VoiceRenameModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::RENAME))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        Components::rename_submit(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceLimitModal {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleModal for VoiceLimitModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::LIMIT))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        Components::limit_submit(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoiceBitrateModal {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleModal for VoiceBitrateModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::BITRATE))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        Components::bitrate_submit(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}

pub(super) struct VoicePasswordModal {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleModal for VoicePasswordModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed(components::PASSWORD))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        Components::password_submit(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.db,
            &self.voice_states,
        )
        .await?;

        Ok(())
    }
}
