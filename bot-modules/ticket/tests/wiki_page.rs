//! Reading pages over the Wiki.js 3 REST API, including the anonymous fallback
//! when the wiki rejects the configured key.

use reqwest::Client;
use ticket::wiki::{self, WikiConfig, WikiError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use zayden_app::config::{FaqSettingsRow, SettingsRow};

const SITE: &str = "11111111-1111-1111-1111-111111111111";
const PAGE_ID: &str = "22222222-2222-2222-2222-222222222222";
const DISABLED: &str =
    r#"{"ok":false,"statusCode":401,"message":"The API is disabled."}"#;
const NOT_FOUND: &str = r#"{"ok":false,"statusCode":404,"message":"Not Found"}"#;

async fn read_request(socket: &mut TcpStream) -> Option<String> {
    let mut buf = Vec::new();
    let mut chunk = [0_u8; 4096];

    loop {
        let n = socket.read(&mut chunk).await.ok()?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(chunk.get(..n)?);

        let text = String::from_utf8_lossy(&buf);
        let Some((head, body)) = text.split_once("\r\n\r\n") else { continue };

        let length = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().ok())
                    .flatten()
            })
            .unwrap_or(0);

        if body.len() >= length {
            return Some(text.into_owned());
        }
    }

    Some(String::from_utf8_lossy(&buf).into_owned())
}

fn search_body(offset: usize) -> String {
    let (start, end) = if offset == 0 { (0, 100) } else { (100, 150) };

    let results = (start..end)
        .map(|n| {
            format!(
                r#"{{"id":"{n:08}-0000-0000-0000-000000000000","path":"p{n}","locale":"en","title":"P{n}","description":"d"}}"#
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(r#"{{"results":[{results}],"totalHits":150}}"#)
}

fn respond(request: &str) -> (&'static str, String) {
    let line = request.lines().next().unwrap_or_default();
    let authorized = request
        .lines()
        .any(|l| l.to_ascii_lowercase().starts_with("authorization:"));

    let site_route = format!("GET /_api/sites/{SITE}/");

    if line.starts_with("GET /_api/sites/127.0.0.1 ") {
        return if authorized {
            ("HTTP/1.1 401 Unauthorized", DISABLED.to_owned())
        } else {
            ("HTTP/1.1 200 OK", format!(r#"{{"id":"{SITE}"}}"#))
        };
    }

    if line.starts_with(&site_route) && authorized {
        return ("HTTP/1.1 401 Unauthorized", DISABLED.to_owned());
    }

    if line.starts_with(&format!("{site_route}pages/16cf565ca1ca91?")) {
        return (
            "HTTP/1.1 200 OK",
            format!(
                r##"{{"id":"{PAGE_ID}","path":"guides/setup","title":"Setup","content":"# Setup\nInstall it."}}"##
            ),
        );
    }

    if line.starts_with(&format!("{site_route}pages/{PAGE_ID}?")) {
        return (
            "HTTP/1.1 200 OK",
            format!(
                r#"{{"id":"{PAGE_ID}","path":"guides/setup","title":"Setup","render":"<h1 id=\"setup\">Setup</h1><p>Install it.</p>"}}"#
            ),
        );
    }

    if line.starts_with(&format!("{site_route}pages/search?")) {
        if line.contains("query=") {
            return (
                "HTTP/1.1 200 OK",
                format!(
                    r#"{{"results":[{{"id":"{PAGE_ID}","path":"guides/setup","title":"Setup","description":null}}],"totalHits":1}}"#
                ),
            );
        }

        let offset = if line.contains("offset=100") { 100 } else { 0 };
        return ("HTTP/1.1 200 OK", search_body(offset));
    }

    ("HTTP/1.1 404 Not Found", NOT_FOUND.to_owned())
}

async fn spawn_wiki() -> Option<String> {
    let listener = TcpListener::bind("127.0.0.1:0").await.ok()?;
    let addr = listener.local_addr().ok()?;

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let request = read_request(&mut socket).await.unwrap_or_default();
            let (status, body) = respond(&request);
            let response = format!(
                "{status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
        }
    });

    Some(format!("http://{addr}"))
}

fn config(url: &str) -> Option<WikiConfig> {
    let mut row = FaqSettingsRow::empty(1);
    row.enabled = true;
    row.wiki_url = Some(url.to_owned());
    row.wiki_api_key = Some(String::from("rejected-key"));

    WikiConfig::from_settings(&row).ok().flatten()
}

#[tokio::test]
async fn a_page_is_read_by_path() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");

    let page = wiki::page(&Client::new(), &config, "guides/setup")
        .await
        .expect("page reads anonymously");

    assert_eq!(page.content, "# Setup\nInstall it.");
}

#[tokio::test]
async fn a_page_without_source_is_converted_from_its_render() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");

    let page = wiki::page_by_id(&Client::new(), &config, PAGE_ID)
        .await
        .expect("page reads anonymously");

    assert!(page.content.contains("# Setup"), "{}", page.content);
    assert!(page.content.contains("Install it."), "{}", page.content);
}

#[tokio::test]
async fn an_unknown_id_reads_as_not_found() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");
    let id = "33333333-3333-3333-3333-333333333333";

    let err = wiki::page_by_id(&Client::new(), &config, id)
        .await
        .expect_err("an id the wiki does not have cannot be read");

    assert!(
        matches!(&err, WikiError::PageNotFound(found) if found == id),
        "expected PageNotFound({id}), got {err:?}"
    );
}

#[tokio::test]
async fn a_short_unknown_id_reads_as_not_found() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");

    let err = wiki::page_by_id(&Client::new(), &config, "999")
        .await
        .expect_err("an id the wiki does not have cannot be read");

    assert!(
        matches!(&err, WikiError::PageNotFound(id) if id == "999"),
        "expected PageNotFound(999), got {err:?}"
    );
}

#[tokio::test]
async fn listing_follows_every_batch() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");

    let items = wiki::list(&Client::new(), &config).await.expect("list reads");

    assert_eq!(items.len(), 150);
}

#[tokio::test]
async fn a_null_description_searches_as_empty() {
    let url = spawn_wiki().await.expect("start mock wiki");
    let config = config(&url).expect("config builds");

    let results =
        wiki::search(&Client::new(), &config, "setup").await.expect("search reads");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].description, "");
}
