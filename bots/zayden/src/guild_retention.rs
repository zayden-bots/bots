use std::sync::Arc;

use patreon::{PatreonApp, PatreonConnection};
use tracing::{error, info};
use youtube::{YoutubeConnection, YoutubeRuntime};
use zayden_app::guilds::{GuildPresence, RETENTION_DAYS};
use zayden_app::state::AppState;
use zayden_core::CronJob;

pub(crate) struct GuildRetentionCron;

impl GuildRetentionCron {
    pub(crate) fn cron_job(
        app: Arc<AppState>,
        patreon: Option<Arc<PatreonApp>>,
        youtube: Option<YoutubeRuntime>,
    ) -> Result<CronJob, jiff_cron::error::Error> {
        CronJob::new("guild_retention_purge", "0 30 4 * * * *").map(|job| {
            job.set_action(move |_ctx, _pool| {
                let app = Arc::clone(&app);
                let patreon = patreon.clone();
                let youtube = youtube.clone();
                async move { purge_expired(&app, patreon, youtube).await }
            })
        })
    }
}

async fn purge_expired(
    app: &AppState,
    patreon: Option<Arc<PatreonApp>>,
    youtube: Option<YoutubeRuntime>,
) {
    let guilds = match GuildPresence::expired(&app.db, RETENTION_DAYS).await {
        Ok(guilds) => guilds,
        Err(e) => {
            error!(error = ?e, "guild retention: failed to list expired guilds");
            return;
        },
    };

    for guild_id in guilds {
        purge(app, patreon.as_deref(), youtube.as_ref(), guild_id).await;
    }
}

async fn purge(
    app: &AppState,
    patreon: Option<&PatreonApp>,
    youtube: Option<&YoutubeRuntime>,
    guild_id: i64,
) {
    let channel = YoutubeConnection::select(&app.db, guild_id).await.ok().flatten();
    let connection =
        PatreonConnection::select(&app.db, guild_id).await.ok().flatten();

    match GuildPresence::purge(&app.db, guild_id, RETENTION_DAYS).await {
        Ok(true) => {},
        Ok(false) => return,
        Err(e) => {
            error!(error = ?e, guild_id, "guild retention: failed to purge guild");
            return;
        },
    }

    if let Some(channel) = channel {
        youtube::release_channel(
            &app.http,
            &app.db,
            youtube.map(|r| &*r.webhook_uri),
            &channel.channel_id,
        )
        .await;
    }

    if let Some(connection) = connection {
        patreon::release(&app.http, &app.db, patreon, &connection).await;
    }

    info!(guild_id, "guild retention: purged guild data");
}
