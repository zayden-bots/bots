mod dungeon_report;
mod goof;
mod hello;
mod playlist;
mod raidreport;
mod sensitivity;
mod socials;

use async_trait::async_trait;
use dungeon_report::DungeonReport;
use goof::Goof;
use hello::Hello;
use playlist::Playlist;
use raidreport::RaidReport;
use sensitivity::Sensitivity;
use serenity::all::{FullEvent, GuildId};
use socials::Socials;
use zayden_core::error::HandlerError;
use zayden_core::{EventCtx, EventKind, Module, OverlapError, RegistryBuilder};

use crate::GoodMorningCache;

pub struct Llamad2Module {
    guild: GuildId,
    good_morning: GoodMorningCache,
}

impl Llamad2Module {
    #[must_use]
    pub fn new(guild: GuildId) -> Self {
        Self { guild, good_morning: GoodMorningCache::default() }
    }
}

#[async_trait]
impl Module for Llamad2Module {
    fn id(&self) -> &'static str {
        "llamad2"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(DungeonReport { guild: self.guild })
            .add_command(Goof { guild: self.guild })
            .add_command(Hello { guild: self.guild })
            .add_command(Playlist { guild: self.guild })
            .add_command(RaidReport { guild: self.guild })
            .add_command(Sensitivity { guild: self.guild })
            .add_command(Socials { guild: self.guild });
        Ok(())
    }

    fn events(&self) -> &'static [EventKind] {
        &[EventKind::Message, EventKind::PresenceUpdate]
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
            FullEvent::Message { new_message, .. } => {
                if new_message.author.bot() {
                    return Ok(());
                }
                let (good_morning, counting) = tokio::join!(
                    crate::GoodMorning::run(cx.ctx, new_message, &self.good_morning),
                    crate::CountingFail::run(cx.ctx, new_message, &cx.app.db),
                );
                good_morning?;
                counting?;
            },
            FullEvent::PresenceUpdate { new_data, .. } => {
                crate::StatusUpdate::presence_update(cx.ctx, new_data);
            },
            _ => {},
        }
        Ok(())
    }
}
