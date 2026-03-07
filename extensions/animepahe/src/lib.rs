mod api;
mod scraper;
mod types;
mod unpacker;

use std::cell::RefCell;
use types::*;
use wasm_bindgen::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Extension state (single-threaded WASM – RefCell is safe)
// ─────────────────────────────────────────────────────────────────────────────

thread_local! {
    /// Session cookies for animepahe.si, cached in memory for the session.
    static COOKIES: RefCell<String> = const { RefCell::new(String::new()) };
    /// Cloudflare clearance cookies for kwik.si, used for CDN requests.
    static KWIK_COOKIES: RefCell<String> = const { RefCell::new(String::new()) };
}

fn get_cookies() -> String {
    COOKIES.with(|c| c.borrow().clone())
}

fn set_cookies(s: String) {
    COOKIES.with(|c| *c.borrow_mut() = s);
}

fn get_kwik_cookies() -> String {
    KWIK_COOKIES.with(|c| c.borrow().clone())
}

fn set_kwik_cookies(s: String) {
    KWIK_COOKIES.with(|c| *c.borrow_mut() = s);
}

// ─────────────────────────────────────────────────────────────────────────────
// Manifest
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the extension manifest as a JSON string.
#[wasm_bindgen]
pub fn get_manifest() -> String {
    // Install the panic hook once so every subsequent panic emits a console.error
    // with the exact Rust source location before triggering the WASM unreachable trap.
    console_error_panic_hook::set_once();
    r#"{
  "id": "animepahe",
  "name": "AnimePahe",
  "version": "0.1.0",
  "author": "Zafkiel",
  "description": "AnimePahe streaming source – HLS playback via Kwik.si",
  "type": "source",
  "entry": "extension.wasm",
  "minAppVersion": "0.1.0",
  "requires": {
    "cookieAuth": true,
    "network": true,
    "storage": true
  },
  "allowedDomains": ["animepahe.si", "animepahe.com", "i.animepahe.si", "kwik.si", "kwik.cx"]
}"#
    .to_owned()
}

// ─────────────────────────────────────────────────────────────────────────────
// Cookie auth
// ─────────────────────────────────────────────────────────────────────────────

/// Called by the host after the user completes the Cloudflare challenge.
/// `cookies_string` is the full `Cookie:` header value
/// (e.g. `"cf_clearance=xxx; PHPSESSID=yyy"`).
#[wasm_bindgen]
pub async fn on_cookies_updated(cookies_string: String) {
    set_cookies(cookies_string.clone());
    api::save_cookies(&cookies_string).await;
    api::log_info("Cookies updated and persisted");
}

/// Called by the host after the user completes the Cloudflare challenge on kwik.si.
/// Stores the cookies so that `resolve_stream` can forward them to the CDN.
#[wasm_bindgen]
pub async fn on_kwik_cookies_updated(cookies_string: String) {
    set_kwik_cookies(cookies_string.clone());
    api::save_kwik_cookies(&cookies_string).await;
    api::log_info("Kwik cookies updated and persisted");
}

/// Returns `true` if the extension has valid cookies and can reach AnimePahe.
#[wasm_bindgen]
pub async fn check_auth() -> bool {
    // Load from storage if not already in memory
    let cookies = {
        let c = get_cookies();
        if c.is_empty() {
            let stored = api::load_cookies().await;
            if stored.is_empty() {
                return false;
            }
            set_cookies(stored.clone());
            stored
        } else {
            c
        }
    };

    // Probe with a cheap airing API call to verify cookies still work
    match api::pahe_get("/api?m=airing&page=1", &cookies).await {
        Ok(body) => {
            let ok = !body.contains("\"status\":403")
                && !body.contains("Just a moment")
                && !body.contains("cf-error");
            if !ok {
                api::log_warn("AnimePahe auth probe failed – cookies may have expired");
            }
            ok
        }
        Err(e) => {
            api::log_warn(&format!("AnimePahe auth probe error: {e}"));
            false
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Search
// ─────────────────────────────────────────────────────────────────────────────

/// Search AnimePahe by title. Returns a JSON-encoded `SearchResult[]`.
#[wasm_bindgen]
pub async fn search(query: String) -> Result<String, String> {
    let cookies = ensure_cookies().await?;
    let path = format!(
        "/api?m=search&q={}",
        url_encode(&query)
    );
    let raw = api::pahe_get(&path, &cookies).await?;

    let resp: PaheSearchResponse =
        serde_json::from_str(&raw).map_err(|e| format!("Failed to parse search response: {e}"))?;

    let results: Vec<SearchResult> = resp
        .data
        .unwrap_or_default()
        .into_iter()
        .map(|item| SearchResult {
            id: item.session.clone(),
            title: item.title,
            cover_url: item.poster.map(|p| {
                if p.starts_with("http") {
                    p
                } else {
                    format!("https://i.animepahe.si/posters/{p}")
                }
            }),
            anilist_id: None,
            r#type: item.media_type,
            status: item.status,
            year: item.year,
        })
        .collect();

    serde_json::to_string(&results).map_err(|e| format!("Serialization error: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Anime details  (GET /anime/{session})
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a JSON-encoded `AnimeDetails` for the given AnimePahe session slug.
#[wasm_bindgen]
pub async fn get_anime_details(session: String) -> Result<String, String> {
    let cookies = ensure_cookies().await?;
    let html = api::pahe_get(&format!("/anime/{session}"), &cookies).await?;
    let details = scraper::scrape_anime_details(&html, &session);
    serde_json::to_string(&details).map_err(|e| format!("Serialization error: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Episodes  (GET /api?m=release&id={session}&page={n})
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a JSON-encoded `Page<Episode>`. Pass `page = 0` to use page 1.
#[wasm_bindgen]
pub async fn get_episodes(anime_session: String, page: u32) -> Result<String, String> {
    let cookies = ensure_cookies().await?;
    let p = if page == 0 { 1 } else { page };
    let path = format!("/api?m=release&id={anime_session}&sort=episode_asc&page={p}");
    let raw = api::pahe_get(&path, &cookies).await?;

    let resp: PaheEpisodesResponse = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse episodes response: {e}"))?;

    let episodes: Vec<Episode> = resp
        .data
        .unwrap_or_default()
        .into_iter()
        .map(|ep| Episode {
            id: ep.session.clone(),
            number: ep.episode,
            title: Some(format!("Episode {}", ep.episode)),
            thumbnail_url: ep.snapshot.as_deref().map(|s| {
                if s.starts_with("http") {
                    s.to_owned()
                } else {
                    format!("https://i.animepahe.si/snapshots/{s}")
                }
            }),
            air_date: ep.created_at,
            slug: ep.session,
        })
        .collect();

    let page_result = Page {
        data: episodes,
        current_page: resp.current_page,
        last_page: resp.last_page,
        total: resp.total,
        per_page: resp.per_page,
    };

    serde_json::to_string(&page_result).map_err(|e| format!("Serialization error: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Stream sources  (scrape /play/{animeSession}/{episodeSession})
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a JSON-encoded `StreamSource[]` for a given episode.
#[wasm_bindgen]
pub async fn get_stream_sources(
    anime_session: String,
    episode_session: String,
) -> Result<String, String> {
    let cookies = ensure_cookies().await?;
    let html = api::pahe_get(&format!("/play/{anime_session}/{episode_session}"), &cookies).await?;
    let sources = scraper::scrape_stream_sources(&html);

    if sources.is_empty() {
        api::log_warn(&format!(
            "No stream sources found for episode '{episode_session}' – cookies may be expired"
        ));
    }

    serde_json::to_string(&sources).map_err(|e| format!("Serialization error: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Resolve stream  (GET Kwik URL → unpack PACKAD → HLS URL)
// ─────────────────────────────────────────────────────────────────────────────

/// Returns a JSON-encoded `ResolvedStream` for the given `StreamSource` JSON.
///
/// The source JSON must be the `StreamSource` returned by `get_stream_sources`.
#[wasm_bindgen]
pub async fn resolve_stream(source_json: String) -> Result<String, String> {
    let source: StreamSource = serde_json::from_str(&source_json)
        .map_err(|e| format!("Invalid source JSON: {e}"))?;

    api::log_info(&format!("[resolve_stream] fetching kwik URL: {}", source.id));

    // Fetch the Kwik page — only Referer+UA, no cookies (Kwik rejects third-party cookies)
    let kwik_html = api::fetch_kwik(&source.id).await.map_err(|e| {
        api::log_warn(&format!("[resolve_stream] fetch_kwik FAILED: {e}"));
        e
    })?;

    api::log_info(&format!(
        "[resolve_stream] kwik page fetched ({} chars), scanning for video URL",
        kwik_html.len()
    ));

    // Log a snippet of the page to help debug if unpacking fails
    let preview: String = kwik_html.chars().take(300).collect();
    api::log_info(&format!("[resolve_stream] page preview: {preview}"));

    // Unpack the PACKAD-obfuscated script to get the .m3u8 URL
    let video_url = unpacker::extract_video_url(&kwik_html).ok_or_else(|| {
        let msg = format!(
            "Could not extract video URL from Kwik page '{}'. \
             The page format may have changed.",
            source.id
        );
        api::log_warn(&format!("[resolve_stream] {msg}"));
        msg
    })?;

    api::log_info(&format!("[resolve_stream] extracted URL: {video_url}"));

    let mut headers = std::collections::HashMap::new();
    headers.insert("Referer".to_owned(), "https://kwik.si/".to_owned());

    // Include kwik.si Cloudflare cookies if available (needed when the CDN
    // enforces CF protection — obtained via the kwik auth webview).
    let kwik_cookies = {
        let c = get_kwik_cookies();
        if c.is_empty() {
            api::load_kwik_cookies().await
        } else {
            c
        }
    };
    if !kwik_cookies.is_empty() {
        set_kwik_cookies(kwik_cookies.clone());
        headers.insert("Cookie".to_owned(), kwik_cookies);
        api::log_info("[resolve_stream] kwik cookies added to CDN headers");
    }

    let resolved = ResolvedStream {
        url: video_url,
        r#type: "hls".to_owned(),
        headers,
    };

    serde_json::to_string(&resolved).map_err(|e| format!("Serialization error: {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Ensure cookies are loaded (from memory or storage). Returns an error if
/// no cookies are available so callers can surface a meaningful auth error.
async fn ensure_cookies() -> Result<String, String> {
    let c = get_cookies();
    if !c.is_empty() {
        return Ok(c);
    }
    let stored = api::load_cookies().await;
    if stored.is_empty() {
        return Err(
            "AnimePahe requires authentication. \
             Please open the AnimePahe auth window and solve the Cloudflare challenge."
                .to_owned(),
        );
    }
    set_cookies(stored.clone());
    Ok(stored)
}

/// Percent-encode a query string value (RFC 3986 unreserved chars pass through).
fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push('+'),
            b => {
                out.push('%');
                out.push(char::from_digit((b >> 4) as u32, 16).unwrap().to_ascii_uppercase());
                out.push(char::from_digit((b & 0xf) as u32, 16).unwrap().to_ascii_uppercase());
            }
        }
    }
    out
}
