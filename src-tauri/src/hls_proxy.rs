/// Local HLS proxy server — the equivalent of Zenshin/Electron's `webSecurity: false`.
///
/// Tauri's WebKit webview enforces CORS and blocks forbidden headers (like
/// `Referer`) on XHR/fetch requests.  The cleanest solution, matching what
/// Zenshin's Express backend does for image proxying, is a local axum server
/// on 127.0.0.1 that forwards requests to the CDN with the correct headers.
///
/// hls.js is pointed at `http://127.0.0.1:{port}/proxy?url=<m3u8_url>` — no
/// custom loader, no base64 round-trips, just a real HTTP response that the
/// browser can receive on localhost (no CORS issues).
///
/// For m3u8 playlists the server rewrites absolute segment URLs inside the
/// playlist text so that each `.ts` request also goes through the proxy.
use axum::{
    extract::Query,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::TcpListener;
use std::sync::OnceLock;

// ─── Global state ────────────────────────────────────────────────────────────

static PROXY_PORT: OnceLock<u16> = OnceLock::new();
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent(UA)
            .pool_max_idle_per_host(8)
            .build()
            .expect("failed to build HLS proxy HTTP client")
    })
}

pub fn proxy_port() -> Option<u16> {
    PROXY_PORT.get().copied()
}

// ─── Query params ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ProxyParams {
    /// The target URL to proxy (percent-encoded).
    url: String,
    /// Optional Referer header value.
    referer: Option<String>,
    /// Optional Cookie header value.
    cookie: Option<String>,
}

// ─── Route handler ────────────────────────────────────────────────────────────

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                  (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

/// CORS headers added to every response (success or error) so the WebView can
/// always read the body, even when the upstream returns a non-2xx status.
fn cors_headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    h.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    h
}

async fn proxy_handler(Query(params): Query<ProxyParams>) -> impl IntoResponse {
    let target_url = params.url.trim().to_owned();
    log::debug!("[hls_proxy] → {}", target_url);

    let client = http_client();

    // Mimic a browser loading an HLS segment from inside the kwik embed page.
    //
    // Strategy:
    //   1. With Referer+Origin — CDN checks that requests come from the kwik
    //      embed domain (e.g. kwik.cx). Both Referer and Origin must match.
    //   2. No Referer          — fallback for servers that reject Referer.
    let build_req = |with_referer: bool| {
        let mut r = client
            .get(&target_url)
            .header("User-Agent", UA)
            .header("Accept", "*/*")
            .header("Accept-Language", "en-US,en;q=0.9")
            // Do NOT send Accept-Encoding — reqwest is built without the gzip
            // feature so it cannot decompress; the CDN must return raw bytes.
            .header("Connection", "keep-alive");
        if with_referer {
            if let Some(ref ref_val) = params.referer {
                r = r.header("Referer", ref_val);
                // Also send Origin so CDN CORS checks pass
                let origin = ref_val.trim_end_matches('/');
                r = r.header("Origin", origin);
            }
        }
        if let Some(ref c) = params.cookie {
            r = r.header("Cookie", c);
        }
        r
    };

    // First attempt: with Referer (kwik.si/ — what the real browser sends)
    let upstream = match build_req(true).send().await {
        Ok(r) if r.status().is_success() => {
            log::debug!("[hls_proxy] ok (with-referer) {}", target_url);
            r
        }
        Ok(bad) => {
            let s1 = bad.status();
            let b1 = bad.text().await.unwrap_or_default();
            log::warn!(
                "[hls_proxy] with-referer → {} for {}\n  body: {}",
                s1, target_url,
                if b1.len() > 256 { &b1[..256] } else { &b1 }
            );
            // Second attempt: no Referer
            match build_req(false).send().await {
                Ok(r2) if r2.status().is_success() => {
                    log::debug!("[hls_proxy] ok (no-referer) {}", target_url);
                    r2
                }
                Ok(bad2) => {
                    let s2 = bad2.status();
                    let b2 = bad2.text().await.unwrap_or_default();
                    log::warn!(
                        "[hls_proxy] no-referer → {} for {}\n  body: {}",
                        s2, target_url,
                        if b2.len() > 256 { &b2[..256] } else { &b2 }
                    );
                    return (
                        StatusCode::from_u16(s2.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                        cors_headers(),
                        format!("Upstream {s2} (tried with-referer→{s1}, no-referer→{s2}): {b2}").into_bytes(),
                    );
                }
                Err(e2) => {
                    log::error!("[hls_proxy] no-referer request error: {}", e2);
                    return (StatusCode::BAD_GATEWAY, cors_headers(), format!("Proxy error (retry): {e2}").into_bytes());
                }
            }
        }
        Err(e) => {
            log::error!("[hls_proxy] request error for {}: {}", target_url, e);
            return (StatusCode::BAD_GATEWAY, cors_headers(), format!("Proxy error: {e}").into_bytes());
        }
    };

    let content_type = upstream
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();

    let is_m3u8 = content_type.contains("mpegurl")
        || target_url.contains(".m3u8");

    let body_bytes = match upstream.bytes().await {
        Ok(b) => b,
        Err(e) => {
            log::error!("[hls_proxy] failed reading body: {}", e);
            return (StatusCode::BAD_GATEWAY, cors_headers(), format!("Body read error: {e}").into_bytes());
        }
    };

    // For m3u8 playlists: rewrite absolute segment/sub-playlist URLs so that
    // every subsequent hls.js request also passes through this proxy.
    let final_body: Vec<u8> = if is_m3u8 {
        let text = String::from_utf8_lossy(&body_bytes);
        let port = proxy_port().unwrap_or(0);

        // Build base URL of the manifest for resolving relative paths
        let manifest_base = target_url
            .rsplit_once('/')
            .map(|(base, _)| format!("{base}/"))
            .unwrap_or_else(|| target_url.clone());

        let rewritten = rewrite_m3u8(
            &text,
            &manifest_base,
            port,
            params.referer.as_deref(),
            params.cookie.as_deref(),
        );

        log::debug!(
            "[hls_proxy] m3u8 rewritten ({} → {} bytes)",
            body_bytes.len(),
            rewritten.len()
        );
        rewritten.into_bytes()
    } else {
        body_bytes.to_vec()
    };

    // Build response headers — CORS + correct content-type
    let mut headers = cors_headers();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap_or_else(|_| {
            HeaderValue::from_static("application/octet-stream")
        }),
    );
    if is_m3u8 {
        headers.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store"),
        );
    }

    (StatusCode::OK, headers, final_body)
}

/// Rewrite every URL in an m3u8 (segment lines AND #EXT-X-KEY URIs) so that
/// every subsequent request from hls.js passes through this proxy.
fn rewrite_m3u8(
    text: &str,
    base: &str,
    port: u16,
    referer: Option<&str>,
    cookie: Option<&str>,
) -> String {
    let rewritten = text.lines()
        .map(|line| {
            let t = line.trim();
            if t.is_empty() {
                return line.to_owned();
            }

            // Rewrite URI="..." inside #EXT-X-KEY (and similar tags).
            // These lines are comments but contain URLs that hls.js fetches
            // directly for AES-128 decryption keys — must also go through proxy.
            if t.starts_with('#') {
                if let Some(uri_start) = t.find("URI=\"") {
                    let after = &t[uri_start + 5..];
                    if let Some(uri_end) = after.find('"') {
                        let raw_uri = &after[..uri_end];
                        let abs = if raw_uri.starts_with("http://") || raw_uri.starts_with("https://") {
                            raw_uri.to_owned()
                        } else {
                            format!("{}/{}", base.trim_end_matches('/'), raw_uri.trim_start_matches('/'))
                        };
                        let proxied = build_proxy_url(port, &abs, referer, cookie);
                        return format!(
                            "{}URI=\"{}\"{}",
                            &t[..uri_start],
                            proxied,
                            &after[uri_end + 1..]
                        );
                    }
                }
                return line.to_owned();
            }

            // Regular segment / sub-playlist line
            let abs = if t.starts_with("http://") || t.starts_with("https://") {
                t.to_owned()
            } else {
                format!("{}{}", base.trim_end_matches('/'), if t.starts_with('/') { t.to_owned() } else { format!("/{t}") })
            };

            build_proxy_url(port, &abs, referer, cookie)
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Normalize AAC codec descriptor: mp4a.40.1 (AAC Main Profile) is rejected
    // by WebView2's MSE implementation.  mp4a.40.2 (AAC-LC) is identical in
    // practice — the bitstream is the same; only the descriptor differs — and is
    // universally supported.  Do the replacement on the full text so it covers
    // both EXT-X-STREAM-INF CODECS="..." attributes and EXT-X-MEDIA lines.
    rewritten.replace("mp4a.40.1", "mp4a.40.2")
}

/// Build a proxy URL for a target URL.
pub fn build_proxy_url(port: u16, url: &str, referer: Option<&str>, cookie: Option<&str>) -> String {
    let encoded_url = urlencoding::encode(url);
    let mut proxy = format!("http://127.0.0.1:{port}/proxy?url={encoded_url}");
    if let Some(r) = referer {
        proxy.push_str(&format!("&referer={}", urlencoding::encode(r)));
    }
    if let Some(c) = cookie {
        proxy.push_str(&format!("&cookie={}", urlencoding::encode(c)));
    }
    proxy
}

// ─── CORS preflight ───────────────────────────────────────────────────────────

async fn options_handler() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    (StatusCode::NO_CONTENT, headers)
}

// ─── Server startup ──────────────────────────────────────────────────────────

/// Start the HLS proxy on a random available port.
///
/// Designed to be called from the synchronous Tauri `setup()` closure —
/// the actual axum server runs inside `tauri::async_runtime::spawn`.
///
/// Subsequent calls are no-ops and return the already-running port.
pub fn start_sync() -> Result<u16, String> {
    // Idempotent: if already started just return the port
    if let Some(port) = PROXY_PORT.get() {
        return Ok(*port);
    }

    // Bind to port 0: the OS picks a free port
    let std_listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("Failed to bind HLS proxy listener: {e}"))?;
    let port = std_listener
        .local_addr()
        .map_err(|e| format!("Failed to get HLS proxy addr: {e}"))?
        .port();

    // Store port *before* spawning so m3u8 rewriter can read it
    PROXY_PORT
        .set(port)
        .map_err(|_| "HLS proxy already initialised".to_string())?;

    log::info!("[hls_proxy] starting on 127.0.0.1:{port}");

    // tokio requires the socket to be non-blocking before conversion
    std_listener
        .set_nonblocking(true)
        .map_err(|e| format!("Failed to set non-blocking on HLS proxy socket: {e}"))?;

    let app = Router::new()
        .route("/proxy", get(proxy_handler).options(options_handler));

    tauri::async_runtime::spawn(async move {
        let listener = tokio::net::TcpListener::from_std(std_listener)
            .expect("[hls_proxy] failed to convert std listener");
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|e| log::error!("[hls_proxy] server error: {e}"));
    });

    Ok(port)
}

// ─── Tauri command ────────────────────────────────────────────────────────────

/// Return the local HLS proxy port so the frontend can build proxy URLs.
#[tauri::command]
pub fn get_hls_proxy_port() -> Result<u16, String> {
    PROXY_PORT
        .get()
        .copied()
        .ok_or_else(|| "HLS proxy not started".to_string())
}
