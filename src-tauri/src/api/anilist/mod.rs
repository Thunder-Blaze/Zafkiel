use anilist_moe::{client::AniListClient, errors::AniListError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Wrapper for AniListClient that handles token management
/// Maintains a single AniListClient instance in app state
pub struct AniListService {
    client: Arc<RwLock<AniListClient>>,
}

impl AniListService {
    /// Create a new AniList service with optional token
    pub fn new(token: Option<String>) -> Self {
        let client = if let Some(t) = token {
            AniListClient::with_token(&t)
        } else {
            AniListClient::new()
        };

        Self {
            client: Arc::new(RwLock::new(client)),
        }
    }

    /// Update the client token (async-safe)
    /// Creates a new client instance with the new token
    pub async fn update_token(&self, token: Option<String>) -> Result<(), String> {
        log::info!("[AniListService] Updating token");
        let mut client = self.client.write().await;
        if let Some(t) = token {
            client.set_token(&t);
        } else {
            client.clear_token();
        };
        log::info!("[AniListService] Token updated successfully");
        Ok(())
    }

    /// Get a reference to the client for making requests
    /// Returns a read guard to prevent cloning
    pub async fn client(&self) -> tokio::sync::RwLockReadGuard<'_, AniListClient> {
        self.client.read().await
    }
}

/// Response wrapper for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct AniListResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> AniListResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

impl<T> From<Result<T, AniListError>> for AniListResponse<T> {
    fn from(result: Result<T, AniListError>) -> Self {
        match result {
            Ok(data) => Self::success(data),
            Err(e) => Self::error(format!("{:?}", e)),
        }
    }
}
