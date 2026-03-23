use base64::Engine as _;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use tauri::command;

/// Proxy-fetch a URL through the Rust backend, bypassing browser CORS restrictions.
///
/// `headers` is an optional map of additional HTTP headers (e.g. `Cookie`, `Referer`).
/// The default `User-Agent` is always applied unless overridden via `headers`.
#[command]
pub async fn fetch_url(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    log::info!("Fetching URL: {}", url);
    let client = Client::new();
    let mut req = client.get(&url).header(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    );

    if let Some(hdrs) = headers {
        for (key, value) in &hdrs {
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(text)
}

/// Proxy-POST a URL through the Rust backend (for form/JSON submissions).
#[command]
pub async fn post_url(
    url: String,
    body: String,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    log::info!("POSTing URL: {}", url);
    let client = Client::new();
    let mut req = client
        .post(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        )
        .body(body);

    if let Some(hdrs) = headers {
        for (key, value) in &hdrs {
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))
}

/// Upload a file to catbox.moe via the server-side Rust backend, bypassing CORS
/// restrictions that would block the upload from the browser webview.
///
/// Returns the catbox URL on success (e.g. "https://files.catbox.moe/abc123.jpg").
#[command]
pub async fn upload_to_catbox(
    bytes: Vec<u8>,
    filename: String,
    mime_type: String,
) -> Result<String, String> {
    log::info!(
        "Uploading {} ({} bytes) to catbox.moe",
        filename,
        bytes.len()
    );

    let client = reqwest::Client::builder()
        .http1_only()
        .user_agent("curl/8.0.0")
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(filename.clone())
        .mime_str(&mime_type)
        .map_err(|e| format!("Invalid MIME type '{}': {}", mime_type, e))?;

    let form = reqwest::multipart::Form::new()
        .text("reqtype", "fileupload")
        .part("fileToUpload", part);

    let response = client
        .post("https://catbox.moe/user/api.php")
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            let msg = format!(
                "Upload request failed: {} (is_connect={} is_timeout={} source={:?})",
                e,
                e.is_connect(),
                e.is_timeout(),
                e.source()
            );
            log::error!("{}", msg);
            msg
        })?;

    if !response.status().is_success() {
        return Err(format!("Catbox returned HTTP {}", response.status()));
    }

    let url = response
        .text()
        .await
        .map_err(|e| format!("Failed to read catbox response: {}", e))?;

    let url = url.trim().to_string();
    if url.starts_with("http") {
        log::info!("Catbox upload successful: {}", url);
        Ok(url)
    } else {
        Err(format!("Unexpected catbox response: {}", url))
    }
}
/// Proxy-fetch a binary resource (e.g. HLS `.ts` segment) through the Rust
/// backend and return the raw bytes encoded as standard base64 (no data-URI
/// prefix).  The caller is responsible for decoding the base64 string to an
/// `ArrayBuffer` on the JS side.
///
/// This lets the frontend (hls.js custom loader) bypass both CORS restrictions
/// and browser-forbidden headers such as `Referer`.
#[command]
pub async fn fetch_bytes_base64(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    log::info!("Proxying binary as base64: {}", url);
    let client = Client::new();
    let mut req = client.get(&url).header(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/124.0 Safari/537.36",
    );

    if let Some(hdrs) = headers {
        for (key, value) in &hdrs {
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;

    Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
}

/// Fetch an image through the Rust backend and return it as a base64 data-URL.
///
/// This is used to proxy CDN images (e.g. `i.animepahe.si`) that require a
/// `Cookie` header — `<img>` tags cannot set arbitrary request headers, so the
/// browser would get a 403 without this proxy.
///
/// Returns a string of the form `data:<mime>;base64,<data>` that can be used
/// directly as an `<img src="...">` value.
#[command]
pub async fn fetch_image_base64(
    url: String,
    cookie: Option<String>,
    referer: Option<String>,
) -> Result<String, String> {
    log::info!("Proxying image as base64: {}", url);
    let client = Client::new();
    let mut req = client.get(&url).header(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/124.0 Safari/537.36",
    );

    if let Some(cookie) = cookie {
        req = req.header("Cookie", cookie);
    }
    if let Some(referer) = referer {
        req = req.header("Referer", referer);
    }

    let response = req
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    // Determine MIME type from Content-Type header, defaulting to image/jpeg.
    let mime = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .split(';')
        .next()
        .unwrap_or("image/jpeg")
        .trim()
        .to_string();

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}
