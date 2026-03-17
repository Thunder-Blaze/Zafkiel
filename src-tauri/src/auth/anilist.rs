/**
 * OAuth Authentication Module
 * Handles AniList OAuth implicit grant flow with browser-based authorization.
 */
use std::{net::TcpListener, sync::Arc};
use tokio::sync::{
    Mutex,
    oneshot::{Receiver, Sender},
};
use url::Url;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub pending_auth: Arc<Mutex<Option<Sender<Result<String, String>>>>>,
    pub pending_receiver: Arc<Mutex<Option<Receiver<Result<String, String>>>>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            pending_auth: Arc::new(Mutex::new(None)),
            pending_receiver: Arc::new(Mutex::new(None)),
        }
    }
}

/// Start OAuth authorization flow
/// Returns the authorization URL to open
pub fn get_authorization_url(client_id: &str) -> String {
    format!(
        "https://anilist.co/api/v2/oauth/authorize?client_id={}&response_type=token",
        client_id
    )
}

/// Start local HTTP server to receive OAuth callback
pub async fn start_callback_server(port: u16, auth_state: AuthState) -> Result<(), String> {
    log::info!("[Auth] Starting callback server on port {}", port);

    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("Failed to bind callback server: {}", e))?;

    // Set non-blocking to allow timeout
    listener
        .set_nonblocking(false)
        .map_err(|e| format!("Failed to set non-blocking: {}", e))?;

    // Spawn server in background
    tokio::task::spawn(async move {
        log::info!("[Auth] Callback server listening on port {}", port);

        // For implicit flow, browser first lands on /callback with hash
        // fragment (not visible to server), then JS re-requests with query.
        for _ in 0..8 {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    log::info!("[Auth] Received callback connection");
                    match handle_callback_request(&mut stream, auth_state.clone()).await {
                        Ok(done) => {
                            if done {
                                break;
                            }
                        }
                        Err(e) => {
                            log::error!("[Auth] Failed to handle callback: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("[Auth] Failed to accept connection: {}", e);
                    break;
                }
            }
        }
    });

    Ok(())
}

async fn handle_callback_request(
    stream: &mut std::net::TcpStream,
    auth_state: AuthState,
) -> Result<bool, String> {
    use std::io::{Read, Write};

    let mut buffer = [0u8; 4096];
    let bytes_read = stream
        .read(&mut buffer)
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

    // Parse callback URL
    let url = Url::parse(&format!("http://localhost{}", path))
        .map_err(|e| format!("Failed to parse callback URL: {}", e))?;

    // AniList implicit grant returns params in URL fragment.
    // Fragment is not sent to server, so we first serve JS to turn
    // location.hash into query params, then parse token from query.
    let access_token = url
        .query_pairs()
        .find(|(key, _)| key == "access_token")
        .map(|(_, value)| value.to_string());

    if let Some(token) = access_token {
        log::info!("[Auth] Extracted implicit access token");

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
                           <h1>Authentication Successful</h1>\
                           <p>You have successfully logged in to AniList.</p>\
                           <p>You can close this window and return to the app.</p>\
                       </div>\
                   </body>\
                   </html>";

        stream
            .write_all(response.as_bytes())
            .map_err(|e| format!("Failed to send response: {}", e))?;
        stream
            .flush()
            .map_err(|e| format!("Failed to flush response: {}", e))?;

        let mut pending = auth_state.pending_auth.lock().await;
        if let Some(sender) = pending.take() {
            let _ = sender.send(Ok(token));
        }
        return Ok(true);
    }

    if let Some(err) = url
        .query_pairs()
        .find(|(key, _)| key == "error")
        .map(|(_, value)| value.to_string())
    {
        let response = "HTTP/1.1 400 Bad Request\r\n\
                   Content-Type: text/html; charset=utf-8\r\n\
                   Connection: close\r\n\
                   \r\n\
                   <!DOCTYPE html><html><body><h1>Authentication Failed</h1><p>Please return to the app and try again.</p></body></html>";
        stream
            .write_all(response.as_bytes())
            .map_err(|e| format!("Failed to send response: {}", e))?;
        stream
            .flush()
            .map_err(|e| format!("Failed to flush response: {}", e))?;

        let mut pending = auth_state.pending_auth.lock().await;
        if let Some(sender) = pending.take() {
            let _ = sender.send(Err(format!("AniList OAuth error: {}", err)));
        }
        return Ok(true);
    }

    // First leg of implicit callback: hash is only in browser.
    let response = "HTTP/1.1 200 OK\r\n\
                   Content-Type: text/html; charset=utf-8\r\n\
                   Connection: close\r\n\
                   \r\n\
                   <!DOCTYPE html>\
                   <html>\
                   <head>\
                       <meta charset='utf-8'>\
                       <title>Completing Authentication</title>\
                       <style>\
                           body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; }\
                           .container { text-align: center; padding: 40px; background: rgba(255,255,255,0.1); border-radius: 20px; backdrop-filter: blur(10px); }\
                           h1 { margin: 0 0 20px 0; }\
                           p { margin: 10px 0; opacity: 0.9; }\
                       </style>\
                   </head>\
                   <body>\
                       <div class='container'>\
                           <h1>Completing login...</h1>\
                           <p>Please wait while we finalize authentication.</p>\
                           <p>If this page does not close, return to the app.</p>\
                       </div>\
                       <script>\
                           (function () {\
                               if (!window.location.hash) return;\
                               var params = new URLSearchParams(window.location.hash.substring(1));\
                               if (!params.has('access_token') && !params.has('error')) return;\
                               var next = window.location.pathname + '?' + params.toString();\
                               window.location.replace(next);\
                           })();\
                       </script>\
                   </body>\
                   </html>";

    stream
        .write_all(response.as_bytes())
        .map_err(|e| format!("Failed to send response: {}", e))?;
    stream
        .flush()
        .map_err(|e| format!("Failed to flush response: {}", e))?;

    Ok(false)
}
