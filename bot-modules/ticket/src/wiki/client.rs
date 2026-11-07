use reqwest::{Client, Response, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::wiki::{WikiConfig, WikiError, site};

#[derive(Deserialize)]
struct ErrorBody {
    message: Option<String>,
}

pub(crate) async fn get<T: DeserializeOwned>(
    client: &Client,
    config: &WikiConfig,
    route: &str,
    query: &[(&str, &str)],
) -> Result<Option<T>, WikiError> {
    let access = site::access(client, config).await?;

    let mut request = client
        .get(config.api_url(&format!("sites/{}/{route}", access.id))?)
        .query(query);

    if let (true, Some(key)) = (access.use_key, config.api_key()) {
        request = request.bearer_auth(key);
    }

    let response = request.send().await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    if !response.status().is_success() {
        return Err(api_error(response).await);
    }

    Ok(Some(response.json().await?))
}

pub(crate) async fn api_error(response: Response) -> WikiError {
    let status = response.status().as_u16();
    let message = response
        .json::<ErrorBody>()
        .await
        .ok()
        .and_then(|body| body.message)
        .unwrap_or_default();

    WikiError::Api { status, message }
}
