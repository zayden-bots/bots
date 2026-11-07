use std::borrow::Cow;

use async_trait::async_trait;
use serenity::all::{
    CommandOptionType,
    CreateCommand,
    CreateCommandOption,
    Permissions,
    User,
};
use zayden_core::{
    ComponentCtx,
    CoreError,
    HandlerError,
    IdMatch,
    InvocationCtx,
    Module,
    ModuleCommand,
    ModuleComponent,
    OverlapError,
    RegistryBuilder,
    SubCommandOptions,
    sole_option,
};

pub struct VerifyModule;

impl VerifyModule {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for VerifyModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for VerifyModule {
    fn id(&self) -> &'static str {
        "verify"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(PanelCmd).add_command(ManVerify).add_component(PanelCmd)?;
        Ok(())
    }
}

struct PanelCmd;

#[async_trait]
impl ModuleCommand for PanelCmd {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("panel")
    }

    fn definition(&self) -> CreateCommand<'static> {
        crate::Panel::register()
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        crate::Panel::run_command(&cx.ctx.http, cx.interaction).await?;
        Ok(())
    }
}

#[async_trait]
impl ModuleComponent for PanelCmd {
    fn id_match(&self) -> IdMatch {
        IdMatch::Exact(Cow::Borrowed("verify"))
    }

    async fn run(&self, cx: &ComponentCtx<'_>) -> Result<(), HandlerError> {
        crate::Panel::run_component(
            &cx.ctx.http,
            cx.interaction,
            &cx.app.settings.roles,
        )
        .await?;
        Ok(())
    }
}

struct ManVerify;

#[async_trait]
impl ModuleCommand for ManVerify {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("manverify")
    }

    fn definition(&self) -> CreateCommand<'static> {
        CreateCommand::new("manverify")
            .default_member_permissions(Permissions::ADMINISTRATOR)
            .description("Manually verifies a user")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::User,
                    "user",
                    "User to verify",
                )
                .required(true),
            )
    }

    async fn run(&self, cx: &InvocationCtx<'_>) -> Result<(), HandlerError> {
        let user: &User = {
            let mut options = cx.interaction.data.options();
            let options: SubCommandOptions<'_> = sole_option(&mut options)?;
            sole_option(&mut options.into_vec())?
        };

        let guild_id = cx.interaction.guild_id.ok_or(CoreError::MissingGuildId)?;

        let role = crate::verified_role(&cx.app.settings.roles, guild_id).await?;

        cx.ctx
            .http
            .add_member_role(
                guild_id,
                user.id,
                role,
                Some(&format!(
                    "User manually verified by {}.",
                    cx.interaction.user.display_name()
                )),
            )
            .await?;

        Ok(())
    }
}
