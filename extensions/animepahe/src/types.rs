use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Types mirroring the Zafkiel extension interface spec
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeDetails {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u32>,
    /// The session slug used in subsequent API calls
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: String,
    pub number: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_date: Option<String>,
    /// AnimePahe session slug (same as id here)
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub data: Vec<T>,
    pub current_page: u32,
    pub last_page: u32,
    pub total: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSource {
    /// Kwik URL used as the opaque source ID
    pub id: String,
    pub label: String,
    pub resolution: String,
    pub fansub: String,
    pub audio: String,
    pub requires_resolution: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedStream {
    pub url: String,
    pub r#type: String,
    pub headers: std::collections::HashMap<String, String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal AnimePahe API wire types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PaheSearchResponse {
    pub data: Option<Vec<PaheSearchItem>>,
}

#[derive(Debug, Deserialize)]
pub struct PaheSearchItem {
    pub session: String,
    pub title: String,
    pub poster: Option<String>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub status: Option<String>,
    pub year: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PaheEpisodesResponse {
    pub data: Option<Vec<PaheEpisodeItem>>,
    pub current_page: u32,
    pub last_page: u32,
    pub total: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct PaheEpisodeItem {
    pub session: String,
    pub episode: f32,
    pub snapshot: Option<String>,
    pub created_at: Option<String>,
}
