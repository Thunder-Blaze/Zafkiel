use crate::commands::config::ConfigState;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use librqbit::{
    AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, Session,
    api::TorrentIdOrHash,
};
use std::process::Stdio;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State, command};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::io::ReaderStream;

use axum::extract::{Path, State as AxumState};
use lazy_static::lazy_static;
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tower_http::cors::{Any, CorsLayer};
use crate::database::Database;
use rusqlite::params;

#[derive(Serialize, Debug)]
pub struct TorrentState {
    pub id: usize,
    pub name: Option<String>,
    pub progress: f64,
    pub speed: f64,        // bytes per second
    pub upload_speed: f64, // bytes per second
    pub peers: usize,
    pub seeds: usize,
    pub state: String, // "downloading", "paused", "seeding", "error"
    pub total_size: u64,
    pub downloaded: u64,
    pub info_hash: String,
    pub anime_id: Option<i32>,
    pub episode_number: Option<f32>,
    pub anime_title: Option<String>,
    pub anime_cover: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamResponse {
    pub url: String,
    pub info_hash: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentFile {
    pub id: usize,
    pub name: String,
    pub size: u64,
    pub progress: f64, // Placeholder for now
}

struct GlobalServer {
    addr: SocketAddr,
}

lazy_static! {
    static ref SPEED_TRACKER: Mutex<HashMap<usize, (u64, u64, Instant)>> =
        Mutex::new(HashMap::new());
    static ref GLOBAL_SERVER: Mutex<Option<GlobalServer>> = Mutex::new(None);
}

#[command]
pub async fn get_stream_base_url(session: State<'_, Arc<Session>>) -> Result<String, String> {
    ensure_global_server(session).await
}

async fn ensure_global_server(session: State<'_, Arc<Session>>) -> Result<String, String> {
    {
        let guard = GLOBAL_SERVER.lock().unwrap();
        if let Some(server) = &*guard {
            return Ok(format!("http://{}", server.addr));
        }
    }

    log::info!("Starting Global Streaming Server...");
    let session_arc = session.inner().clone();

    // Try binding to port 3030, fallback to random
    let listener = match TcpListener::bind("127.0.0.1:3030").await {
        Ok(l) => {
            log::info!("Bound to fixed port 3030");
            l
        }
        Err(e) => {
            log::warn!("Port 3030 busy ({}), falling back to random port", e);
            TcpListener::bind("127.0.0.1:0")
                .await
                .map_err(|e| e.to_string())?
        }
    };

    let addr = listener.local_addr().map_err(|e| e.to_string())?;
    log::info!("Global Streaming Server listening on: {}", addr);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/torrents/:id/stream/:file_id", get(global_stream_handler))
        .route(
            "/torrents/:id/subtitles/:file_id/:track_index",
            get(subtitle_handler),
        )
        .route("/files/:filename", get(static_file_handler))
        .layer(cors)
        .with_state(session_arc);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    {
        let mut guard = GLOBAL_SERVER.lock().unwrap();
        *guard = Some(GlobalServer { addr });
    }

    Ok(format!("http://{}", addr))
}

#[command]
pub async fn get_torrent_files(
    session: State<'_, Arc<Session>>,
    magnet: String,
) -> Result<Vec<TorrentFile>, String> {
    let handle = add_torrent_internal(&session, &magnet, false).await?;

    // Wait for metadata
    for _ in 0..100 {
        if handle.metadata.load().is_some() {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let metadata = handle.metadata.load();
    let info = &metadata
        .as_ref()
        .ok_or("Timeout waiting for metadata")?
        .info;

    let mut files = Vec::new();

    if let Some(info_files) = &info.files {
        for (idx, f) in info_files.iter().enumerate() {
            let name = f
                .path
                .iter()
                .map(|p| String::from_utf8_lossy(p).to_string())
                .collect::<Vec<_>>()
                .join("/");

            files.push(TorrentFile {
                id: idx,
                name,
                size: f.length,
                progress: 0.0,
            });
        }
    } else {
        let len = info.length.ok_or("Single file torrent has no length")?;
        let name_bytes = info.name.clone().ok_or("Single file torrent has no name")?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();
        files.push(TorrentFile {
            id: 0,
            name,
            size: len,
            progress: handle.stats().progress_bytes as f64 / len as f64 * 100.0,
        });
    }

    Ok(files)
}

#[command]
pub async fn get_torrent_files_by_id(
    session: State<'_, Arc<Session>>,
    id: usize,
) -> Result<Vec<TorrentFile>, String> {
    let handle = session
        .get(TorrentIdOrHash::Id(id))
        .ok_or("Torrent not found".to_string())?;

    for _ in 0..100 {
        if handle.metadata.load().is_some() {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let metadata = handle.metadata.load();
    let info = &metadata
        .as_ref()
        .ok_or("Timeout waiting for metadata")?
        .info;

    let mut files = Vec::new();

    if let Some(info_files) = &info.files {
        for (idx, f) in info_files.iter().enumerate() {
            let name = f
                .path
                .iter()
                .map(|p| String::from_utf8_lossy(p).to_string())
                .collect::<Vec<_>>()
                .join("/");

            files.push(TorrentFile {
                id: idx,
                name,
                size: f.length,
                progress: 0.0,
            });
        }
    } else {
        let len = info.length.ok_or("Single file torrent has no length")?;
        let name_bytes = info.name.clone().ok_or("Single file torrent has no name")?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();
        files.push(TorrentFile {
            id: 0,
            name,
            size: len,
            progress: handle.stats().progress_bytes as f64 / len as f64 * 100.0,
        });
    }

    Ok(files)
}

async fn add_torrent_internal(
    session: &Arc<Session>,
    magnet: &str,
    paused: bool,
) -> Result<Arc<ManagedTorrent>, String> {
    let response = session
        .add_torrent(
            AddTorrent::from_url(magnet),
            Some(AddTorrentOptions {
                overwrite: true,
                paused,
                ..Default::default()
            }),
        )
        .await
        .map_err(|e| e.to_string())?;

    match response {
        AddTorrentResponse::Added(_, h) => Ok(h),
        AddTorrentResponse::AlreadyManaged(_, h) => Ok(h),
        _ => Err("Unexpected response from add_torrent".to_string()),
    }
}

#[command]
pub async fn stream_torrent(
    _app: AppHandle,
    session: State<'_, Arc<Session>>,
    magnet: String,
    file_id: Option<usize>,
) -> Result<StreamResponse, String> {
    let handle = add_torrent_internal(&session, &magnet, false).await?;

    let info_hash = handle
        .info_hash()
        .0
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    add_to_persistence(&_app, &magnet, &info_hash);

    let base_url = ensure_global_server(session).await?;

    let target_file_id = if let Some(fid) = file_id {
        fid
    } else {
        find_largest_file(&handle).await?.0
    };

    Ok(StreamResponse {
        url: format!(
            "{}/torrents/{}/stream/{}",
            base_url,
            handle.id(),
            target_file_id
        ),
        info_hash,
    })
}

#[command]
pub async fn stream_torrent_by_id(
    _app: AppHandle,
    session: State<'_, Arc<Session>>,
    id: usize,
    file_id: Option<usize>,
) -> Result<StreamResponse, String> {
    let handle = session
        .get(TorrentIdOrHash::Id(id))
        .ok_or("Torrent not found".to_string())?;

    let base_url = ensure_global_server(session).await?;

    let target_file_id = if let Some(fid) = file_id {
        fid
    } else {
        find_largest_file(&handle).await?.0
    };

    let info_hash = handle
        .info_hash()
        .0
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    Ok(StreamResponse {
        url: format!("{}/torrents/{}/stream/{}", base_url, id, target_file_id),
        info_hash,
    })
}

async fn global_stream_handler(
    AxumState(session): AxumState<Arc<Session>>,
    Path((id, file_id)): Path<(usize, usize)>,
    req: Request<Body>,
) -> impl IntoResponse {
    let handle = match session.get(TorrentIdOrHash::Id(id)) {
        Some(h) => h,
        None => return (StatusCode::NOT_FOUND, "Torrent not found").into_response(),
    };

    let (file_len, file_name) = match get_file_info(&handle, file_id) {
        Ok(info) => info,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    // ALWAYS use raw stream handler. No transcoding.
    stream_handler(req, handle, file_id, file_len, file_name)
        .await
        .into_response()
}

async fn subtitle_handler(
    AxumState(session): AxumState<Arc<Session>>,
    Path((id, file_id, track_index)): Path<(usize, usize, usize)>,
) -> impl IntoResponse {
    let handle = match session.get(TorrentIdOrHash::Id(id)) {
        Some(h) => h,
        None => return (StatusCode::NOT_FOUND, "Torrent not found").into_response(),
    };

    let stream_result = handle.stream(file_id);

    match stream_result {
        Ok(rqbit_stream) => {
            let child = Command::new("ffmpeg")
                .arg("-i")
                .arg("pipe:0")
                .arg("-map")
                .arg(format!("0:s:{}", track_index))
                .arg("-f")
                .arg("webvtt")
                .arg("pipe:1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn();

            match child {
                Ok(mut child_process) => {
                    let mut stdin = child_process.stdin.take().unwrap();
                    let stdout = child_process.stdout.take().unwrap();

                    tokio::spawn(async move {
                        let mut reader = rqbit_stream;
                        if let Err(e) = tokio::io::copy(&mut reader, &mut stdin).await {
                            log::warn!("Error piping to ffmpeg subs: {}", e);
                        }
                    });

                    let reader_stream = ReaderStream::new(stdout);
                    let body = Body::from_stream(reader_stream);

                    let mut headers = header::HeaderMap::new();
                    headers.insert(header::CONTENT_TYPE, "text/vtt".parse().unwrap());

                    (StatusCode::OK, headers, body).into_response()
                }
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "FFmpeg failed").into_response(),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Stream failed").into_response(),
    }
}

async fn find_largest_file(handle: &Arc<ManagedTorrent>) -> Result<(usize, u64, String), String> {
    let mut info = None;
    for _ in 0..100 {
        if let Some(metadata) = handle.metadata.load().as_ref() {
            info = Some(metadata.info.clone());
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let info = info.ok_or("Timeout waiting for metadata")?;

    if let Some(files) = &info.files {
        let flat_files: Vec<_> = files.iter().collect();
        let (id, f) = flat_files
            .iter()
            .enumerate()
            .max_by_key(|(_, f)| f.length)
            .ok_or("No files in torrent")?;

        let name = f
            .path
            .iter()
            .map(|p| String::from_utf8_lossy(p).to_string())
            .collect::<Vec<_>>()
            .join("/");
        Ok((id, f.length, name))
    } else {
        let len = info.length.ok_or("Single file torrent has no length")?;
        let name_bytes = info.name.clone().ok_or("Single file torrent has no name")?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();
        Ok((0, len, name))
    }
}

fn get_file_info(handle: &Arc<ManagedTorrent>, file_id: usize) -> Result<(u64, String), ()> {
    let metadata = handle.metadata.load();
    let info = &metadata.as_ref().ok_or(())?.info;

    if let Some(files) = &info.files {
        let f = files.iter().nth(file_id).ok_or(())?;
        let name = f
            .path
            .iter()
            .map(|p| String::from_utf8_lossy(p).to_string())
            .collect::<Vec<_>>()
            .join("/");
        Ok((f.length, name))
    } else {
        if file_id != 0 {
            return Err(());
        }
        let len = info.length.ok_or(())?;
        let name_bytes = info.name.clone().ok_or(())?;
        Ok((len, String::from_utf8_lossy(&name_bytes).to_string()))
    }
}

#[command]
pub async fn get_torrents(
    app: AppHandle,
    session: State<'_, Arc<Session>>,
) -> Result<Vec<TorrentState>, String> {
    let db = app.state::<Database>();
    let conn = db.get();

    let result = session.with_torrents(|torrents| {
        let mut result = Vec::new();
        let mut tracker = SPEED_TRACKER.lock().unwrap();

        for (id, handle) in torrents {
            let stats = handle.stats();
            let info_hash = handle
                .info_hash()
                .0
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();

            // Fetch metadata from DB (all fields nullable except anime_id- default 0)
            let metadata: Option<(i32, f32, Option<String>, Option<String>)> = conn.query_row(
                "SELECT anime_id, episode_number, anime_title, anime_cover FROM torrents WHERE LOWER(info_hash) = LOWER(?1)",
                params![info_hash],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2).ok(), row.get(3).ok()))
            ).ok();

            let (db_anime_id, db_episode_number, db_anime_title, db_anime_cover) = match metadata {
                Some(m) if m.0 > 0 => (Some(m.0), Some(m.1), m.2, m.3),
                _ => (None, None, None, None),
            };

            let info = handle.metadata.load();
            let mut name = info.as_ref().and_then(|m| {
                m.info
                    .name
                    .as_ref()
                    .map(|n| String::from_utf8_lossy(n).to_string())
            });

            if name.is_none() {
                name = db_anime_title.clone();
            }

            let total_bytes = stats.total_bytes;
            let progress = if total_bytes > 0 {
                (stats.progress_bytes as f64 / total_bytes as f64) * 100.0
            } else {
                0.0
            };

            let current_bytes = stats.progress_bytes;
            let current_upload = stats.uploaded_bytes;
            let now = Instant::now();

            let (speed, upload_speed) =
                if let Some((prev_bytes, prev_upload, prev_time)) = tracker.get(&id) {
                    let bytes_diff = current_bytes.saturating_sub(*prev_bytes);
                    let upload_diff = current_upload.saturating_sub(*prev_upload);
                    let time_diff = now.duration_since(*prev_time).as_secs_f64();

                    if time_diff > 0.0 {
                        (
                            bytes_diff as f64 / time_diff,
                            upload_diff as f64 / time_diff,
                        )
                    } else {
                        (0.0, 0.0)
                    }
                } else {
                    (0.0, 0.0)
                };
            tracker.insert(id, (current_bytes, current_upload, now));

            let state = if handle.is_paused() {
                "paused".to_string()
            } else if stats.progress_bytes >= total_bytes && total_bytes > 0 {
                "seeding".to_string()
            } else if stats.progress_bytes == 0 && total_bytes > 0 {
                "starting".to_string()
            } else {
                "downloading".to_string()
            };

            result.push(TorrentState {
                id: id,
                name,
                progress,
                speed,
                upload_speed,
                peers: 0,
                seeds: 0,
                state,
                total_size: total_bytes,
                downloaded: stats.progress_bytes,
                info_hash: info_hash,
                anime_id: db_anime_id,
                episode_number: db_episode_number,
                anime_title: db_anime_title,
                anime_cover: db_anime_cover,
            });
        }
        result
    });
    Ok(result)
}

#[command]
pub async fn pause_torrent(
    app: AppHandle,
    session: State<'_, Arc<Session>>,
    id: usize,
) -> Result<(), String> {
    if let Some(handle) = session.get(TorrentIdOrHash::Id(id)) {
        session.pause(&handle).await.map_err(|e| e.to_string())?;
        let info_hash = handle
            .info_hash()
            .0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        update_persistence_state(&app, &info_hash, true);
        Ok(())
    } else {
        Err("Torrent not found".to_string())
    }
}

#[command]
pub async fn resume_torrent(
    app: AppHandle,
    session: State<'_, Arc<Session>>,
    id: usize,
) -> Result<(), String> {
    if let Some(handle) = session.get(TorrentIdOrHash::Id(id)) {
        session.unpause(&handle).await.map_err(|e| e.to_string())?;
        let info_hash = handle
            .info_hash()
            .0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        update_persistence_state(&app, &info_hash, false);
        Ok(())
    } else {
        Err("Torrent not found".to_string())
    }
}

#[command]
pub async fn delete_torrent(
    app: AppHandle,
    session: State<'_, Arc<Session>>,
    id: usize,
    delete_files: bool,
) -> Result<(), String> {
    if let Some(handle) = session.get(TorrentIdOrHash::Id(id)) {
        let info_hash = handle
            .info_hash()
            .0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        session
            .delete(TorrentIdOrHash::Id(id), delete_files)
            .await
            .map_err(|e| e.to_string())?;
        remove_from_persistence(&app, &info_hash);
        Ok(())
    } else {
        Err("Torrent not found".to_string())
    }
}

#[command]
pub async fn open_in_external_player(
    app: AppHandle,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
    config: State<'_, ConfigState>,
) -> Result<(), String> {
    log::info!("Opening external player for URL: {}", url);

    // Resolve player executable: use configured path, fall back to "mpv"
    let player_exe = config
        .get_player_config()
        .ok()
        .and_then(|c| c.external_player_path)
        .unwrap_or_else(|| "mpv".to_string());

    let mut cmd = std::process::Command::new(&player_exe);
    cmd.arg(&url);
    // Disable yt-dlp/youtube-dl hook so MPV uses its native HLS stack directly.
    cmd.arg("--no-ytdl");
    // Forward any CDN-required headers (e.g. Referer, Cookie) to mpv so it can
    // fetch HLS segments without a proxy relay.
    if let Some(hdrs) = headers {
        if !hdrs.is_empty() {
            let fields: Vec<String> = hdrs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
            cmd.arg(format!("--http-header-fields={}", fields.join(",")));
        }
    }

    // Apply Shaders to external player if enabled
    if let Ok(player_config) = config.get_player_config() {
        if player_config.shaders.enabled && !player_config.shaders.selected_shaders.is_empty() {
            let mut resolved_shaders = Vec::new();
            if let Ok(res_dir) = app.path().resource_dir() {
                for shader in &player_config.shaders.selected_shaders {
                    let file_name = shader.split('/').last().unwrap_or(shader);
                    let absolute_path = res_dir.join("shaders").join(file_name);
                    resolved_shaders.push(absolute_path.to_string_lossy().to_string());
                }
            }
            if !resolved_shaders.is_empty() {
                #[cfg(target_os = "windows")]
                let separator = ";";
                #[cfg(not(target_os = "windows"))]
                let separator = ":";
                cmd.arg(format!(
                    "--glsl-shaders={}",
                    resolved_shaders.join(separator)
                ));
            }
        }
    }

    cmd.spawn()
        .map_err(|e| format!("Failed to launch '{}': {}", player_exe, e))?;
    Ok(())
}

async fn stream_handler(
    req: Request<Body>,
    handle: Arc<ManagedTorrent>,
    file_id: usize,
    file_len: u64,
    file_name: String,
) -> impl IntoResponse {
    log::info!("Stream request for file: {}", file_name);

    let range_header = req
        .headers()
        .get(header::RANGE)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let (start, end) = match parse_range(&range_header, file_len) {
        Ok(range) => range,
        Err(_) => {
            let mut res = (StatusCode::RANGE_NOT_SATISFIABLE, "Invalid Range").into_response();
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
            return res;
        }
    };

    let len = end - start + 1;
    log::info!(
        "Streaming range: {}-{}/{} (len: {})",
        start,
        end,
        file_len,
        len
    );

    let content_type = if file_name.ends_with(".mp4") {
        "video/mp4"
    } else if file_name.ends_with(".mkv") {
        "video/x-matroska"
    } else if file_name.ends_with(".webm") {
        "video/webm"
    } else if file_name.ends_with(".avi") {
        "video/x-msvideo"
    } else {
        "application/octet-stream"
    };

    let stream_result = handle.stream(file_id);

    match stream_result {
        Ok(mut s) => {
            if let Err(_) =
                tokio::io::AsyncSeekExt::seek(&mut s, std::io::SeekFrom::Start(start)).await
            {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to seek").into_response();
            }

            let limited_stream = tokio::io::AsyncReadExt::take(s, len);
            let reader_stream = ReaderStream::new(limited_stream);
            let body = Body::from_stream(reader_stream);

            let mut headers = header::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
            headers.insert(header::CONTENT_LENGTH, len.to_string().parse().unwrap());
            headers.insert(
                header::CONTENT_RANGE,
                format!("bytes {}-{}/{}", start, end, file_len)
                    .parse()
                    .unwrap(),
            );
            headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());

            (StatusCode::PARTIAL_CONTENT, headers, body).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open stream").into_response(),
    }
}

fn parse_range(range_header: &Option<String>, file_len: u64) -> Result<(u64, u64), ()> {
    if let Some(range) = range_header {
        if let Some(range) = range.strip_prefix("bytes=") {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().map_err(|_| ())?;
                let end = if parts[1].is_empty() {
                    file_len - 1
                } else {
                    parts[1].parse::<u64>().map_err(|_| ())?
                };
                return Ok((start, end));
            }
        }
    }
    Ok((0, file_len - 1))
}

#[derive(Serialize, serde::Deserialize, Debug, Clone)]
pub struct TorrentMetadata {
    pub anime_id: i32,
    pub episode_number: f32,
    pub anime_title: Option<String>,
    pub anime_cover: Option<String>,
}

#[derive(Serialize, serde::Deserialize, Debug, Clone)]
pub struct PersistTorrent {
    pub magnet: String,
    pub info_hash: String,
    pub name: Option<String>,
    pub paused: bool,
    pub anime_id: Option<i32>,
    pub episode_number: Option<f32>,
    pub anime_title: Option<String>,
    pub anime_cover: Option<String>,
}

fn add_to_persistence(app: &AppHandle, magnet: &str, info_hash: &str) {
    let db = app.state::<Database>();
    let conn = db.get();
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let anime_name = url::Url::parse(magnet).ok().and_then(|url| {
        url.query_pairs()
            .find(|(k, _)| k == "dn")
            .map(|(_, v)| v.to_string())
    });

    let _ = conn.execute(
        "INSERT OR IGNORE INTO torrents (
            info_hash, anime_id, episode_number, anime_title, anime_cover, 
            magnet_uri, status, progress, created_at, updated_at
        ) VALUES (?1, 0, 0.0, ?2, '', ?3, ?4, ?5, ?6, ?7)",
        params![
            info_hash,
            anime_name,   // raw dn= from magnet, used as torrent name fallback only
            magnet,
            "downloading",
            0.0,
            now,
            now
        ],
    );
}

fn update_persistence_state(app: &AppHandle, info_hash: &str, paused: bool) {
    let db = app.state::<Database>();
    let conn = db.get();
    let status = if paused { "paused" } else { "downloading" };
    
    let _ = conn.execute(
        "UPDATE torrents SET status = ?1, updated_at = ?2 WHERE info_hash = ?3",
        params![
            status,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            info_hash
        ],
    );
}

fn remove_from_persistence(app: &AppHandle, info_hash: &str) {
    let db = app.state::<Database>();
    let conn = db.get();
    let _ = conn.execute("DELETE FROM torrents WHERE info_hash = ?1", params![info_hash]);
}

pub async fn restore_torrents(app: &AppHandle, session: &Arc<Session>) {
    let db = app.state::<Database>();
    let conn = db.get();
    
    let torrents_to_restore = {
        let mut stmt = conn.prepare("SELECT magnet_uri, status FROM torrents").unwrap();
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap();

        let mut list = Vec::new();
        for row in rows {
            if let Ok(data) = row {
                list.push(data);
            }
        }
        list
    };

    log::info!("Restoring {} torrents from persistence...", torrents_to_restore.len());
    for (magnet, status) in torrents_to_restore {
        let _ = session
            .add_torrent(
                AddTorrent::from_url(&magnet),
                Some(AddTorrentOptions {
                    overwrite: true,
                    paused: status == "paused",
                    ..Default::default()
                }),
            )
            .await;
    }
}

#[command]
pub async fn save_torrent_metadata(
    app: AppHandle,
    info_hash: String,
    metadata: TorrentMetadata,
) -> Result<(), String> {
    let db = app.state::<Database>();
    let conn = db.get();
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Use UPDATE with WHERE to preserve magnet/status if row exists, or do INSERT if not
    let updated = conn.execute(
        "UPDATE torrents SET 
            anime_id = ?1, 
            episode_number = ?2, 
            anime_title = ?3, 
            anime_cover = ?4,
            updated_at = ?5
         WHERE LOWER(info_hash) = LOWER(?6)",
        params![
            metadata.anime_id,
            metadata.episode_number,
            metadata.anime_title,
            metadata.anime_cover,
            now,
            info_hash
        ],
    ).map_err(|e| e.to_string())?;

    log::info!("[Metadata] Updated {} rows for hash {}", updated, info_hash);

    // If no row was updated (race: metadata saved before stream_torrent INSERT),
    // insert a minimal row with just the metadata
    if updated == 0 {
        log::info!("[Metadata] No existing row for hash {}, inserting new row", info_hash);
        conn.execute(
            "INSERT OR IGNORE INTO torrents (info_hash, anime_id, episode_number, anime_title, anime_cover, magnet_uri, status, progress, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, '', 'downloading', 0.0, ?6, ?6)",
            params![
                info_hash,
                metadata.anime_id,
                metadata.episode_number,
                metadata.anime_title,
                metadata.anime_cover,
                now
            ],
        ).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[command]
pub async fn get_saved_torrents_for_episode(
    app: AppHandle,
    anime_id: i32,
    episode_number: f32,
) -> Result<Vec<PersistTorrent>, String> {
    let db = app.state::<Database>();
    let conn = db.get();
    
    let mut stmt = conn.prepare(
        "SELECT magnet_uri, info_hash, anime_title, status, anime_id, episode_number, anime_cover 
         FROM torrents 
         WHERE anime_id = ?1 AND episode_number = ?2"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(params![anime_id, episode_number], |row| {
        Ok(PersistTorrent {
            magnet: row.get(0)?,
            info_hash: row.get(1)?,
            name: row.get(2).ok(),
            paused: row.get::<_, String>(3)? == "paused",
            anime_id: Some(row.get(4)?),
            episode_number: Some(row.get(5)?),
            anime_title: row.get(2).ok(),
            anime_cover: row.get(6).ok(),
        })
    }).map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for row in rows {
        if let Ok(t) = row {
            results.push(t);
        }
    }
    
    Ok(results)
}

async fn static_file_handler(
    Path(filename): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    // Get the torrents folder path (same as where torrent files are downloaded)
    // You'll need to determine this path - it might be from session config or app data dir
    let torrents_dir = std::path::PathBuf::from("/home/ThunderBlaze/Downloads/zafkiel"); // TODO: Get actual path

    let file_path = torrents_dir.join(&filename);

    // Security check: only allow .mp4 files
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("mp4") {
        return (StatusCode::BAD_REQUEST, "Only MP4 files allowed").into_response();
    }

    // Check if file exists
    if !file_path.exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    // Get file metadata
    let metadata = match tokio::fs::metadata(&file_path).await {
        Ok(m) => m,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read file metadata",
            )
                .into_response();
        }
    };

    let file_len = metadata.len();

    // Parse range header (same as torrent handler)
    let range_header = req
        .headers()
        .get(header::RANGE)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let (start, end) = match parse_range(&range_header, file_len) {
        Ok(range) => range,
        Err(_) => {
            let mut res = (StatusCode::RANGE_NOT_SATISFIABLE, "Invalid Range").into_response();
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
            return res;
        }
    };

    let len = end - start + 1;

    // Open file and seek to range
    let file = match tokio::fs::File::open(&file_path).await {
        Ok(f) => f,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file").into_response();
        }
    };

    let mut file = tokio::io::BufReader::new(file);
    if let Err(_) = tokio::io::AsyncSeekExt::seek(&mut file, std::io::SeekFrom::Start(start)).await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to seek file").into_response();
    }

    // Create limited reader for the range
    let limited_reader = tokio::io::AsyncReadExt::take(file, len);
    let reader_stream = ReaderStream::new(limited_reader);
    let body = Body::from_stream(reader_stream);

    // Set headers
    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "video/mp4".parse().unwrap());
    headers.insert(header::CONTENT_LENGTH, len.to_string().parse().unwrap());
    headers.insert(
        header::CONTENT_RANGE,
        format!("bytes {}-{}/{}", start, end, file_len)
            .parse()
            .unwrap(),
    );
    headers.insert(header::ACCEPT_RANGES, "bytes".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());

    (StatusCode::PARTIAL_CONTENT, headers, body).into_response()
}
