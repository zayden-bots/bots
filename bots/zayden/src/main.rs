#![recursion_limit = "256"]

use songbird::Songbird;

mod guild_retention;
mod modules;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::boot("zayden").await?;
    let songbird = Songbird::serenity();
    let assembly = modules::assemble(&boot.module_ctx(), &songbird).await?;
    boot.run(assembly, Some(songbird)).await
}
