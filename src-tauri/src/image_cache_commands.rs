use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tokio::fs as async_fs;
use reqwest;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    pub total_images: u64,
    pub total_size: u64,
    pub oldest_image: u64,
}

/// Download an image and save it to the cache directory
#[tauri::command]
pub async fn download_image(
    app_handle: AppHandle,
    url: String,
    local_path: String,
    quality: Option<String>,
) -> Result<bool, String> {
    // Get app data directory
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache");
    let full_path = cache_dir.join(&local_path);
    
    // Create directory if it doesn't exist
    if let Some(parent) = full_path.parent() {
        async_fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }
    
    // Download the image
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to download image: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;
    
    // Save to file
    async_fs::write(&full_path, &bytes)
        .await
        .map_err(|e| format!("Failed to save image: {}", e))?;
    
    log::info!("Downloaded and cached image: {} -> {}", url, full_path.display());
    Ok(true)
}

/// Check if a file exists in the cache
#[tauri::command]
pub async fn file_exists(app_handle: AppHandle, path: String) -> Result<bool, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache");
    let full_path = cache_dir.join(&path);
    
    Ok(full_path.exists())
}

/// Delete a cached file
#[tauri::command]
pub async fn delete_file(app_handle: AppHandle, path: String) -> Result<bool, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache");
    let full_path = cache_dir.join(&path);
    
    if full_path.exists() {
        async_fs::remove_file(&full_path)
            .await
            .map_err(|e| format!("Failed to delete file: {}", e))?;
        log::info!("Deleted cached file: {}", full_path.display());
    }
    
    Ok(true)
}

/// Get cache statistics
#[tauri::command]
pub async fn get_cache_stats(app_handle: AppHandle) -> Result<CacheStats, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache").join("images");
    
    if !cache_dir.exists() {
        return Ok(CacheStats {
            total_images: 0,
            total_size: 0,
            oldest_image: 0,
        });
    }
    
    let mut total_images = 0;
    let mut total_size = 0;
    let mut oldest_image = u64::MAX;
    
    fn scan_directory(dir: &Path, stats: &mut (u64, u64, u64)) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let metadata = entry.metadata()?;
                stats.0 += 1; // total_images
                stats.1 += metadata.len(); // total_size
                
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        let timestamp = duration.as_secs();
                        if timestamp < stats.2 {
                            stats.2 = timestamp;
                        }
                    }
                }
            } else if path.is_dir() {
                scan_directory(&path, stats)?;
            }
        }
        Ok(())
    }
    
    let mut stats = (total_images, total_size, oldest_image);
    scan_directory(&cache_dir, &mut stats).map_err(|e| format!("Failed to scan cache directory: {}", e))?;
    
    Ok(CacheStats {
        total_images: stats.0,
        total_size: stats.1,
        oldest_image: if stats.2 == u64::MAX { 0 } else { stats.2 },
    })
}

/// Cleanup old cached images
#[tauri::command]
pub async fn cleanup_image_cache(app_handle: AppHandle, max_age_days: u64) -> Result<u64, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache").join("images");
    
    if !cache_dir.exists() {
        return Ok(0);
    }
    
    let cutoff_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() - (max_age_days * 24 * 60 * 60);
    
    let mut deleted_count = 0;
    
    fn cleanup_directory(dir: &Path, cutoff: u64, count: &mut u64) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let metadata = entry.metadata()?;
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                        if duration.as_secs() < cutoff {
                            fs::remove_file(&path)?;
                            *count += 1;
                            log::info!("Cleaned up old cached image: {}", path.display());
                        }
                    }
                }
            } else if path.is_dir() {
                cleanup_directory(&path, cutoff, count)?;
                
                // Remove empty directories
                if fs::read_dir(&path)?.next().is_none() {
                    fs::remove_dir(&path)?;
                }
            }
        }
        Ok(())
    }
    
    cleanup_directory(&cache_dir, cutoff_time, &mut deleted_count)
        .map_err(|e| format!("Failed to cleanup cache: {}", e))?;
    
    log::info!("Cleaned up {} old cached images", deleted_count);
    Ok(deleted_count)
}

/// Get the full path to a cached file
#[tauri::command]
pub async fn get_cached_file_path(app_handle: AppHandle, relative_path: String) -> Result<String, String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    
    let cache_dir = app_data_dir.join("cache");
    let full_path = cache_dir.join(&relative_path);
    
    Ok(full_path.to_string_lossy().to_string())
}