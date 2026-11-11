use llamad2::Llamad2Module;
use serenity::all::GuildId;
use songbird::Songbird;
use zayden_app::applications::Slot;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::boot("llama", Slot::Zayden).await?;
    let songbird = Songbird::serenity();
    let cx = boot.module_ctx();
    let mut list = zayden::module_list(&cx, &songbird).await;
    list.add(Llamad2Module::new(GuildId::new(cx.config.llamad2_guild)));
    let assembly = list.finish()?;
    boot.run(assembly, Some(songbird)).await
}
