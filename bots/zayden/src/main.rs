use songbird::Songbird;
use zayden_app::applications::Slot;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::boot("zayden", Slot::Zayden).await?;
    let songbird = Songbird::serenity();
    let assembly =
        zayden::module_list(&boot.module_ctx(), &songbird).await.finish()?;
    boot.run(assembly, Some(songbird)).await
}
