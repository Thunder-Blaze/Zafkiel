use tauri::command;
use reqwest::Client;

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
