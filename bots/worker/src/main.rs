mod guild_retention;
mod workers;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::worker::boot().await?;
    let assembly = workers::assemble(&boot.module_ctx())?;
    boot.run(assembly).await
}
