use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::error::HandlerError;
use zayden_core::{InvocationCtx, ModuleCommand};

use crate::{VoiceCommand, VoiceStateCache};

pub(super) struct Voice {
    pub(super) voice_states: Arc<VoiceStateCache>,
}

#[async_trait]
impl ModuleCommand for Voice {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("voice")
    }

    fn definition(&self) -> CreateCommand<'static> {
        VoiceCommand::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        VoiceCommand::run(cx.ctx, cx.interaction, &cx.app.db, &self.voice_states)
            .await?;
        Ok(())
    }
}
