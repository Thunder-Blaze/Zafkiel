/// Extension management commands for Zafkiel.
///
/// # Directory layout (all under `~/.config/zafkiel/`)
/// ```
/// extensions/
///   index.json                       ← Extension registry (persisted)
///   animepahe/
///     manifest.json                  ← Extension manifest
///     extension.js (or .wasm)        ← Extension code
///     .bundle.zext                   ← Original downloaded bundle (for reinstall)
///     storage.json                   ← Per-extension key-value store
///   ...
/// ```
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};
use std::{
    collections::HashMap,
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::command;

// ─────────────────────────────────────────────────────────────────────────────
// Shared types
// ─────────────────────────────────────────────────────────────────────────────

/// Entry persisted in `extensions/index.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionIndexEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
    /// "source" | "torrent" | "other"
    pub r#type: String,
    /// Absolute filesystem path to the extension's directory.
    pub path: String,
    /// Unix timestamp (seconds) when the extension was installed.
    pub installed_at: u64,
    /// Filename of the main entry-point inside the extension dir.
    pub entry: String,
    /// Optional minimum application version required.
    pub min_app_version: Option<String>,
}

/// `manifest.json` schema inside a `.zext` bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
    pub r#type: String,
    /// Entry-point file: "extension.js" or "extension.wasm"
    pub entry: String,
    pub min_app_version: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Path helpers
// ─────────────────────────────────────────────────────────────────────────────

fn get_extensions_dir() -> Result<PathBuf, String> {
    let dirs = ProjectDirs::from("", "", "zafkiel")
        .ok_or_else(|| "Failed to resolve platform config directory".to_string())?;
    let ext_dir = dirs.config_dir().join("extensions");
    fs::create_dir_all(&ext_dir)
        .map_err(|e| format!("Failed to create extensions directory: {e}"))?;
    Ok(ext_dir)
}

fn index_path() -> Result<PathBuf, String> {
    Ok(get_extensions_dir()?.join("index.json"))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ─────────────────────────────────────────────────────────────────────────────
// Index read / write
// ─────────────────────────────────────────────────────────────────────────────

fn read_index() -> Vec<ExtensionIndexEntry> {
    let path = match index_path() {
        Ok(p) => p,
        Err(_) => return vec![],
    };
    if !path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

fn write_index(entries: &[ExtensionIndexEntry]) -> Result<(), String> {
    let path = index_path()?;
    let content =
        serde_json::to_string_pretty(entries).map_err(|e| format!("Failed to serialize index: {e}"))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write index.json: {e}"))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Zip extraction
// ─────────────────────────────────────────────────────────────────────────────

fn unzip_bundle(bytes: &[u8], target_dir: &Path) -> Result<(), io::Error> {
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        // Sanitise path: strip leading "/" or ".." components to prevent path traversal.
        let raw_name = file.name().to_owned();
        let stripped = raw_name
            .trim_start_matches('/')
            .trim_start_matches("../");
        let outpath = target_dir.join(stripped);

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn read_manifest_from_dir(dir: &Path) -> Result<ExtensionManifest, String> {
    let manifest_path = dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!(
            "manifest.json not found in {}",
            dir.display()
        ));
    }
    let content =
        fs::read_to_string(&manifest_path).map_err(|e| format!("Failed to read manifest.json: {e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("Invalid manifest.json — {e}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri commands
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the full list of installed extensions from the index.
///
/// This is *fast* – it only reads `index.json`, no directory scanning.
#[command]
pub fn get_installed_extensions() -> Vec<ExtensionIndexEntry> {
    read_index()
}

/// Downloads a `.zext` bundle from `download_url`, extracts it into
/// `~/.config/zafkiel/extensions/{id}/`, and registers it in `index.json`.
///
/// The original bundle is preserved as `.bundle.zext` so the extension can
/// be reinstalled locally without a network call.
#[command]
pub async fn install_extension(id: String, download_url: String) -> Result<ExtensionIndexEntry, String> {
    // 1. Download
    let client = reqwest::Client::new();
    let bytes = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {e}"))?
        .error_for_status()
        .map_err(|e| format!("Server returned error: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Failed to read download body: {e}"))?;

    log::info!("[Extensions] Downloaded {} ({} bytes)", id, bytes.len());

    // 2. Create / clean extension directory
    let ext_dir = get_extensions_dir()?.join(&id);
    if ext_dir.exists() {
        // Remove old files but keep storage.json if it exists
        let storage_backup = if ext_dir.join("storage.json").exists() {
            Some(fs::read_to_string(ext_dir.join("storage.json")).unwrap_or_default())
        } else {
            None
        };
        fs::remove_dir_all(&ext_dir).map_err(|e| format!("Failed to clean old extension dir: {e}"))?;
        fs::create_dir_all(&ext_dir).map_err(|e| format!("Failed to recreate extension dir: {e}"))?;
        // Restore storage
        if let Some(storage) = storage_backup {
            let _ = fs::write(ext_dir.join("storage.json"), storage);
        }
    } else {
        fs::create_dir_all(&ext_dir).map_err(|e| format!("Failed to create extension dir: {e}"))?;
    }

    // 3. Save original bundle for future reinstalls
    let bundle_path = ext_dir.join(".bundle.zext");
    fs::write(&bundle_path, &bytes).map_err(|e| format!("Failed to save bundle: {e}"))?;

    // 4. Extract
    unzip_bundle(&bytes, &ext_dir)
        .map_err(|e| format!("Failed to extract extension bundle: {e}"))?;

    // 5. Validate manifest
    let manifest = read_manifest_from_dir(&ext_dir)
        .map_err(|e| format!("Extension installed but manifest is invalid — {e}"))?;

    // Ensure entry file exists
    let entry_path = ext_dir.join(&manifest.entry);
    if !entry_path.exists() {
        return Err(format!(
            "Extension entry file '{}' not found after extraction",
            manifest.entry
        ));
    }

    // 6. Build and persist index entry
    let entry = ExtensionIndexEntry {
        id: manifest.id.clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        author: manifest.author.clone(),
        description: manifest.description.clone(),
        r#type: manifest.r#type.clone(),
        path: ext_dir.to_string_lossy().to_string(),
        installed_at: now_secs(),
        entry: manifest.entry.clone(),
        min_app_version: manifest.min_app_version.clone(),
    };

    let mut entries = read_index();
    entries.retain(|e| e.id != id);
    entries.push(entry.clone());
    write_index(&entries)?;

    log::info!("[Extensions] Installed: {} v{}", manifest.name, manifest.version);
    Ok(entry)
}

/// Re-extracts the cached `.bundle.zext` without downloading again.
///
/// Useful to recover from corrupt installs.  Preserves `storage.json`.
#[command]
pub fn reinstall_extension(id: String) -> Result<ExtensionIndexEntry, String> {
    let ext_dir = get_extensions_dir()?.join(&id);
    let bundle_path = ext_dir.join(".bundle.zext");

    if !bundle_path.exists() {
        return Err(format!(
            "Local bundle not found for '{}'. Please download the extension again.",
            id
        ));
    }

    let bytes = fs::read(&bundle_path).map_err(|e| format!("Failed to read bundle: {e}"))?;

    // Preserve storage
    let storage_backup = if ext_dir.join("storage.json").exists() {
        Some(fs::read_to_string(ext_dir.join("storage.json")).unwrap_or_default())
    } else {
        None
    };

    // Remove everything except the bundle itself
    if let Ok(entries_iter) = fs::read_dir(&ext_dir) {
        for entry in entries_iter.flatten() {
            let p = entry.path();
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name != ".bundle.zext" {
                let _ = if p.is_dir() {
                    fs::remove_dir_all(&p)
                } else {
                    fs::remove_file(&p)
                };
            }
        }
    }

    unzip_bundle(&bytes, &ext_dir)
        .map_err(|e| format!("Failed to re-extract bundle: {e}"))?;

    // Restore storage
    if let Some(storage) = storage_backup {
        let _ = fs::write(ext_dir.join("storage.json"), storage);
    }

    let manifest = read_manifest_from_dir(&ext_dir)?;

    let mut index = read_index();
    if let Some(entry) = index.iter_mut().find(|e| e.id == id) {
        entry.version = manifest.version.clone();
        entry.name = manifest.name.clone();
        entry.entry = manifest.entry.clone();
        let result = entry.clone();
        write_index(&index)?;
        return Ok(result);
    }

    // Not in index — re-register
    let entry = ExtensionIndexEntry {
        id: manifest.id.clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        author: manifest.author.clone(),
        description: manifest.description.clone(),
        r#type: manifest.r#type.clone(),
        path: ext_dir.to_string_lossy().to_string(),
        installed_at: now_secs(),
        entry: manifest.entry.clone(),
        min_app_version: manifest.min_app_version.clone(),
    };
    index.push(entry.clone());
    write_index(&index)?;
    Ok(entry)
}

/// Removes an installed extension's directory and its index entry.
#[command]
pub fn uninstall_extension(id: String) -> Result<(), String> {
    let ext_dir = get_extensions_dir()?.join(&id);
    if ext_dir.exists() {
        fs::remove_dir_all(&ext_dir)
            .map_err(|e| format!("Failed to remove extension directory: {e}"))?;
    }
    let mut entries = read_index();
    let before = entries.len();
    entries.retain(|e| e.id != id);
    if entries.len() == before {
        log::warn!("[Extensions] uninstall_extension: '{}' was not in index", id);
    }
    write_index(&entries)?;
    log::info!("[Extensions] Uninstalled: {}", id);
    Ok(())
}

/// Reads the path of the entry-point file for a loaded extension.
///
/// Returns the absolute path so the frontend JS loader can do
/// `fetch('asset://...')` via the Tauri asset protocol.
#[command]
pub fn get_extension_entry_path(id: String) -> Result<String, String> {
    let ext_dir = get_extensions_dir()?.join(&id);
    let manifest = read_manifest_from_dir(&ext_dir)?;
    let entry_path = ext_dir.join(&manifest.entry);
    if !entry_path.exists() {
        return Err(format!(
            "Entry file '{}' is missing from extension '{}'.  Try reinstalling.",
            manifest.entry, id
        ));
    }
    Ok(entry_path.to_string_lossy().to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-extension key-value storage
// ─────────────────────────────────────────────────────────────────────────────

#[command]
pub fn ext_storage_get(ext_id: String, key: String) -> Result<Option<String>, String> {
    let storage_path = get_extensions_dir()?.join(&ext_id).join("storage.json");
    if !storage_path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(&storage_path)
        .map_err(|e| format!("Failed to read extension storage: {e}"))?;
    let map: HashMap<String, String> = serde_json::from_str(&content).unwrap_or_default();
    Ok(map.get(&key).cloned())
}

#[command]
pub fn ext_storage_set(ext_id: String, key: String, value: String) -> Result<(), String> {
    let ext_dir = get_extensions_dir()?.join(&ext_id);
    fs::create_dir_all(&ext_dir)
        .map_err(|e| format!("Failed to create extension dir: {e}"))?;

    let storage_path = ext_dir.join("storage.json");
    let mut map: HashMap<String, String> = if storage_path.exists() {
        let content = fs::read_to_string(&storage_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        HashMap::new()
    };

    map.insert(key, value);
    let content = serde_json::to_string_pretty(&map)
        .map_err(|e| format!("Failed to serialize storage: {e}"))?;
    fs::write(&storage_path, content)
        .map_err(|e| format!("Failed to write extension storage: {e}"))?;
    Ok(())
}

#[command]
pub fn ext_storage_delete(ext_id: String, key: String) -> Result<(), String> {
    let storage_path = get_extensions_dir()?.join(&ext_id).join("storage.json");
    if !storage_path.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&storage_path)
        .map_err(|e| format!("Failed to read extension storage: {e}"))?;
    let mut map: HashMap<String, String> = serde_json::from_str(&content).unwrap_or_default();
    map.remove(&key);
    let content = serde_json::to_string_pretty(&map)
        .map_err(|e| format!("Failed to serialize storage: {e}"))?;
    fs::write(&storage_path, content)
        .map_err(|e| format!("Failed to write extension storage: {e}"))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Local install (dev / testing)
// ─────────────────────────────────────────────────────────────────────────────

/// Installs a `.zext` bundle from a local filesystem path.
///
/// Identical to `install_extension` except it reads from disk instead of
/// downloading.  Use this during development to test local extension builds.
#[command]
pub fn install_extension_from_local(id: String, bundle_path: String) -> Result<ExtensionIndexEntry, String> {
    let bytes = fs::read(&bundle_path)
        .map_err(|e| format!("Failed to read local bundle '{}': {e}", bundle_path))?;

    log::info!("[Extensions] Local install of '{}' from '{}' ({} bytes)", id, bundle_path, bytes.len());

    let ext_dir = get_extensions_dir()?.join(&id);

    let storage_backup = if ext_dir.join("storage.json").exists() {
        Some(fs::read_to_string(ext_dir.join("storage.json")).unwrap_or_default())
    } else {
        None
    };

    if ext_dir.exists() {
        fs::remove_dir_all(&ext_dir).map_err(|e| format!("Failed to clean old dir: {e}"))?;
    }
    fs::create_dir_all(&ext_dir).map_err(|e| format!("Failed to create ext dir: {e}"))?;

    fs::write(ext_dir.join(".bundle.zext"), &bytes)
        .map_err(|e| format!("Failed to copy bundle: {e}"))?;

    // Record the original source path for debugging
    let _ = fs::write(ext_dir.join(".source_path"), &bundle_path);

    unzip_bundle(&bytes, &ext_dir)
        .map_err(|e| format!("Failed to extract bundle: {e}"))?;

    if let Some(storage) = storage_backup {
        let _ = fs::write(ext_dir.join("storage.json"), storage);
    }

    let manifest = read_manifest_from_dir(&ext_dir)?;

    let entry_path = ext_dir.join(&manifest.entry);
    if !entry_path.exists() {
        return Err(format!("Entry file '{}' not found after extraction", manifest.entry));
    }

    let entry = ExtensionIndexEntry {
        id: manifest.id.clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        author: manifest.author.clone(),
        description: manifest.description.clone(),
        r#type: manifest.r#type.clone(),
        path: ext_dir.to_string_lossy().to_string(),
        installed_at: now_secs(),
        entry: manifest.entry.clone(),
        min_app_version: manifest.min_app_version.clone(),
    };

    let mut entries = read_index();
    entries.retain(|e| e.id != id);
    entries.push(entry.clone());
    write_index(&entries)?;

    log::info!("[Extensions] Local install done: {} v{}", manifest.name, manifest.version);
    Ok(entry)
}

// ─────────────────────────────────────────────────────────────────────────────
// Generic extension auth webview
//
// Extensions that need a Cloudflare challenge (or any browser-based auth)
// call these from their JS bundle.  No extension-specific logic lives here;
// the overlay script and cookie event name are controlled by the caller.
// ─────────────────────────────────────────────────────────────────────────────

/// Open a named browser window pointing at `url` for extension auth flows.
/// If a window with `window_label` already exists it is closed first.
/// The injected overlay in the extension JS calls `collect_extension_cookies`
/// when the user has passed the challenge.
#[command]
pub async fn open_extension_auth_webview(
    app: tauri::AppHandle,
    window_label: String,
    url: String,
    title: String,
) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    if let Some(existing) = app.get_webview_window(&window_label) {
        let _ = existing.close();
    }

    let parsed_url: url::Url = url
        .parse()
        .map_err(|e: url::ParseError| format!("Invalid URL '{url}': {e}"))?;

    WebviewWindowBuilder::new(&app, &window_label, WebviewUrl::External(parsed_url))
        .title(&title)
        .inner_size(960.0, 720.0)
        .build()
        .map_err(|e| format!("Failed to open auth window '{window_label}': {e}"))?;

    Ok(())
}

/// Called from inside the auth webview (via the extension's injected script).
/// Emits `{window_label}-cookies-ready` to all windows so the extension JS
/// in the main window can pick up the cookies, then closes the auth window.
#[command]
pub fn collect_extension_cookies(
    app: tauri::AppHandle,
    window_label: String,
    cookies: String,
) -> Result<(), String> {
    let event = format!("{window_label}-cookies-ready");
    app.emit(&event, cookies.clone())
        .map_err(|e| format!("Failed to emit '{event}': {e}"))?;

    if let Some(win) = app.get_webview_window(&window_label) {
        let _ = win.close();
    }

    log::info!("[Extensions] '{window_label}' cookies collected ({} chars)", cookies.len());
    Ok(())
}
