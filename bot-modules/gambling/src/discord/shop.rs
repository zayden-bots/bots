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

pub(super) struct Shop;

#[async_trait]
impl ModuleCommand for Shop {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("shop")
    }

    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn definition(&self) -> CreateCommand<'static> {
        Commands::register_shop()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let options = cx.interaction.data.options();
        Commands::shop(cx.ctx, cx.interaction, options, &cx.app.db).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for Shop {
    fn module(&self) -> Option<&'static str> {
        Some("gambling")
    }

    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("shop_"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::components::Shop::run_components(cx.ctx, cx.interaction, &cx.app.db)
            .await?;
        Ok(())
    }
}
