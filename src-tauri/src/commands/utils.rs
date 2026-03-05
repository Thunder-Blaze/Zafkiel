use tauri::command;
use reqwest::Client;
use std::error::Error;

#[command]
pub async fn fetch_url(url: String) -> Result<String, String> {
    log::info!("Fetching URL: {}", url);
    let client = Client::new();
    let response = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()));
    }

    let text = response.text().await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
        
    Ok(text)
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
    log::info!("Uploading {} ({} bytes) to catbox.moe", filename, bytes.len());

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
            let msg = format!("Upload request failed: {} (is_connect={} is_timeout={} source={:?})", e, e.is_connect(), e.is_timeout(), e.source());
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
