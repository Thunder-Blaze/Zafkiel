/**
 * OAuth Authentication Module
 * Handles AniList OAuth flow with browser-based authorization
 */

use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub pending_auth: Arc<Mutex<Option<tokio::sync::oneshot::Sender<Result<String, String>>>>>,
    pub pending_receiver: Arc<Mutex<Option<tokio::sync::oneshot::Receiver<Result<String, String>>>>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            pending_auth: Arc::new(Mutex::new(None)),
            pending_receiver: Arc::new(Mutex::new(None)),
        }
    }
}

/// Find an available port, preferring 57575
pub fn find_available_port() -> Result<u16, String> {
    // Try preferred port first
    if TcpListener::bind(("127.0.0.1", 57575)).is_ok() {
        return Ok(57575);
    }

    // Try range around preferred port
    for port in 57576..=57600 {
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return Ok(port);
        }
    }

    // Let OS assign a port
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .map_err(|e| format!("Failed to bind to any port: {}", e))?;
    
    let port = listener.local_addr()
        .map_err(|e| format!("Failed to get local address: {}", e))?
        .port();
    
    Ok(port)
}

/// Start OAuth authorization flow
/// Returns the authorization URL to open
pub fn get_authorization_url(client_id: &str, redirect_port: u16) -> String {
    let redirect_uri = format!("http://localhost:{}/auth/callback", redirect_port);
    
    format!(
        "https://anilist.co/api/v2/oauth/authorize?client_id={}&redirect_uri={}&response_type=code",
        client_id,
        urlencoding::encode(&redirect_uri)
    )
}

/// Exchange authorization code for access token
pub async fn exchange_code_for_token(
    oauth_config: &OAuthConfig,
    code: &str,
    redirect_uri: &str,
) -> Result<TokenResponse, String> {
    log::info!("[Auth] Exchanging authorization code for token");
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://anilist.co/api/v2/oauth/token")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&serde_json::json!({
            "grant_type": "authorization_code",
            "client_id": oauth_config.client_id,
            "client_secret": oauth_config.client_secret,
            "redirect_uri": redirect_uri,
            "code": code
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send token request: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        log::error!("[Auth] Token exchange failed: {} - {}", status, error_text);
        return Err(format!("Token exchange failed: {} - {}", status, error_text));
    }

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    log::info!("[Auth] Successfully obtained access token");
    Ok(token_response)
}

/// Start local HTTP server to receive OAuth callback
pub async fn start_callback_server(
    port: u16,
    auth_state: AuthState,
) -> Result<(), String> {
    log::info!("[Auth] Starting callback server on port {}", port);

    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("Failed to bind callback server: {}", e))?;

    // Set non-blocking to allow timeout
    listener.set_nonblocking(false)
        .map_err(|e| format!("Failed to set non-blocking: {}", e))?;

    // Spawn server in background
    tokio::task::spawn(async move {
        log::info!("[Auth] Callback server listening on port {}", port);
        
        // Wait for exactly one connection (the OAuth callback)
        match listener.accept() {
            Ok((mut stream, _)) => {
                log::info!("[Auth] Received callback connection");
                
                if let Err(e) = handle_callback_request(&mut stream, auth_state).await {
                    log::error!("[Auth] Failed to handle callback: {}", e);
                }
            }
            Err(e) => {
                log::error!("[Auth] Failed to accept connection: {}", e);
            }
        }
    });

    Ok(())
}

async fn handle_callback_request(
    stream: &mut std::net::TcpStream,
    auth_state: AuthState,
) -> Result<(), String> {
    use std::io::{Read, Write};
    
    let mut buffer = [0u8; 4096];
    let bytes_read = stream.read(&mut buffer)
        .map_err(|e| format!("Failed to read request: {}", e))?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    log::debug!("[Auth] Callback request: {}", request);

    // Parse the request line to get the URL
    let first_line = request.lines().next().ok_or("Empty request")?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return Err("Invalid HTTP request".to_string());
    }

    let path = parts[1];
    
    // Parse URL to extract code
    let url = Url::parse(&format!("http://localhost{}", path))
        .map_err(|e| format!("Failed to parse callback URL: {}", e))?;

    let code = url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .ok_or("No authorization code found in callback")?;

    log::info!("[Auth] Extracted authorization code");

    // Send response to browser
    let response = "HTTP/1.1 200 OK\r\n\
                   Content-Type: text/html; charset=utf-8\r\n\
                   Connection: close\r\n\
                   \r\n\
                   <!DOCTYPE html>\
                   <html>\
                   <head>\
                       <meta charset='utf-8'>\
                       <title>Authentication Successful</title>\
                       <style>\
                           body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }\
                           .container { text-align: center; padding: 40px; background: rgba(255,255,255,0.1); border-radius: 20px; backdrop-filter: blur(10px); }\
                           h1 { margin: 0 0 20px 0; }\
                           p { margin: 10px 0; opacity: 0.9; }\
                       </style>\
                   </head>\
                   <body>\
                       <div class='container'>\
                           <h1>✅ Authentication Successful!</h1>\
                           <p>You have successfully logged in to AniList.</p>\
                           <p>You can close this window and return to the app.</p>\
                       </div>\
                   </body>\
                   </html>";

    stream.write_all(response.as_bytes())
        .map_err(|e| format!("Failed to send response: {}", e))?;
    stream.flush()
        .map_err(|e| format!("Failed to flush response: {}", e))?;

    // Send code through channel
    let mut pending = auth_state.pending_auth.lock().await;
    if let Some(sender) = pending.take() {
        let _ = sender.send(Ok(code));
    }

    Ok(())
}
