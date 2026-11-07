use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::{
    CreateInputText,
    CreateLabel,
    CreateModalComponent,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption,
    InputTextStyle,
};
use zayden_core::ctx::{ComponentCtx, ModalCtx};
use zayden_core::error::HandlerError;
use zayden_core::module::{ModuleComponent, ModuleModal};
use zayden_core::scope::IdMatch;

use crate::{TicketComponent, TicketModal};

fn make_ticket_modal_components<'a>() -> Vec<CreateModalComponent<'a>> {
    vec![
        CreateModalComponent::Label(CreateLabel::select_menu(
            "Select a relevent category:",
            CreateSelectMenu::new("ticket_category", CreateSelectMenuKind::String {
                options: vec![
                    CreateSelectMenuOption::new("Complaint", "complaint"),
                    CreateSelectMenuOption::new(
                        "General question",
                        "general_question",
                    ),
                    CreateSelectMenuOption::new(
                        "Role Application",
                        "role_application",
                    ),
                    CreateSelectMenuOption::new("Suggestion", "suggestion"),
                ]
                .into(),
            }),
        )),
        CreateModalComponent::Label(CreateLabel::input_text(
            "Describe the issue:",
            CreateInputText::new(InputTextStyle::Paragraph, "ticket_body"),
        )),
    ]
}

pub(super) struct TicketCreate;

#[async_trait]
impl ModuleComponent for TicketCreate {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("ticket_create"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::ticket_create(
            &cx.ctx.http,
            cx.interaction,
            make_ticket_modal_components(),
        )
        .await?;
        Ok(())
    }
}

pub(super) struct SupportTicket;

#[async_trait]
impl ModuleComponent for SupportTicket {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("support_ticket"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::ticket_create(
            &cx.ctx.http,
            cx.interaction,
            make_ticket_modal_components(),
        )
        .await?;
        Ok(())
    }
}

pub(super) struct SupportClose;

#[async_trait]
impl ModuleComponent for SupportClose {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("support_close"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::support_close(&cx.ctx.http, cx.interaction, &cx.app)
            .await?;
        Ok(())
    }
}

pub(super) struct SupportSolved;

#[async_trait]
impl ModuleComponent for SupportSolved {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("support_solved"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::support_solved(&cx.ctx.http, cx.interaction, &cx.app)
            .await?;
        Ok(())
    }
}

pub(super) struct SupportStillOpen;

#[async_trait]
impl ModuleComponent for SupportStillOpen {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("support_still_open"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::support_still_open(&cx.ctx.http, cx.interaction, &cx.app)
            .await?;
        Ok(())
    }
}

pub(super) struct SupportFaq;

#[async_trait]
impl ModuleComponent for SupportFaq {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("support_faq"))
    }

    fn module(&self) -> Option<&'static str> {
        Some("ticket")
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        TicketComponent::support_faq(&cx.ctx.http, cx.interaction, &cx.app).await?;
        Ok(())
    }
}

pub(super) struct CreateTicketModal;

#[async_trait]
impl ModuleModal for CreateTicketModal {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("create_ticket"))
    }

    async fn run(&self, cx: &ModalCtx<'_>) -> Result<(), HandlerError> {
        TicketModal::run(&cx.ctx.http, cx.interaction, &cx.app).await?;
        Ok(())
    }
}
