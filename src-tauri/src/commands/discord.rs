use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};
use std::sync::Mutex;
use tauri::State;

use crate::constants::DISCORD_APP_ID;

pub struct DiscordState {
    pub client: Mutex<Option<DiscordIpcClient>>,
}

impl DiscordState {
    pub fn new() -> Self {
        Self {
            client: Mutex::new(None),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordActivityArgs {
    pub state: String,
    pub details: String,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
}

#[tauri::command]
pub fn set_discord_activity(
    payload: DiscordActivityArgs,
    discord_state: State<'_, DiscordState>,
) -> Result<(), String> {
    let mut client_lock = discord_state.client.lock().map_err(|e| e.to_string())?;

    if client_lock.is_none() {
        // Create new client if it doesn't exist
        let mut new_client = DiscordIpcClient::new(DISCORD_APP_ID);
        if let Err(e) = new_client.connect() {
            log::warn!("[Discord RPC] Failed to connect: {}", e);
            // Discord might not be running, don't crash the app
            return Ok(());
        }
        *client_lock = Some(new_client);
    }

    if let Some(client) = client_lock.as_mut() {
        let mut act = activity::Activity::new()
            .state(&payload.state)
            .details(&payload.details)
            .activity_type(activity::ActivityType::Watching);

        let mut assets = activity::Assets::new();
        let mut has_assets = false;

        if let Some(li) = &payload.large_image {
            assets = assets.large_image(li);
            has_assets = true;
        }
        if let Some(lt) = &payload.large_text {
            assets = assets.large_text(lt);
            has_assets = true;
        }
        if let Some(si) = &payload.small_image {
            assets = assets.small_image(si);
            has_assets = true;
        }
        if let Some(st) = &payload.small_text {
            assets = assets.small_text(st);
            has_assets = true;
        }

        if has_assets {
            act = act.assets(assets);
        }

        let mut timestamps = activity::Timestamps::new();
        let mut has_timestamps = false;

        if let Some(ts) = payload.start_timestamp {
            timestamps = timestamps.start(ts);
            has_timestamps = true;
        }

        if let Some(ts) = payload.end_timestamp {
            timestamps = timestamps.end(ts);
            has_timestamps = true;
        }

        if has_timestamps {
            act = act.timestamps(timestamps);
        }

        if let Err(e) = client.set_activity(act) {
            log::warn!("[Discord RPC] Failed to set activity: {}", e);
            // Drop client so we try reconnecting next time
            *client_lock = None;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn clear_discord_activity(discord_state: State<'_, DiscordState>) -> Result<(), String> {
    let mut client_lock = discord_state.client.lock().map_err(|e| e.to_string())?;

    if let Some(client) = client_lock.as_mut() {
        if let Err(e) = client.clear_activity() {
            log::warn!("[Discord RPC] Failed to clear activity: {}", e);
        }
        let _ = client.close();
    }

    *client_lock = None;
    Ok(())
}
