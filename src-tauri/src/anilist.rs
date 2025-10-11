use anilist_moe::{
    client::AniListClient,
    enums::media::MediaSeason,
    errors::AniListError,
    objects::{media::Media, responses::ViewerUserData, user::User},
};
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
        *client = if let Some(t) = token {
            AniListClient::with_token(&t)
        } else {
            AniListClient::new()
        };
        log::info!("[AniListService] Token updated successfully");
        Ok(())
    }

    /// Get a reference to the client for making requests
    /// Returns a read guard to prevent cloning
    async fn client(&self) -> tokio::sync::RwLockReadGuard<'_, AniListClient> {
        self.client.read().await
    }

    /// Search for anime
    pub async fn search_anime(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        log::info!(
            "Searching anime: query='{}', page={:?}, per_page={:?}",
            query,
            page,
            per_page
        );
        let client = self.client().await;
        let response = client.anime().search_anime(query, page, per_page).await?;
        log::info!(
            "Found {} anime matching '{}'",
            response.data.media.len(),
            query
        );
        Ok(response.data.media)
    }

    /// Get anime by ID
    pub async fn get_anime_by_id(&self, id: i32) -> Result<Media, AniListError> {
        let client = self.client().await;
        let response = client.anime().get_anime_by_id(id).await?;
        Ok(response)
    }

    /// Get trending anime
    pub async fn get_trending_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        log::info!(
            "Fetching trending anime: page={:?}, per_page={:?}",
            page,
            per_page
        );
        let client = self.client().await;
        let response = client.anime().get_trending_anime(page, per_page).await?;
        log::info!(
            "Successfully fetched {} trending anime",
            response.data.media.len()
        );
        Ok(response.data.media)
    }

    /// Get popular anime
    pub async fn get_popular_anime(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        let client = self.client().await;
        let response = client.anime().get_popular_anime(page, per_page).await?;
        Ok(response.data.media)
    }

    /// Get seasonal anime (note: anilist_moe doesn't have direct seasonal support)
    /// This is a placeholder - in production, you'd filter get_airing_anime by season
    pub async fn get_seasonal_anime(
        &self,
        _season: MediaSeason,
        _year: i32,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        // Fallback to popular anime since seasonal isn't directly supported
        let client = self.client().await;
        let response = client.anime().get_popular_anime(page, per_page).await?;
        Ok(response.data.media)
    }

    /// Search for manga
    pub async fn search_manga(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        let client = self.client().await;
        let response = client.manga().search_manga(query, page, per_page).await?;
        Ok(response.data.media)
    }

    /// Get manga by ID
    pub async fn get_manga_by_id(&self, id: i32) -> Result<Media, AniListError> {
        let client = self.client().await;
        let response = client.manga().get_manga_by_id(id).await?;
        Ok(response)
    }

    /// Get trending manga
    pub async fn get_trending_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        let client = self.client().await;
        let response = client.manga().get_trending_manga(page, per_page).await?;
        Ok(response.data.media)
    }

    /// Get popular manga
    pub async fn get_popular_manga(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Media>, AniListError> {
        let client = self.client().await;
        let response = client.manga().get_popular_manga(page, per_page).await?;
        Ok(response.data.media)
    }

    /// Get current authenticated user
    pub async fn get_current_user(&self) -> Result<ViewerUserData, AniListError> {
        let client = self.client().await;
        let response = client.user().fetch_basic().await?;
        Ok(response)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<User, AniListError> {
        let client = self.client().await;
        let response = client.user().get_by_id(id).await?;
        Ok(response)
    }

    /// Get user by name
    pub async fn get_user_by_name(&self, name: &str) -> Result<User, AniListError> {
        let client = self.client().await;
        let response = client.user().get_by_name(name).await?;
        Ok(response)
    }

    /// Search users
    pub async fn search_users(
        &self,
        query: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<User>, AniListError> {
        let client = self.client().await;
        let response = client.user().search(query, page, per_page).await?;
        Ok(response.data.users)
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
