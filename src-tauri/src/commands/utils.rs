use base64::Engine as _;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use tauri::command;
use std::sync::Arc;
use std::time::Duration;

use std::sync::RwLock;
pub static DYNAMIC_USER_AGENT: RwLock<String> = RwLock::new(String::new());
pub const CHROME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

/// Proxy-fetch a URL through the Rust backend, bypassing browser CORS restrictions.
///
/// `headers` is an optional map of additional HTTP headers (e.g. `Cookie`, `Referer`).
/// The default `User-Agent` is always applied unless overridden via `headers`.
#[tauri::command]
fn get_client_builder(config: &crate::config::AppConfig) -> reqwest::ClientBuilder {
    let dynamic_ua = DYNAMIC_USER_AGENT.read().unwrap().clone();
    let user_agent = if dynamic_ua.is_empty() { CHROME_USER_AGENT } else { &dynamic_ua };
    let mut builder = reqwest::Client::builder()
        .user_agent(user_agent)
        .timeout(Duration::from_secs(30));

    if let Some(proxy_url) = &config.network.proxy_url {
        if !proxy_url.is_empty() {
            match reqwest::Proxy::all(proxy_url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                }
                Err(e) => {
                    log::error!("[Network] Invalid proxy URL '{}': {}", proxy_url, e);
                }
            }
        }
    }

    builder
}
fn apply_standard_headers(req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    let dynamic_ua = DYNAMIC_USER_AGENT.read().unwrap().clone();
    let is_dynamic = !dynamic_ua.is_empty();
    let user_agent = if is_dynamic { &dynamic_ua } else { CHROME_USER_AGENT };
    let mut req = req.header("User-Agent", user_agent)
       .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
       .header("Accept-Language", "en-US,en;q=0.9");
    // If not using dynamic UA (so we are defaulting to Chrome), or if the dynamic UA is Chrome-based
    if !is_dynamic || user_agent.contains("Chrome") {
        req = req.header("Sec-Ch-Ua", "\"Chromium\";v=\"124\", \"Google Chrome\";v=\"124\", \"Not-A.Brand\";v=\"99\"")
                 .header("Sec-Ch-Ua-Mobile", "?0")
                 .header("Sec-Ch-Ua-Platform", "\"Windows\"");
    }
    req.header("Sec-Fetch-Dest", "document")
       .header("Sec-Fetch-Mode", "navigate")
       .header("Sec-Fetch-Site", "none")
       .header("Sec-Fetch-User", "?1")
       .header("Upgrade-Insecure-Requests", "1")
}
/// Proxy-fetch a URL through the Rust backend, bypassing browser CORS restrictions.
#[tauri::command]
pub async fn fetch_url(
    url: String,
    headers: Option<HashMap<String, String>>,
    config_loader: tauri::State<'_, Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    let cfg = config_loader.get_config().map_err(|e| e.to_string())?;
    let client = get_client_builder(&cfg).build().map_err(|e| e.to_string())?;
    let mut req = client.get(&url);
    req = apply_standard_headers(req);
    if let Some(hdrs) = &headers {
        for (key, value) in hdrs {
            let key_lower = key.to_lowercase();
            // Block the extension from overriding the precise browser identity we enforce natively
            if key_lower == "user-agent" || key_lower.starts_with("sec-ch-ua") {
                log::info!("[fetch_url] Ignoring extension override for {}", key);
                continue;
            }
            if key_lower == "referer" {
                log::info!("[fetch_url] Using Referer: {}", value);
            }
            req = req.header(key.as_str(), value.as_str());
        }
    }
    let response = req.send().await.map_err(|e| {
        let msg = format!(
            "Request failed: {} (connect={} timeout={} source={:?})",
            e, e.is_connect(), e.is_timeout(), e.source()
        );
        log::error!("{}", msg);
        msg
    })?;
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    Ok(response.text().await.map_err(|e| e.to_string())?)
}

/// Proxy-POST a URL through the Rust backend (for form/JSON submissions).
#[tauri::command]
pub async fn post_url(
    url: String,
    body: String,
    headers: Option<HashMap<String, String>>,
    config_loader: tauri::State<'_, Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    let cfg = config_loader.get_config().map_err(|e| e.to_string())?;
    let client = get_client_builder(&cfg).build().map_err(|e| e.to_string())?;

    let mut req = client.post(&url).body(body);
    req = apply_standard_headers(req);

    if let Some(hdrs) = &headers {
        for (key, value) in hdrs {
            let key_lower = key.to_lowercase();
            if key_lower == "user-agent" || key_lower.starts_with("sec-ch-ua") { continue; }
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req.send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    Ok(response.text().await.map_err(|e| e.to_string())?)
}

/// Upload a file to catbox.moe via the server-side Rust backend.
#[command]
pub async fn upload_to_catbox(
    bytes: Vec<u8>,
    filename: String,
    mime_type: String,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .http1_only()
        .user_agent("curl/8.0.0")
        .build()
        .map_err(|e| e.to_string())?;

    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(filename.clone())
        .mime_str(&mime_type)
        .map_err(|e| e.to_string())?;

    let form = reqwest::multipart::Form::new().text("reqtype", "fileupload").part("fileToUpload", part);

    let response = client.post("https://catbox.moe/user/api.php").multipart(form).send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Catbox returned HTTP {}", response.status()));
    }

    let url = response.text().await.map_err(|e| e.to_string())?.trim().to_string();
    if url.starts_with("http") {
        Ok(url)
    } else {
        Err(format!("Unexpected catbox response: {}", url))
    }
}

/// Proxy-fetch a binary resource (e.g. HLS `.ts` segment) through the Rust backend.
#[command]
pub async fn fetch_bytes_base64(
    url: String,
    headers: Option<HashMap<String, String>>,
    config_loader: tauri::State<'_, Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    let cfg = config_loader.get_config().map_err(|e| e.to_string())?;
    let client = get_client_builder(&cfg).build().map_err(|e| e.to_string())?;

    let mut req = client.get(&url);
    req = apply_standard_headers(req);

    if let Some(hdrs) = &headers {
        for (key, value) in hdrs {
            let key_lower = key.to_lowercase();
            if key_lower == "user-agent" || key_lower.starts_with("sec-ch-ua") { continue; }
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req.send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

/// Fetch an image through the Rust backend and return it as a base64 data-URL.
#[command]
pub async fn fetch_image_base64(
    url: String,
    cookie: Option<String>,
    referer: Option<String>,
    config_loader: tauri::State<'_, Arc<crate::config::ConfigLoader>>,
) -> Result<String, String> {
    let cfg = config_loader.get_config().map_err(|e| e.to_string())?;
    let client = get_client_builder(&cfg).build().map_err(|e| e.to_string())?;

    let mut req = client.get(&url);
    req = apply_standard_headers(req);

    if let Some(cookie) = cookie { req = req.header("Cookie", cookie); }
    if let Some(referer) = referer { req = req.header("Referer", referer); }

    let response = req.send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let mime = response.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("image/jpeg").split(';').next().unwrap_or("image/jpeg").trim().to_string();
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}
