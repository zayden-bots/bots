use std::sync::Arc;

use async_trait::async_trait;
use zayden_core::{CronResult, Module, OverlapError, RegistryBuilder};

use crate::{GameCache, StaminaCron};

mod blackjack;
mod coinflip;
mod craft;
mod daily;
mod dig;
mod gift;
mod goals;
mod higher_lower;
mod inventory;
mod leaderboard;
mod lotto;
mod mine;
mod prestige;
mod profile;
mod roll;
mod rps;
mod send;
mod shop;
mod tictactoe;
mod work;

use blackjack::Blackjack;
use coinflip::Coinflip;
use craft::Craft;
use daily::Daily;
use dig::Dig;
use gift::Gift;
use goals::Goals;
use higher_lower::HigherLower;
use inventory::Inventory;
use leaderboard::Leaderboard;
use lotto::Lotto;
use mine::Mine;
use prestige::Prestige;
use profile::Profile;
use roll::Roll;
use rps::RockPaperScissors;
use send::SendCmd;
use shop::Shop;
use tictactoe::TicTacToe;
use work::Work;

pub struct GamblingModule {
    cache: Arc<GameCache>,
}

impl GamblingModule {
    #[must_use]
    pub fn new() -> Self {
        Self { cache: Arc::new(GameCache::default()) }
    }
}

impl Default for GamblingModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for GamblingModule {
    fn id(&self) -> &'static str {
        "gambling"
    }

    fn cron_jobs(&self) -> Vec<CronResult> {
        vec![
            StaminaCron::cron_job(),
            crate::Lotto::cron_job(),
            crate::HigherLower::cron_job(),
        ]
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        let cache = &self.cache;
        b.add_command(Blackjack(Arc::clone(cache)))
            .add_component(Blackjack(Arc::clone(cache)))?
            .add_command(Coinflip(Arc::clone(cache)))
            .add_command(Craft)
            .add_command(Daily)
            .add_command(Dig)
            .add_command(Gift)
            .add_command(Goals)
            .add_command(HigherLower(Arc::clone(cache)))
            .add_component(HigherLower(Arc::clone(cache)))?
            .add_command(Inventory)
            .add_command(Leaderboard)
            .add_component(Leaderboard)?
            .add_command(Lotto)
            .add_command(Mine)
            .add_command(Prestige)
            .add_component(Prestige)?
            .add_command(Profile)
            .add_command(Roll(Arc::clone(cache)))
            .add_command(RockPaperScissors(Arc::clone(cache)))
            .add_command(SendCmd)
            .add_command(Shop)
            .add_component(Shop)?
            .add_command(TicTacToe(Arc::clone(cache)))
            .add_component(TicTacToe(Arc::clone(cache)))?
            .add_command(Work);

        Ok(())
    }
}
