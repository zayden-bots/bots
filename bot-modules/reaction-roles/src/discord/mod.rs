use async_trait::async_trait;
use serenity::all::FullEvent;
use zayden_core::{
    EventCtx,
    EventKind,
    HandlerError,
    Module,
    OverlapError,
    RegistryBuilder,
};

use crate::ReactionRoleReaction;

mod slash_command;

use slash_command::ReactionRoleCmd;

pub struct ReactionRolesModule;

impl ReactionRolesModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ReactionRolesModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for ReactionRolesModule {
    fn id(&self) -> &'static str {
        "reaction_roles"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(ReactionRoleCmd);
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::ReactionAdd, EventKind::ReactionRemove]
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
            FullEvent::ReactionAdd { add_reaction, .. } => {
                ReactionRoleReaction::reaction_add(
                    &cx.ctx.http,
                    add_reaction,
                    &cx.app.db,
                )
                .await?;
            },
            FullEvent::ReactionRemove { removed_reaction, .. } => {
                ReactionRoleReaction::reaction_remove(
                    &cx.ctx.http,
                    removed_reaction,
                    &cx.app.db,
                )
                .await?;
            },
            _ => {},
        }
        Ok(())
    }
}
