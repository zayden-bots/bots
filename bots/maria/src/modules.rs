use ai::AiModule;
use bot_runtime::{Assembly, BotError, ModuleList};
use zayden_core::ModuleCtx;

use crate::persona;

pub(crate) fn assemble(cx: &ModuleCtx<'_>) -> Result<Assembly, BotError> {
    let mut list = ModuleList::default();
    list.try_add(AiModule::from_config(cx, persona::PERSONA, persona::WORLD));
    list.finish()
}
