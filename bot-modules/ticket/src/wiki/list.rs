use reqwest::Client;

use crate::wiki::search::{PAGE_LIMIT, SearchResponse};
use crate::wiki::{WikiConfig, WikiError, client};

const MAX_LISTED: usize = 5000;

#[derive(Debug, Clone)]
pub struct PageListItem {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub async fn list(
    client: &Client,
    config: &WikiConfig,
) -> Result<Vec<PageListItem>, WikiError> {
    let limit = PAGE_LIMIT.to_string();
    let mut items = Vec::new();
    let mut offset = 0;

    loop {
        let Some(response) =
            client::get::<SearchResponse>(client, config, "pages/search", &[
                ("locales", config.locale()),
                ("publishState", "published"),
                ("limit", &limit),
                ("offset", &offset.to_string()),
            ])
            .await?
        else {
            break;
        };

        let batch = response.results.len();

        items.extend(response.results.into_iter().map(|result| {
            PageListItem {
                id: result.id,
                path: result.path,
                title: Some(result.title).filter(|title| !title.trim().is_empty()),
                description: Some(result.description)
                    .filter(|description| !description.trim().is_empty()),
            }
        }));

        offset += batch;

        if batch < PAGE_LIMIT
            || response.total_hits.is_some_and(|total| offset >= total)
            || offset >= MAX_LISTED
        {
            break;
        }
    }

    Ok(items)
}
