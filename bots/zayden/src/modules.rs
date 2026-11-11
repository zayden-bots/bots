use std::sync::Arc;

use accounts::AccountsModule;
use ai::AiModule;
use async_trait::async_trait;
use bot_runtime::ModuleList;
use deadlock::DeadlockModule;
use destiny2::{Destiny2Module, ParentToken};
use family::FamilyModule;
use gambling::{CoinReward, GamblingModule};
use gold_star::GoldStarModule;
use greetings::GreetingsModule;
use honeypot::HoneypotModule;
use hosting::HostingModule;
use jellyfin::JellyfinModule;
use levels::LevelsModule;
use lfg::LfgModule;
use marathon::MarathonModule;
use misc::MiscModule;
use moderation::ModerationModule;
use music::MusicModule;
use palworld::PalworldModule;
use patreon::PatreonModule;
use reaction_roles::ReactionRolesModule;
use songbird::Songbird;
use sqlx::PgPool;
use suggestions::SuggestionsModule;
use temp_voice::TempVoiceModule;
use ticket::TicketModule;
use verify::VerifyModule;
use watch::WatchModule;
use youtube::YoutubeModule;
use zayden_app::state::AppState;
use zayden_core::error::HandlerError;
use zayden_core::{Identity, ModuleCtx};

use crate::persona;

struct ZaydenParentToken {
    app: Arc<AppState>,
    me: Identity,
}

#[async_trait]
impl ParentToken for ZaydenParentToken {
    async fn get(&self, _db: &PgPool) -> Result<String, HandlerError> {
        bot_runtime::parent_token(&self.app, self.me)
            .await
            .map_err(HandlerError::new)
    }
}

pub async fn module_list(
    cx: &ModuleCtx<'_>,
    songbird: &Arc<Songbird>,
) -> ModuleList {
    let parent_token: Arc<dyn ParentToken> =
        Arc::new(ZaydenParentToken { app: Arc::clone(cx.app), me: cx.me });
    let mut list = ModuleList::default();
    list.add(GreetingsModule::new())
        .add(GoldStarModule::new())
        .add(ModerationModule::new())
        .add(HoneypotModule::new())
        .add(MiscModule::new())
        .add(ReactionRolesModule::new())
        .add(SuggestionsModule::new())
        .add(VerifyModule::new())
        .add(FamilyModule::new())
        .add(AccountsModule::new(Arc::from(cx.config.dashboard_url.as_str())))
        .add(LevelsModule::new(Some(Arc::new(CoinReward))))
        .add(GamblingModule::new())
        .add(LfgModule::new());
    list.try_add(DeadlockModule::from_config(cx));
    list.add(MarathonModule::from_config(cx)).add(PalworldModule::from_config(cx));
    list.try_add(Destiny2Module::from_config(cx, parent_token));
    list.add(TempVoiceModule::new());
    list.try_add(MusicModule::from_config(cx, Arc::clone(songbird)).await);
    let jellyfin = list.take(jellyfin::runtime_from_config(&cx.worker())).flatten();
    list.try_add(TicketModule::from_config(cx));
    list.try_add(AiModule::from_config(cx, persona::PERSONA, persona::WORLD));
    list.add(HostingModule::from_config(cx))
        .add(JellyfinModule::new(jellyfin.as_ref().map(Arc::clone)))
        .add(WatchModule::new(jellyfin))
        .add(PatreonModule)
        .add(YoutubeModule);
    list
}
