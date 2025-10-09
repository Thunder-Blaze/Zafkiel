/**
 * Tauri Commands for OAuth Authentication
 */
use crate::anilist::AniListService;
use crate::auth::{
    AuthState, OAuthConfig, exchange_code_for_token, find_available_port, get_authorization_url,
    start_callback_server,
};
use crate::config;
use std::sync::Arc;
use tauri::{AppHandle, State};

/// Initialize OAuth flow
/// Returns the authorization URL and callback port
#[tauri::command]
pub async fn start_oauth_flow(
    _app: AppHandle,
    auth_state: State<'_, AuthState>,
) -> Result<(String, u16), String> {
    log::info!("[Auth Command] Starting OAuth flow");

    // Load OAuth config from environment
    let client_id = std::env::var("ANILIST_CLIENT_ID")
        .map_err(|_| "ANILIST_CLIENT_ID not found in environment. Make sure .env file exists with ANILIST_CLIENT_ID and ANILIST_CLIENT_SECRET".to_string())?;

    if client_id.is_empty() {
        return Err("ANILIST_CLIENT_ID is empty".to_string());
    }

    // Create a channel to receive the authorization code
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Store the receiver in auth state BEFORE starting server
    {
        let mut pending = auth_state.pending_auth.lock().await;
        *pending = Some(tx);
    }

    // Store the receiver for later retrieval
    {
        let mut receiver = auth_state.pending_receiver.lock().await;
        *receiver = Some(rx);
    }

    // Find available port
    let port = find_available_port()?;
    log::info!("[Auth Command] Using port {} for callback", port);

    // Start callback server
    start_callback_server(port, auth_state.inner().clone()).await?;

    // Generate authorization URL
    let auth_url = get_authorization_url(&client_id, port);
    log::info!("[Auth Command] Authorization URL generated");

    Ok((auth_url, port))
}

/// Open authorization URL in default browser
/// Falls back to webview if no browser available
#[tauri::command]
pub async fn open_auth_browser(app: AppHandle, auth_url: String) -> Result<(), String> {
    log::info!("[Auth Command] Opening authorization URL");

    // Try to open in default browser
    match open::that(&auth_url) {
        Ok(_) => {
            log::info!("[Auth Command] Opened URL in default browser");
            Ok(())
        }
        Err(e) => {
            log::warn!("[Auth Command] Failed to open default browser: {}", e);
            log::info!("[Auth Command] Falling back to WebView");

            // Fallback: Open in Tauri webview window
            let _webview_window = tauri::WebviewWindowBuilder::new(
                &app,
                "oauth",
                tauri::WebviewUrl::External(auth_url.parse().unwrap()),
            )
            .title("AniList Login")
            .inner_size(600.0, 800.0)
            .center()
            .resizable(true)
            .build()
            .map_err(|e| format!("Failed to create webview window: {}", e))?;

            Ok(())
        }
    }
}

/// Wait for OAuth callback and exchange code for token
#[tauri::command]
pub async fn wait_for_oauth_callback(
    auth_state: State<'_, AuthState>,
    anilist_service: State<'_, Arc<AniListService>>,
) -> Result<String, String> {
    log::info!("[Auth Command] Waiting for OAuth callback");

    // Get the receiver that was created in start_oauth_flow
    let rx = {
        let mut receiver = auth_state.pending_receiver.lock().await;
        receiver
            .take()
            .ok_or("No pending OAuth flow. Call start_oauth_flow first.".to_string())?
    };

    // Wait for callback (with timeout)
    let code = match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
        Ok(Ok(Ok(code))) => code,
        Ok(Ok(Err(e))) => return Err(e),
        Ok(Err(_)) => return Err("OAuth callback channel closed".to_string()),
        Err(_) => return Err("OAuth timeout - no callback received within 5 minutes".to_string()),
    };

    log::info!("[Auth Command] Received authorization code, exchanging for token");

    // Load OAuth config
    let client_id = std::env::var("ANILIST_CLIENT_ID")
        .map_err(|_| "ANILIST_CLIENT_ID not found".to_string())?;
    let client_secret = std::env::var("ANILIST_CLIENT_SECRET")
        .map_err(|_| "ANILIST_CLIENT_SECRET not found".to_string())?;

    let oauth_config = OAuthConfig {
        client_id,
        client_secret,
    };

    // Get the redirect URI (reconstruct from the port we used)
    let redirect_uri = "http://localhost:57575/auth/callback"; // Use standard port for redirect URI

    // Exchange code for token
    let token_response = exchange_code_for_token(&oauth_config, &code, redirect_uri).await?;

    // Save token to config using ConfigLoader (with encryption)
    let config_loader =
        config::ConfigLoader::new().map_err(|e| format!("Failed to load config: {}", e))?;
    config_loader
        .set_anilist_token(&token_response.access_token)
        .map_err(|e| format!("Failed to save token: {}", e))?;

    log::info!("[Auth Command] Token saved to config (encrypted)");

    // Update AniList service with new token
    anilist_service
        .update_token(Some(token_response.access_token.clone()))
        .await
        .map_err(|e| format!("Failed to update AniList service: {}", e))?;

    log::info!("[Auth Command] OAuth flow completed successfully");

    Ok(token_response.access_token)
}

/// Check if user is authenticated by fetching user profile
#[tauri::command]
pub async fn check_auth_status(
    anilist_service: State<'_, Arc<AniListService>>,
) -> Result<bool, String> {
    log::info!("[Auth Command] Checking authentication status");

    // Try to fetch current user
    match anilist_service.get_current_user().await {
        Ok(_) => {
            log::info!("[Auth Command] User is authenticated");
            Ok(true)
        }
        Err(e) => {
            log::info!("[Auth Command] User is not authenticated: {}", e);
            Ok(false)
        }
    }
}

/// Logout - clear stored token
#[tauri::command]
pub async fn logout(anilist_service: State<'_, Arc<AniListService>>) -> Result<(), String> {
    log::info!("[Auth Command] Logging out");

    // Clear token from config using ConfigLoader
    let config_loader =
        config::ConfigLoader::new().map_err(|e| format!("Failed to load config: {}", e))?;
    config_loader
        .clear_anilist_token()
        .map_err(|e| format!("Failed to clear token: {}", e))?;

    // Clear token from AniList service
    anilist_service
        .update_token(None)
        .await
        .map_err(|e| format!("Failed to update AniList service: {}", e))?;

    log::info!("[Auth Command] Logout successful");
    Ok(())
}
