/**
 * Tauri Commands for OAuth Authentication
 */
use crate::api::anilist::AniListService;
use crate::auth::anilist::{
    AuthState, get_authorization_url, start_callback_server,
};
use crate::config;
use crate::constants::{ANILIST_CLIENT_ID, ANILIST_REDIRECT_PORT};
use std::sync::Arc;
use tauri::{AppHandle, State};

type ConfigState = Arc<config::ConfigLoader>;

/// Initialize OAuth flow
/// Returns the authorization URL and callback port
#[tauri::command]
pub async fn start_oauth_flow(
    _app: AppHandle,
    auth_state: State<'_, AuthState>,
) -> Result<(String, u16), String> {
    log::info!("[Auth Command] Starting OAuth flow");

    if ANILIST_CLIENT_ID.is_empty() {
        return Err("ANILIST_CLIENT_ID constant is empty".to_string());
    }

    // Create a channel to receive the access token from callback.
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

    // AniList requires exact redirect URI matching with app settings.
    // Keep callback port fixed to avoid redirect_uri mismatch errors.
    let port = ANILIST_REDIRECT_PORT;
    log::info!("[Auth Command] Using fixed port {} for callback", port);

    // Start callback server
    start_callback_server(port, auth_state.inner().clone()).await?;

    // Generate authorization URL
    let auth_url = get_authorization_url(ANILIST_CLIENT_ID);
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

/// Wait for OAuth callback and persist token (implicit grant)
#[tauri::command]
pub async fn wait_for_oauth_callback(
    auth_state: State<'_, AuthState>,
    anilist_service: State<'_, Arc<AniListService>>,
    config_loader: State<'_, ConfigState>,
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
    let token = match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
        Ok(Ok(Ok(token))) => token,
        Ok(Ok(Err(e))) => return Err(e),
        Ok(Err(_)) => return Err("OAuth callback channel closed".to_string()),
        Err(_) => return Err("OAuth timeout - no callback received within 5 minutes".to_string()),
    };

    log::info!("[Auth Command] Received implicit access token");

    // Save token to the managed config (writes to disk and updates in-memory state)
    config_loader
        .set_anilist_token(&token)
        .map_err(|e| format!("Failed to save token: {}", e))?;

    log::info!("[Auth Command] Token saved to config (encrypted)");

    // Update AniList service with new token
    anilist_service
        .update_token(Some(token.clone()))
        .await
        .map_err(|e| format!("Failed to update AniList service: {}", e))?;

    log::info!("[Auth Command] OAuth flow completed successfully");

    Ok(token)
}

/// Check if user is authenticated by fetching user profile
#[tauri::command]
pub async fn check_auth_status(
    anilist_service: State<'_, Arc<AniListService>>,
) -> Result<bool, String> {
    log::info!("[Auth Command] Checking authentication status");

    // Try to fetch current user
    let client = anilist_service.client().await;
    match client.user().get_current_user().await {
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
pub async fn logout(
    anilist_service: State<'_, Arc<AniListService>>,
    config_loader: State<'_, ConfigState>,
) -> Result<(), String> {
    log::info!("[Auth Command] Logging out");

    // Clear token from the managed config (writes to disk and updates in-memory state)
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
