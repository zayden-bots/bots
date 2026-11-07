use async_trait::async_trait;
use serenity::all::UserId;
use sqlx::PgPool;
use zayden_core::HandlerError;

use crate::{GamblingManager, level_up_reward};

pub struct CoinReward;

#[async_trait]
impl levels::LevelUpReward for CoinReward {
    async fn on_level_up(
        &self,
        pool: &PgPool,
        user: UserId,
        level: i32,
    ) -> Result<(), HandlerError> {
        let reward = level_up_reward(i64::from(level));

        if reward > 0 {
            let mut tx = pool.begin().await?;

            GamblingManager::add_coins(&mut tx, user, reward).await?;

            tx.commit().await?;
        }

        Ok(())
    }
}
