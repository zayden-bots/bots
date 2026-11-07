use reqwest::Client;
use serde::Deserialize;

use crate::wiki::{WikiConfig, WikiError, client, hash, html};

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub path: String,
    pub content: String,
}

#[derive(Deserialize)]
struct PageBody {
    path: String,
    title: String,
    content: Option<String>,
    render: Option<String>,
}

pub async fn page(
    client: &Client,
    config: &WikiConfig,
    path: &str,
) -> Result<Page, WikiError> {
    let normalized = hash::normalize_path(path);

    let body = client::get::<PageBody>(
        client,
        config,
        &format!("pages/{}", hash::path_hash(&normalized)),
        &[
            ("path", &normalized),
            ("locale", config.locale()),
            ("withContent", "true"),
        ],
    )
    .await?
    .ok_or_else(|| WikiError::PageNotFound(path.to_owned()))?;

    into_page(body, path)
}

pub async fn page_by_id(
    client: &Client,
    config: &WikiConfig,
    id: &str,
) -> Result<Page, WikiError> {
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        return Err(WikiError::PageNotFound(id.to_owned()));
    }

    let body = client::get::<PageBody>(client, config, &format!("pages/{id}"), &[(
        "withContent",
        "true",
    )])
    .await?
    .ok_or_else(|| WikiError::PageNotFound(id.to_owned()))?;

    let path = body.path.clone();

    into_page(body, &path)
}

fn into_page(body: PageBody, path: &str) -> Result<Page, WikiError> {
    let content = match (body.content, body.render) {
        (Some(content), _) if !content.trim().is_empty() => content,
        (_, Some(render)) if !render.trim().is_empty() => {
            html::html_to_markdown(&render)?
        },
        _ => return Err(WikiError::NoBody(path.to_owned())),
    };

    Ok(Page { title: body.title, path: body.path, content })
}
