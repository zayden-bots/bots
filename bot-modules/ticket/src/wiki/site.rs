use std::sync::LazyLock;

use dashmap::DashMap;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tracing::warn;

use crate::wiki::client::api_error;
use crate::wiki::{WikiConfig, WikiError};

#[derive(Debug, Clone)]
pub(crate) struct Access {
    pub(crate) id: String,
    pub(crate) use_key: bool,
}

#[derive(Deserialize)]
struct Site {
    id: String,
}

static SITES: LazyLock<DashMap<String, Access>> = LazyLock::new(DashMap::new);

pub(crate) async fn access(
    client: &Client,
    config: &WikiConfig,
) -> Result<Access, WikiError> {
    let key = config.cache_key();

    if let Some(cached) = SITES.get(&key) {
        return Ok(cached.clone());
    }

    let url = config.api_url(&format!("sites/{}", config.host()?))?;

    let mut use_key = false;
    let mut request = client.get(url.clone());

    if let Some(api_key) = config.api_key() {
        request = request.bearer_auth(api_key);
        use_key = true;
    }

    let mut response = request.send().await?;

    if use_key
        && matches!(
            response.status(),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN
        )
    {
        warn!(
            status = response.status().as_u16(),
            "wiki rejected the configured api key; reading it anonymously"
        );
        use_key = false;
        response = client.get(url).send().await?;
    }

    if response.status() == StatusCode::NOT_FOUND {
        return Err(WikiError::Api {
            status: 404,
            message: "no wiki site for this host".into(),
        });
    }

    if !response.status().is_success() {
        return Err(api_error(response).await);
    }

    let site: Site = response.json().await?;
    let access = Access { id: site.id, use_key };

    SITES.insert(key, access.clone());

    Ok(access)
}
