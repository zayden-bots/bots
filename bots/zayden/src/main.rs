#![recursion_limit = "256"]

use songbird::Songbird;
use zayden_app::applications::Slot;

mod guild_retention;
mod modules;
mod persona;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::boot("zayden", Slot::Zayden).await?;
    let songbird = Songbird::serenity();
    let assembly = modules::assemble(&boot.module_ctx(), &songbird).await?;
    boot.run(assembly, Some(songbird)).await
}
