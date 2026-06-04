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
use std::{
    collections::HashMap,
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::command;
use tauri::{Emitter, Manager};

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
pub struct ExtensionSetting {
    pub id: String,
    pub label: String,
    pub r#type: String,
    pub default: serde_json::Value,
    pub options: Option<Vec<serde_json::Value>>,
    pub description: Option<String>,
}
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
    #[serde(default)]
    pub settings: Option<Vec<ExtensionSetting>>,
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
    let content = serde_json::to_string_pretty(entries)
        .map_err(|e| format!("Failed to serialize index: {e}"))?;
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
        let stripped = raw_name.trim_start_matches('/').trim_start_matches("../");
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
        return Err(format!("manifest.json not found in {}", dir.display()));
    }
    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest.json: {e}"))?;
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
pub async fn install_extension(
    id: String,
    download_url: String,
) -> Result<ExtensionIndexEntry, String> {
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
        fs::remove_dir_all(&ext_dir)
            .map_err(|e| format!("Failed to clean old extension dir: {e}"))?;
        fs::create_dir_all(&ext_dir)
            .map_err(|e| format!("Failed to recreate extension dir: {e}"))?;
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

    log::info!(
        "[Extensions] Installed: {} v{}",
        manifest.name,
        manifest.version
    );
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

    unzip_bundle(&bytes, &ext_dir).map_err(|e| format!("Failed to re-extract bundle: {e}"))?;

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
        log::warn!(
            "[Extensions] uninstall_extension: '{}' was not in index",
            id
        );
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
    fs::create_dir_all(&ext_dir).map_err(|e| format!("Failed to create extension dir: {e}"))?;

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
pub fn install_extension_from_local(
    id: String,
    bundle_path: String,
) -> Result<ExtensionIndexEntry, String> {
    let bytes = fs::read(&bundle_path)
        .map_err(|e| format!("Failed to read local bundle '{}': {e}", bundle_path))?;

    log::info!(
        "[Extensions] Local install of '{}' from '{}' ({} bytes)",
        id,
        bundle_path,
        bytes.len()
    );

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

    unzip_bundle(&bytes, &ext_dir).map_err(|e| format!("Failed to extract bundle: {e}"))?;

    if let Some(storage) = storage_backup {
        let _ = fs::write(ext_dir.join("storage.json"), storage);
    }

    let manifest = read_manifest_from_dir(&ext_dir)?;

    let entry_path = ext_dir.join(&manifest.entry);
    if !entry_path.exists() {
        return Err(format!(
            "Entry file '{}' not found after extraction",
            manifest.entry
        ));
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

    log::info!(
        "[Extensions] Local install done: {} v{}",
        manifest.name,
        manifest.version
    );
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
///
/// A one-shot local TCP server is spawned on a random port. The injected script
/// POSTs a "ping" when the user presses Done (or closes the window). The server
/// then reads ALL cookies — including HttpOnly ones — directly from WebKit's
/// native cookie store via `read_webview_cookies`.
#[command]
pub async fn open_extension_auth_webview(
    app: tauri::AppHandle,
    window_label: String,
    url: String,
    title: String,
) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    if let Some(existing) = app.get_webview_window(&window_label) {
        let _ = existing.close();
    }

    let parsed_url: url::Url = url
        .parse()
        .map_err(|e: url::ParseError| format!("Invalid URL '{url}': {e}"))?;

    // Bind on an OS-assigned port so we don't collide with other windows.
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Failed to bind cookie-collect server: {e}"))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("Cannot get local port: {e}"))?
        .port();

    // Background task: wait for the "Done" ping, then read cookies natively.
    let app2 = app.clone();
    let label2 = window_label.clone();
    let url_for_cookies = url.clone();
    tokio::spawn(async move {
        // Accept the signal POST – body is irrelevant, we just need the ping.
        if let Ok((mut stream, _)) = listener.accept().await {
            let _ = stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
                .await;
            drop(stream);
        }

        // Brief delay so WebKit flushes any in-flight Set-Cookie headers
        // that arrived just before the user pressed "Done".
        tokio::time::sleep(std::time::Duration::from_millis(400)).await;

        // Read cookies from WebKit's native store (includes HttpOnly).
        let cookies = read_webview_cookies(&app2, &label2, &url_for_cookies).await;

        log::info!(
            "[Extensions] '{label2}' cookies collected ({} chars): {}",
            cookies.len(),
            &cookies[..cookies.len().min(200)]
        );

        let event = format!("{label2}-cookies-ready");
        let _ = app2.emit(&event, cookies);

        if let Some(win) = app2.get_webview_window(&label2) {
            let _ = win.close();
        }
    });

    // Injected into every page load. The script only sends a *ping* to the
    // local server — the actual cookies are read natively in Rust, so HttpOnly
    // cookies are included even though JS cannot see them.
    let cookie_script = format!(
        r#"(function(){{
  'use strict';
  var PORT={port};
  var sent=false;

  function ping(){{
    if(sent)return;
    sent=true;
    if(navigator.sendBeacon){{
      navigator.sendBeacon('http://127.0.0.1:'+PORT+'/collect','ping');
    }}
    try{{
      fetch('http://127.0.0.1:'+PORT+'/collect',{{
        method:'POST',mode:'no-cors',keepalive:true,
        headers:{{'Content-Type':'text/plain'}},body:'ping'
      }}).catch(function(){{}});
    }}catch(e){{}}
  }}

  window.addEventListener('beforeunload',ping);
  window.addEventListener('unload',ping);

  function addButton(){{
    if(!document.body||document.getElementById('__zafkiel_done__'))return;
    var btn=document.createElement('div');
    btn.id='__zafkiel_done__';
    btn.textContent='\u2714 Done \u2014 Collect Cookies';
    btn.style.cssText='position:fixed;bottom:24px;right:24px;z-index:2147483647;'+
      'background:#22c55e;color:#fff;padding:10px 20px;border-radius:10px;'+
      'cursor:pointer;font:bold 14px/1.4 sans-serif;box-shadow:0 4px 14px rgba(0,0,0,.55);'+
      'user-select:none;';
    btn.addEventListener('click',function(){{
      btn.textContent='\u23f3 Collecting\u2026';
      btn.style.background='#2563eb';
      ping();
    }});
    document.body.appendChild(btn);
  }}

  if(document.readyState==='loading'){{
    document.addEventListener('DOMContentLoaded',addButton);
  }}else{{
    addButton();
  }}
  var _obs=new MutationObserver(function(){{addButton();}});
  _obs.observe(document.documentElement,{{childList:true,subtree:false}});
}})();"#,
        port = port
    );

    WebviewWindowBuilder::new(&app, &window_label, WebviewUrl::External(parsed_url))
        .title(&title)
        .inner_size(960.0, 720.0)
        .resizable(true)
        .decorations(true)
        .visible(true)
        .focused(true)
        .initialization_script(&cookie_script)
        .build()
        .map_err(|e| format!("Failed to open auth window '{window_label}': {e}"))?;

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Native cookie reading (includes HttpOnly cookies – JS cannot see these)
// ─────────────────────────────────────────────────────────────────────────────

/// Reads **all** cookies for `url` from the named webview's native cookie store,
/// including `HttpOnly` and `Secure` cookies that `document.cookie` never exposes.
async fn read_webview_cookies(app: &tauri::AppHandle, window_label: &str, url: &str) -> String {
    let Some(win) = app.get_webview_window(window_label) else {
        log::warn!("[Extensions] read_webview_cookies: window '{window_label}' not found");
        return String::new();
    };

    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
    let url = url.to_owned();

    if win
        .with_webview({
            let tx = tx.clone();
            move |wv| platform_collect_cookies(wv, url, tx)
        })
        .is_err()
    {
        log::warn!("[Extensions] with_webview failed for '{window_label}'");
        return String::new();
    }

    rx.await.unwrap_or_default()
}

type CookieTx = std::sync::Arc<std::sync::Mutex<Option<tokio::sync::oneshot::Sender<String>>>>;

fn send_cookies(tx: &CookieTx, cookies: String) {
    if let Ok(mut guard) = tx.lock() {
        if let Some(sender) = guard.take() {
            let _ = sender.send(cookies);
        }
    }
}

/// Linux: webkit2gtk CookieManager — reads all cookies including HttpOnly from
/// the native WebKit store. Methods on `soup3::Cookie` take `&mut self` so we
/// consume the Vec with `into_iter()` to get owned, mutable cookies.
#[cfg(target_os = "linux")]
fn platform_collect_cookies(wv: tauri::webview::PlatformWebview, url: String, tx: CookieTx) {
    use webkit2gtk::{CookieManagerExt, WebContextExt, WebViewExt};

    let webview = wv.inner();

    let Some(ctx) = webview.web_context() else {
        log::warn!("[Extensions] webkit: no web context");
        send_cookies(&tx, String::new());
        return;
    };
    let Some(cm) = ctx.cookie_manager() else {
        log::warn!("[Extensions] webkit: no cookie manager");
        send_cookies(&tx, String::new());
        return;
    };

    // Clone url_log for use inside the move closure; `url` is borrowed by
    // `cm.cookies()` as a `&str` argument and the borrow ends after the call.
    let url_log = url.clone();
    cm.cookies(&url, gio::Cancellable::NONE, move |result| {
        let cookie_str = match result {
            Ok(cookies) => {
                log::debug!(
                    "[Extensions] webkit got {} cookies for {}",
                    cookies.len(),
                    url_log
                );
                cookies
                    .into_iter()
                    .filter_map(|mut c| {
                        let name = c.name()?;
                        let value = c.value().unwrap_or_default();
                        if name.is_empty() {
                            None
                        } else {
                            Some(format!("{name}={value}"))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("; ")
            }
            Err(e) => {
                log::warn!("[Extensions] get_cookies error: {e}");
                String::new()
            }
        };
        send_cookies(&tx, cookie_str);
    });
}

/// macOS: WKHTTPCookieStore via objc — includes HttpOnly cookies.
/// `getAllCookies:` fires an ObjC block which is bridged to our channel.
#[cfg(target_os = "macos")]
fn platform_collect_cookies(wv: tauri::webview::PlatformWebview, _url: String, tx: CookieTx) {
    use objc::{msg_send, runtime::Object, sel, sel_impl};
    use std::ffi::CStr;

    unsafe {
        let wk_webview = wv.inner() as *mut Object;
        let config: *mut Object = msg_send![wk_webview, configuration];
        let data_store: *mut Object = msg_send![config, websiteDataStore];
        let cookie_store: *mut Object = msg_send![data_store, httpCookieStore];

        let tx_inner = tx.clone();
        let block = block::ConcreteBlock::new(move |cookies: *mut Object| {
            let count: usize = msg_send![cookies, count];
            let mut parts = Vec::with_capacity(count);
            for i in 0..count {
                let c: *mut Object = msg_send![cookies, objectAtIndex: i];
                let name_ptr: *const std::os::raw::c_char = msg_send![c, name];
                let val_ptr: *const std::os::raw::c_char = msg_send![c, value];
                if !name_ptr.is_null() && !val_ptr.is_null() {
                    let name = CStr::from_ptr(name_ptr).to_string_lossy().into_owned();
                    let val = CStr::from_ptr(val_ptr).to_string_lossy().into_owned();
                    if !name.is_empty() {
                        parts.push(format!("{name}={val}"));
                    }
                }
            }
            send_cookies(&tx_inner, parts.join("; "));
        });
        let block = block.copy();
        let _: () = msg_send![cookie_store, getAllCookies: &*block];
    }
}

/// Windows: WebView2 CookieManager via ICoreWebView2CookieManager — includes HttpOnly.
/// `GetCookies` is async/COM-callback based; we bridge it to our tokio oneshot
/// via a Mutex-guarded sender that fires from the completion handler closure.
#[cfg(target_os = "windows")]
fn platform_collect_cookies(wv: tauri::webview::PlatformWebview, url: String, tx: CookieTx) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_2, ICoreWebView2Cookie, ICoreWebView2CookieList, ICoreWebView2CookieManager,
        ICoreWebView2GetCookiesCompletedHandler, ICoreWebView2GetCookiesCompletedHandler_Impl,
    };
    use windows::core::{HSTRING, Interface, PWSTR, implement};

    #[implement(ICoreWebView2GetCookiesCompletedHandler)]
    struct Handler {
        tx: CookieTx,
    }

    impl ICoreWebView2GetCookiesCompletedHandler_Impl for Handler_Impl {
        fn Invoke(
            &self,
            error_code: windows_core::HRESULT,
            cookie_list: windows_core::Ref<'_, ICoreWebView2CookieList>,
        ) -> Result<(), windows_core::Error> {
            let cookies = if error_code.is_ok() {
                cookie_list
                    .as_ref()
                    .map(|list| unsafe {
                        let mut count = 0u32;
                        let _ = list.Count(&mut count);
                        (0..count)
                            .filter_map(|i| {
                                let cookie: ICoreWebView2Cookie = list.GetValueAtIndex(i).ok()?;
                                let mut name = PWSTR::null();
                                let mut value = PWSTR::null();
                                let _ = cookie.Name(&mut name);
                                let _ = cookie.Value(&mut value);
                                let n = name.to_string().ok().filter(|s| !s.is_empty())?;
                                let v = value.to_string().unwrap_or_default();
                                Some(format!("{n}={v}"))
                            })
                            .collect::<Vec<_>>()
                            .join("; ")
                    })
                    .unwrap_or_default()
            } else {
                log::warn!("[Extensions] WebView2 GetCookies error: {error_code:?}");
                String::new()
            };
            send_cookies(&self.tx, cookies);
            Ok(())
        }
    }

    unsafe {
        let controller = wv.controller();
        let core = match controller.CoreWebView2() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("[Extensions] webview2 CoreWebView2: {e}");
                send_cookies(&tx, String::new());
                return;
            }
        };
        let core2: ICoreWebView2_2 = match core.cast() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("[Extensions] webview2 cast to ICoreWebView2_2: {e}");
                send_cookies(&tx, String::new());
                return;
            }
        };
        let mgr: ICoreWebView2CookieManager = match core2.CookieManager() {
            Ok(v) => v,
            Err(e) => {
                log::warn!("[Extensions] webview2 CookieManager: {e}");
                send_cookies(&tx, String::new());
                return;
            }
        };

        let uri = HSTRING::from(url.as_str());
        let handler: ICoreWebView2GetCookiesCompletedHandler = Handler { tx }.into();
        if let Err(e) = mgr.GetCookies(&uri, &handler) {
            log::warn!("[Extensions] webview2 GetCookies call failed: {e}");
        }
    }
}

/// Fallback for any other platform.
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn platform_collect_cookies(_wv: tauri::webview::PlatformWebview, _url: String, tx: CookieTx) {
    send_cookies(&tx, String::new());
}

// ─────────────────────────────────────────────────────────────────────────────
// CDN cookie warm-up (Cloudflare JS challenge)
// ─────────────────────────────────────────────────────────────────────────────

/// Open a hidden webview to a Kwik.si embed URL and wait for Cloudflare's JS
/// challenge to auto-execute on the CDN domain, then return those cookies.
///
/// This mirrors exactly what Zenshin/Electron does with `webSecurity: false`:
/// the real browser visits kwik.si, CF challenge runs, the `cf_clearance`
/// cookie is set on `owocdn.top`, and subsequent CDN requests succeed.
///
/// * `kwik_url`  — `https://kwik.si/e/{embed_id}` (the source button src)
/// * `cdn_url`   — origin of the m3u8 URL, e.g. `https://vault-99.owocdn.top`
///
/// Returns the collected cookie string for `cdn_url` (empty string on failure).
#[command]
pub async fn collect_cdn_cookies_for_kwik(
    app: tauri::AppHandle,
    kwik_url: String,
    cdn_url: String,
) -> Result<String, String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    const LABEL: &str = "kwik-cdn-warm";

    // Close any leftover window from a previous attempt
    if let Some(existing) = app.get_webview_window(LABEL) {
        let _ = existing.close();
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    let parsed = kwik_url
        .parse::<url::Url>()
        .map_err(|e| format!("Invalid kwik URL: {e}"))?;

    log::info!("[Extensions] kwik CDN warm-up: opening {kwik_url}");

    // Use a small but visible off-screen window — hidden windows may not execute
    // JS on all platforms (WebKit can skip rendering for invisible views).
    let _win = WebviewWindowBuilder::new(&app, LABEL, WebviewUrl::External(parsed))
        .title("Loading…")
        .inner_size(400.0, 300.0)
        .position(-8000.0, -8000.0) // off all normal screen coordinates
        .decorations(false)
        .resizable(false)
        .build()
        .map_err(|e| format!("Failed to open kwik CDN warm-up webview: {e}"))?;

    // Poll every 500 ms for the cf_clearance cookie on the CDN domain.
    // CF JS challenges typically resolve in under 3 seconds.
    // We wait up to 20 seconds in case the challenge takes longer.
    let mut cdn_cookies = String::new();
    for attempt in 0u32..40 {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        cdn_cookies = read_webview_cookies(&app, LABEL, &cdn_url).await;

        // Consider success either when cf_clearance is present OR after 5 s
        // if any cookie at all appeared (CDN may use a different cookie name).
        let has_clearance = cdn_cookies.contains("cf_clearance");
        let has_any_cookie = !cdn_cookies.is_empty() && attempt >= 10;
        if has_clearance || has_any_cookie {
            log::info!(
                "[Extensions] CDN cookies ready after {}ms ({} chars): {}",
                (attempt + 1) * 500,
                cdn_cookies.len(),
                &cdn_cookies[..cdn_cookies.len().min(200)]
            );
            break;
        }
    }

    if cdn_cookies.is_empty() {
        log::warn!("[Extensions] No CDN cookies collected for {cdn_url} after 20 s");
    }

    // Close the warm-up window
    if let Some(w) = app.get_webview_window(LABEL) {
        let _ = w.close();
    }

    Ok(cdn_cookies)
}
