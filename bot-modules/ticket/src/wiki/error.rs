#[derive(Debug)]
pub enum WikiError {
    InvalidUrl(String, url::ParseError),
    UnsupportedScheme(String),
    MissingHost(String),
    PageNotFound(String),
    NoBody(String),
    Api { status: u16, message: String },
    Html(std::io::Error),
    Http(reqwest::Error),
}

impl std::fmt::Display for WikiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUrl(url, e) => write!(f, "invalid wiki url {url}: {e}"),
            Self::UnsupportedScheme(scheme) => {
                write!(f, "unsupported wiki url scheme: {scheme}")
            },
            Self::MissingHost(url) => write!(f, "wiki url {url} has no host"),
            Self::PageNotFound(path) => write!(f, "no wiki page at {path}"),
            Self::NoBody(path) => write!(f, "wiki page {path} returned no content"),
            Self::Api { status, message } => {
                write!(f, "wiki api error (HTTP {status}): {message}")
            },
            Self::Html(e) => write!(f, "could not convert wiki html: {e}"),
            Self::Http(e) => write!(f, "wiki request failed: {e}"),
        }
    }
}

impl std::error::Error for WikiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUrl(_, e) => Some(e),
            Self::Html(e) => Some(e),
            Self::Http(e) => Some(e),
            Self::UnsupportedScheme(_)
            | Self::MissingHost(_)
            | Self::PageNotFound(_)
            | Self::NoBody(_)
            | Self::Api { .. } => None,
        }
    }
}

impl From<reqwest::Error> for WikiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}
