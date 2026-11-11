use std::sync::Arc;

use bot_runtime::BotError;
use bot_runtime::worker::{WorkerAssembly, WorkerList};
use deadlock::DeadlockWorker;
use destiny2::Destiny2Worker;
use gambling::GamblingWorker;
use hosting::HostingWorker;
use jellyfin::JellyfinWorker;
use marathon::MarathonWorker;
use patreon::PatreonWorker;
use ticket::TicketWorker;
use watch::WatchWorker;
use youtube::YoutubeWorker;
use zayden_core::WorkerModuleCtx;

use crate::guild_retention::GuildRetentionCron;

pub(crate) fn assemble(
    cx: &WorkerModuleCtx<'_>,
) -> Result<WorkerAssembly, BotError> {
    let patreon_app = patreon::app_from_config(cx.config);
    let youtube = youtube::runtime_from_config(cx.config);
    let mut list = WorkerList::default();
    let jellyfin = list.take(jellyfin::runtime_from_config(cx)).flatten();
    list.add(YoutubeWorker::new(cx.app.http.clone(), youtube.clone()))
        .add(PatreonWorker::new(cx.app.http.clone(), patreon_app.clone()))
        .add(MarathonWorker::from_config(cx));
    list.try_add(DeadlockWorker::from_config(cx));
    list.try_add(Destiny2Worker::from_config(cx));
    list.add(GamblingWorker)
        .add(TicketWorker)
        .add(JellyfinWorker::new(jellyfin.as_ref().map(Arc::clone)))
        .add(WatchWorker::new(jellyfin))
        .add(HostingWorker::from_config(cx))
        .job(GuildRetentionCron::worker_job(
            Arc::clone(cx.app),
            patreon_app,
            youtube,
        ));
    list.finish()
}
