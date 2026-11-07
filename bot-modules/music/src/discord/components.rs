use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::UserId;
use songbird::Songbird;
use zayden_core::ctx::ComponentCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleComponent;
use zayden_core::scope::IdMatch;

use crate::components::{
    CONTROL_PANEL_PREFIX,
    ControlPanel as MusicControlPanel,
    PanelCtx,
    PanelServices,
    QUEUE_PAGER_PREFIX,
    QueuePager as MusicQueuePager,
};
use crate::{MusicManager, TrackResolver};

pub(super) struct ControlPanel {
    pub(super) songbird: Arc<Songbird>,
    pub(super) music: Arc<MusicManager>,
    pub(super) resolver: Arc<dyn TrackResolver>,
}

#[async_trait]
impl ModuleComponent for ControlPanel {
    fn module(&self) -> Option<&'static str> {
        Some("music")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(CONTROL_PANEL_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let songbird = Arc::clone(&self.songbird);
        let music = Arc::clone(&self.music);
        let resolver = Arc::clone(&self.resolver);

        let settings = Arc::clone(&cx.app.settings.music);

        let ctx = PanelCtx::new(
            &cx.ctx.http,
            cx.interaction,
            UserId::new(cx.app.zayden_id),
            PanelServices {
                http: Arc::clone(&cx.ctx.http),
                songbird,
                music,
                resolver,
                settings,
            },
        )?;

        let action = cx
            .interaction
            .data
            .custom_id
            .strip_prefix(CONTROL_PANEL_PREFIX)
            .unwrap_or_default();

        MusicControlPanel::run(&ctx, action).await?;
        Ok(())
    }
}

pub(super) struct QueuePager {
    pub(super) songbird: Arc<Songbird>,
    pub(super) music: Arc<MusicManager>,
    pub(super) resolver: Arc<dyn TrackResolver>,
}

#[async_trait]
impl ModuleComponent for QueuePager {
    fn module(&self) -> Option<&'static str> {
        Some("music")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed(QUEUE_PAGER_PREFIX))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        let songbird = Arc::clone(&self.songbird);
        let music = Arc::clone(&self.music);
        let resolver = Arc::clone(&self.resolver);

        let settings = Arc::clone(&cx.app.settings.music);

        let ctx = PanelCtx::new(
            &cx.ctx.http,
            cx.interaction,
            UserId::new(cx.app.zayden_id),
            PanelServices {
                http: Arc::clone(&cx.ctx.http),
                songbird,
                music,
                resolver,
                settings,
            },
        )?;

        let page_suffix = cx
            .interaction
            .data
            .custom_id
            .strip_prefix(QUEUE_PAGER_PREFIX)
            .unwrap_or_default();

        MusicQueuePager::run(&ctx, page_suffix).await?;
        Ok(())
    }
}
