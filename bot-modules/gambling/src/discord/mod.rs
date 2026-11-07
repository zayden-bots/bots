use std::marker::PhantomData;

use async_trait::async_trait;
use zayden_core::{
    EmojiCacheData,
    GuildMembersCache,
    Module,
    OverlapError,
    RegistryBuilder,
};

use crate::GamblingData;

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

pub struct GamblingModule<D>(PhantomData<fn() -> D>);

impl<D> GamblingModule<D> {
    #[must_use]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<D> Default for GamblingModule<D> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<D> Module for GamblingModule<D>
where
    D: EmojiCacheData + GamblingData + GuildMembersCache + Send + Sync + 'static,
{
    fn id(&self) -> &'static str {
        "gambling"
    }

    fn register(&self, b: &mut RegistryBuilder) -> Result<(), OverlapError> {
        b.add_command(Blackjack::<D>(PhantomData))
            .add_component(Blackjack::<D>(PhantomData))?
            .add_command(Coinflip::<D>(PhantomData))
            .add_command(Craft::<D>(PhantomData))
            .add_command(Daily::<D>(PhantomData))
            .add_command(Dig::<D>(PhantomData))
            .add_command(Gift::<D>(PhantomData))
            .add_command(Goals::<D>(PhantomData))
            .add_command(HigherLower::<D>(PhantomData))
            .add_component(HigherLower::<D>(PhantomData))?
            .add_command(Inventory::<D>(PhantomData))
            .add_command(Leaderboard::<D>(PhantomData))
            .add_component(Leaderboard::<D>(PhantomData))?
            .add_command(Lotto::<D>(PhantomData))
            .add_command(Mine::<D>(PhantomData))
            .add_command(Prestige)
            .add_component(Prestige)?
            .add_command(Profile::<D>(PhantomData))
            .add_command(Roll::<D>(PhantomData))
            .add_command(RockPaperScissors::<D>(PhantomData))
            .add_command(SendCmd::<D>(PhantomData))
            .add_command(Shop::<D>(PhantomData))
            .add_component(Shop::<D>(PhantomData))?
            .add_command(TicTacToe::<D>(PhantomData))
            .add_component(TicTacToe::<D>(PhantomData))?
            .add_command(Work::<D>(PhantomData));

        Ok(())
    }
}
