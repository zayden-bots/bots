mod slash_command;

use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::FullEvent;
use slash_command::Lfg;
use zayden_core::ctx::{ComponentCtx, ModalCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleComponent, ModuleModal};
use zayden_core::scope::IdMatch;
use zayden_core::{EventCtx, EventKind, Module, OverlapError, RegistryBuilder};

use crate::{Components, KickComponent, TagsComponent};

// region: Components

struct LfgJoin;

#[async_trait]
impl ModuleComponent for LfgJoin {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_join"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::join(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgLeave;

#[async_trait]
impl ModuleComponent for LfgLeave {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_leave"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::leave(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgAlternative;

#[async_trait]
impl ModuleComponent for LfgAlternative {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_alternative"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::alternative(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgSettings;

#[async_trait]
impl ModuleComponent for LfgSettings {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_settings"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::settings(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgEditComponent;

#[async_trait]
impl ModuleComponent for LfgEditComponent {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_edit"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::edit(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgCopy;

#[async_trait]
impl ModuleComponent for LfgCopy {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_copy"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::copy(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgKick;

#[async_trait]
impl ModuleComponent for LfgKick {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_kick"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::kick(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgKickMenu;

#[async_trait]
impl ModuleComponent for LfgKickMenu {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_kick_menu"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        KickComponent::run(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgDelete;

#[async_trait]
impl ModuleComponent for LfgDelete {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_delete"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        Components::delete(&cx.ctx.http, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgTagsAdd;

#[async_trait]
impl ModuleComponent for LfgTagsAdd {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_tags_add"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TagsComponent::add(&cx.ctx.http, cx.interaction).await?;
        Ok(())
    }
}

struct LfgTagsRemove;

#[async_trait]
impl ModuleComponent for LfgTagsRemove {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_tags_remove"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TagsComponent::remove(&cx.ctx.http, cx.interaction).await?;
        Ok(())
    }
}

// endregion

// region: Modals

struct LfgEditModal;

#[async_trait]
impl ModuleModal for LfgEditModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("lfg_edit"))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        crate::modals::Edit::run(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

struct LfgCreateModal;

#[async_trait]
impl ModuleModal for LfgCreateModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Prefix(Cow::Borrowed("lfg_create"))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        crate::modals::Create::run(cx.ctx, cx.interaction, &cx.app.db).await?;
        Ok(())
    }
}

// endregion

pub struct LfgModule;

impl LfgModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for LfgModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for LfgModule {
    fn id(&self) -> &'static str {
        "lfg"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Lfg)
            .add_autocomplete(Lfg)
            .add_component(LfgJoin)?
            .add_component(LfgLeave)?
            .add_component(LfgAlternative)?
            .add_component(LfgSettings)?
            .add_component(LfgEditComponent)?
            .add_component(LfgCopy)?
            .add_component(LfgKick)?
            .add_component(LfgKickMenu)?
            .add_component(LfgDelete)?
            .add_component(LfgTagsAdd)?
            .add_component(LfgTagsRemove)?
            .add_modal(LfgEditModal)?
            .add_modal(LfgCreateModal)?;
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::GuildCreate, EventKind::ThreadDelete]
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "FullEvent is non_exhaustive; unrouted kinds are ignored"
    )]
    async fn on_event(
        &self,
        cx: &EventCtx<'_>,
        ev: &FullEvent,
    ) -> Result<(), HandlerError> {
        match ev {
            FullEvent::GuildCreate { guild, .. } => {
                crate::events::guild_create(cx.ctx, guild, &cx.app.db).await?;
            },
            FullEvent::ThreadDelete { thread, .. } => {
                crate::events::thread_delete(&cx.ctx.http, thread, &cx.app.db)
                    .await?;
            },
            _ => {},
        }
        Ok(())
    }
}
