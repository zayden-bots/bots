use reqwest::Client;
use serde::{Deserialize, Deserializer};

use crate::wiki::{WikiConfig, WikiError, client};

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    #[serde(default, deserialize_with = "null_as_empty")]
    pub description: String,
    pub path: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResponse {
    #[serde(default)]
    pub(crate) results: Vec<SearchResult>,
    pub(crate) total_hits: Option<usize>,
}

pub(crate) const PAGE_LIMIT: usize = 100;
const SEARCH_LIMIT: &str = "50";

fn null_as_empty<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
}

pub async fn search(
    client: &Client,
    config: &WikiConfig,
    query: &str,
) -> Result<Vec<SearchResult>, WikiError> {
    let response = client::get::<SearchResponse>(client, config, "pages/search", &[
        ("query", query),
        ("locales", config.locale()),
        ("publishState", "published"),
        ("limit", SEARCH_LIMIT),
    ])
    .await?;

    Ok(response.map_or_default(|response| response.results))
}
