use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::{CreateCommand, UserId};
use songbird::Songbird;
use zayden_core::ctx::InvocationCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleCommand;

use crate::commands::{Command as MusicCommand, MusicCtx, MusicServices};
use crate::{MusicManager, TrackResolver};

pub(super) struct Music {
    pub(super) songbird: Arc<Songbird>,
    pub(super) manager: Arc<MusicManager>,
    pub(super) resolver: Arc<dyn TrackResolver>,
}

#[async_trait]
impl ModuleCommand for Music {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("music")
    }

    fn module(&self) -> Option<&'static str> {
        Some("music")
    }

    fn definition(&self) -> CreateCommand<'static> {
        MusicCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();

        let songbird = Arc::clone(&self.songbird);
        let music = Arc::clone(&self.manager);
        let resolver = Arc::clone(&self.resolver);

        let settings = Arc::clone(&cx.app.settings.music);

        let music_ctx = MusicCtx::new(
            &cx.ctx.http,
            cx.interaction,
            UserId::new(cx.app.zayden_id),
            MusicServices {
                http: Arc::clone(&cx.ctx.http),
                songbird,
                music,
                resolver,
                settings,
                entitlements: Arc::clone(&cx.app.entitlements),
                radio_stations: Arc::clone(&cx.app.radio_stations),
            },
        )?;

        MusicCommand::run(&music_ctx, options).await?;
        Ok(())
    }
}
