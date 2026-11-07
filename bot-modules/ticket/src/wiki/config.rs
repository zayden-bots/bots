use url::Url;
use zayden_app::config::FaqSettingsRow;

use crate::wiki::WikiError;

const DEFAULT_LOCALE: &str = "en";
const DEFAULT_MAX_RESULTS: usize = 5;

#[derive(Debug, Clone)]
pub struct WikiConfig {
    site_root: Url,
    api_base: Url,
    article_base: Url,
    api_key: Option<String>,
    locale: String,
    max_results: usize,
}

impl WikiConfig {
    /// `None` when the guild has not enabled the FAQ or has not set a wiki URL.
    pub fn from_settings(row: &FaqSettingsRow) -> Result<Option<Self>, WikiError> {
        if !row.enabled {
            return Ok(None);
        }

        let Some(raw) =
            row.wiki_url.as_deref().map(str::trim).filter(|u| !u.is_empty())
        else {
            return Ok(None);
        };

        let base = Url::parse(&format!("{}/", raw.trim_end_matches('/')))
            .map_err(|e| WikiError::InvalidUrl(raw.to_owned(), e))?;

        if !matches!(base.scheme(), "http" | "https") {
            return Err(WikiError::UnsupportedScheme(base.scheme().to_owned()));
        }

        let locale = match row.wiki_locale.trim() {
            "" => DEFAULT_LOCALE,
            locale => locale,
        };

        let join = |path: &str| {
            base.join(path).map_err(|e| WikiError::InvalidUrl(raw.to_owned(), e))
        };

        Ok(Some(Self {
            api_base: join("_api/")?,
            site_root: base.clone(),
            article_base: join(&format!("{locale}/"))?,
            api_key: row
                .wiki_api_key
                .as_deref()
                .map(str::trim)
                .filter(|k| !k.is_empty())
                .map(str::to_owned),
            locale: locale.to_owned(),
            max_results: usize::try_from(row.max_results.clamp(1, 25))
                .unwrap_or(DEFAULT_MAX_RESULTS),
        }))
    }

    #[must_use]
    pub const fn api_base(&self) -> &Url {
        &self.api_base
    }

    pub(crate) fn host(&self) -> Result<&str, WikiError> {
        self.site_root
            .host_str()
            .ok_or_else(|| WikiError::MissingHost(self.site_root.to_string()))
    }

    #[must_use]
    pub(crate) fn cache_key(&self) -> String {
        format!(
            "{}\n{}",
            self.site_root,
            self.api_key.as_deref().unwrap_or_default()
        )
    }

    #[must_use]
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }

    #[must_use]
    pub const fn max_results(&self) -> usize {
        self.max_results
    }

    pub fn site_url(&self, path: &str) -> Result<Url, WikiError> {
        self.site_root
            .join(path.trim_start_matches('/'))
            .map_err(|e| WikiError::InvalidUrl(path.to_owned(), e))
    }

    pub fn article_url(&self, path: &str) -> Result<Url, WikiError> {
        self.article_base
            .join(path.trim_start_matches('/'))
            .map_err(|e| WikiError::InvalidUrl(path.to_owned(), e))
    }

    pub(crate) fn api_url(&self, path: &str) -> Result<Url, WikiError> {
        self.api_base
            .join(path.trim_start_matches('/'))
            .map_err(|e| WikiError::InvalidUrl(path.to_owned(), e))
    }
}
