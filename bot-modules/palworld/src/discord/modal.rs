use std::borrow::Cow;
use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::ctx::ModalCtx;
use zayden_core::error::HandlerError;
use zayden_core::module::ModuleModal;
use zayden_core::scope::IdMatch;

use crate::client::PalworldClient;
use crate::commands::Command as PalworldCommand;

pub(super) struct PalworldUploadModal {
    pub(super) client: Arc<PalworldClient>,
}

#[async_trait]
impl ModuleModal for PalworldUploadModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("palworld_save_upload"))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        PalworldCommand::upload_submit(cx, &self.client, &cx.app.db).await?;
        Ok(())
    }
}
