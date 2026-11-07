use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::CreateCommand;
use zayden_core::{
    ComponentCtx,
    HandlerError,
    IdMatch,
    InvocationCtx,
    ModuleCommand,
    ModuleComponent,
};

use crate::Commands;
use crate::components::PrestigeCustomId;

pub(super) struct Prestige;

#[async_trait]
impl ModuleCommand for Prestige {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("prestige")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_prestige()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        Commands::prestige(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for Prestige {
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("prestige"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        match cx.interaction.data.custom_id.parse::<PrestigeCustomId>()? {
            PrestigeCustomId::Confirm => {
                Commands::confirm_prestige(
                    cx.ctx,
                    cx.interaction,
                    &cx.app.db,
                    cx.app.zayden_id,
                )
                .await?;
            },
            PrestigeCustomId::Cancel => {
                Commands::cancel_prestige(cx.ctx, cx.interaction).await?;
            },
        }

        Ok(())
    }
}
