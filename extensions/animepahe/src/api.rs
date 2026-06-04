/// Host API bindings — all network I/O is routed through the Zafkiel host.
///
/// The host injects a `__zafkiel_host__` object into the WASM import namespace
/// at load time.  These `extern "C"` declarations bind to those JS functions.
use wasm_bindgen::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Raw host imports
// ─────────────────────────────────────────────────────────────────────────────

#[wasm_bindgen]
extern "C" {
    /// Make an HTTP GET request through the Tauri backend (bypasses CORS).
    /// `headers_json` is a JSON-encoded `{ "Header-Name": "value" }` object.
    /// Returns the response body as a UTF-8 string.
    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = fetchUrl, catch)]
    async fn host_fetch_url(url: &str, headers_json: &str) -> Result<JsValue, JsValue>;

    /// Read a value from the extension's persistent key-value store.
    /// Returns `null` (JsValue::NULL) if the key doesn't exist.
    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = storageGet, catch)]
    async fn host_storage_get(key: &str) -> Result<JsValue, JsValue>;

    /// Write a value into the extension's persistent key-value store.
    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = storageSet, catch)]
    async fn host_storage_set(key: &str, value: &str) -> Result<JsValue, JsValue>;

    /// Emit a log message to the host (forwarded to Tauri log + devtools).
    #[wasm_bindgen(js_namespace = __zafkiel_host__, js_name = log)]
    fn host_log(level: &str, message: &str);
}

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const BASE_URL: &str = "https://animepahe.pw";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

// ─────────────────────────────────────────────────────────────────────────────
// Storage helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Retrieve the stored cookie string (empty string if absent).
pub async fn load_cookies() -> String {
    match host_storage_get("cookies").await {
        Ok(v) if !v.is_null() && !v.is_undefined() => v.as_string().unwrap_or_default(),
        _ => String::new(),
    }
}

/// Persist cookie string to extension storage.
pub async fn save_cookies(cookies: &str) {
    let _ = host_storage_set("cookies", cookies).await;
}

/// Retrieve the stored kwik.si cookie string (empty string if absent).
pub async fn load_kwik_cookies() -> String {
    match host_storage_get("kwik_cookies").await {
        Ok(v) if !v.is_null() && !v.is_undefined() => v.as_string().unwrap_or_default(),
        _ => String::new(),
    }
}

/// Persist kwik.si cookie string to extension storage.
pub async fn save_kwik_cookies(cookies: &str) {
    let _ = host_storage_set("kwik_cookies", cookies).await;
}

// ─────────────────────────────────────────────────────────────────────────────
// Header builders
// ─────────────────────────────────────────────────────────────────────────────

/// Headers for AnimePahe **JSON API** calls (`/api?m=…`).
///
/// AnimePahe's API endpoints are served via XHR; without `X-Requested-With`
/// and an `Accept: application/json` header the server returns 403.
fn build_api_headers(cookies: &str) -> String {
    format!(
        r#"{{"User-Agent":{ua},"Accept":"application/json, text/javascript, */*; q=0.01","Accept-Language":"en-US,en;q=0.5","X-Requested-With":"XMLHttpRequest","Referer":"{base}/","Cookie":{cookie}}}"#,
        ua     = serde_json::to_string(USER_AGENT).unwrap(),
        base   = BASE_URL,
        cookie = serde_json::to_string(cookies).unwrap(),
    )
}

/// Headers for AnimePahe **HTML page** requests (`/anime/…`, `/play/…`).
fn build_html_headers(cookies: &str) -> String {
    format!(
        r#"{{"User-Agent":{ua},"Accept":"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8","Accept-Language":"en-US,en;q=0.5","Referer":"{base}/","Cookie":{cookie}}}"#,
        ua     = serde_json::to_string(USER_AGENT).unwrap(),
        base   = BASE_URL,
        cookie = serde_json::to_string(cookies).unwrap(),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Public fetch helpers
// ─────────────────────────────────────────────────────────────────────────────

/// GET a `/api?m=…` endpoint with proper AJAX (XHR) headers.
/// Use for: search, episodes, airing, user lists.
pub async fn pahe_api_get(path: &str, cookies: &str) -> Result<String, String> {
    let url = format!("{BASE_URL}{path}");
    fetch(&url, &build_api_headers(cookies)).await
}

/// GET an HTML page (`/anime/…`, `/play/…`) with browser-like headers.
pub async fn pahe_get(path: &str, cookies: &str) -> Result<String, String> {
    let url = format!("{BASE_URL}{path}");
    fetch(&url, &build_html_headers(cookies)).await
}

/// GET a Kwik.si embed page.
///
/// Sends a `Referer` pointing back to AnimePahe so Kwik accepts the request.
/// If `kwik_cookies` is non-empty they are forwarded (needed when the Kwik
/// subdomain is behind a Cloudflare challenge that has already been solved).
pub async fn fetch_kwik(url: &str, kwik_cookies: &str) -> Result<String, String> {
    let cookie_field = if kwik_cookies.is_empty() {
        String::new()
    } else {
        format!(r#","Cookie":{}"#, serde_json::to_string(kwik_cookies).unwrap())
    };
    let headers = format!(
        r#"{{"User-Agent":{ua},"Accept":"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8","Accept-Language":"en-US,en;q=0.5","Referer":"{base}/"{cookie}}}"#,
        ua     = serde_json::to_string(USER_AGENT).unwrap(),
        base   = BASE_URL,
        cookie = cookie_field,
    );
    fetch(url, &headers).await
}

/// Raw fetch — returns the response body as a UTF-8 String.
pub async fn fetch(url: &str, headers_json: &str) -> Result<String, String> {
    match host_fetch_url(url, headers_json).await {
        Ok(v) => Ok(v.as_string().unwrap_or_default()),
        Err(e) => {
            let msg = e.as_string().unwrap_or_else(|| format!("{e:?}"));
            Err(format!("fetch_url failed for '{url}': {msg}"))
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Logging
// ─────────────────────────────────────────────────────────────────────────────

pub fn log_info(msg: &str) {
    host_log("info", msg);
}

pub fn log_warn(msg: &str) {
    host_log("warn", msg);
}
