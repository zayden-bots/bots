use zayden_app::applications::Slot;

mod modules;
mod persona;

#[tokio::main]
async fn main() -> bot_runtime::Result<()> {
    let boot = bot_runtime::boot("maria", Slot::Maria).await?;
    let assembly = modules::assemble(&boot.module_ctx())?;
    boot.run(assembly, None).await
}
