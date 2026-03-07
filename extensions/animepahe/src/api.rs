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
// Safe wrappers
// ─────────────────────────────────────────────────────────────────────────────

const BASE_URL: &str = "https://animepahe.si";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36";

/// Retrieve the stored cookie string (empty string if absent).
pub async fn load_cookies() -> String {
    match host_storage_get("cookies").await {
        Ok(v) if !v.is_null() && !v.is_undefined() => {
            v.as_string().unwrap_or_default()
        }
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

/// Build standard headers JSON for AnimePahe requests.
fn build_headers(cookies: &str, referer: Option<&str>) -> String {
    let mut obj = format!(
        r#"{{"User-Agent":{},"Cookie":{}}}"#,
        serde_json::to_string(USER_AGENT).unwrap(),
        serde_json::to_string(cookies).unwrap(),
    );
    if let Some(r) = referer {
        // insert before closing brace
        obj.pop(); // remove '}'
        let _ = write!(
            &mut obj,
            r#","Referer":{}}}"#,
            serde_json::to_string(r).unwrap()
        );
    }
    obj
}

use std::fmt::Write;

/// GET `path` relative to animepahe.si with cookie auth.
pub async fn pahe_get(path: &str, cookies: &str) -> Result<String, String> {
    let url = format!("{BASE_URL}{path}");
    let headers = build_headers(cookies, None);
    fetch(&url, &headers).await
}

/// GET an arbitrary absolute URL with a `Referer: https://animepahe.si` header.
#[allow(dead_code)]
pub async fn fetch_with_referer(url: &str, cookies: &str) -> Result<String, String> {
    let headers = build_headers(cookies, Some(BASE_URL));
    fetch(url, &headers).await
}

/// GET a Kwik.si URL — sends **only** `Referer` + `User-Agent`, never animepahe cookies.
/// Kwik rejects requests that carry third-party Cookie headers.
pub async fn fetch_kwik(url: &str) -> Result<String, String> {
    let headers = format!(
        r#"{{"User-Agent":{},"Referer":"https://animepahe.si/"}}"#,
        serde_json::to_string(USER_AGENT).unwrap_or_default(),
    );
    fetch(url, &headers).await
}

/// Raw fetch — returns the response body as a String.
pub async fn fetch(url: &str, headers_json: &str) -> Result<String, String> {
    match host_fetch_url(url, headers_json).await {
        Ok(v) => Ok(v.as_string().unwrap_or_default()),
        Err(e) => {
            let msg = e
                .as_string()
                .unwrap_or_else(|| format!("{e:?}"));
            Err(format!("fetch_url failed for '{url}': {msg}"))
        }
    }
}

pub fn log_info(msg: &str) {
    host_log("info", msg);
}

pub fn log_warn(msg: &str) {
    host_log("warn", msg);
}
