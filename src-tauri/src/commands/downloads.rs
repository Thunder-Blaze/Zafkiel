/// Extension download management — HLS/MP4 episode downloads via ffmpeg.
///
/// Downloads are stored at:
///   {OS Downloads dir}/zafkiel/{Anime Name}/S{NN} Ep {NNN} - {source label}.mp4
///
/// Progress is emitted as `extension-download-progress` Tauri events so the
/// frontend can update its UI without polling.
use crate::database::Database;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager, State, command};
use tokio::io::AsyncBufReadExt;

// ─────────────────────────────────────────────────────────────────────────────
// Shared types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDownloadParams {
    pub anime_name: String,
    pub anilist_id: Option<i64>,
    pub season: i32,
    /// May be a decimal (e.g. 5.5 for half-episodes / specials).
    pub episode_number: f64,
    /// Human-readable label such as "HorribleSubs 1080p (JPN)".
    pub source_label: String,
    pub extension_id: String,
    /// Resolved HLS (.m3u8) or MP4 URL.
    pub url: String,
    /// HTTP headers needed to fetch the stream (e.g. Referer, Cookie).
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDownload {
    pub id: i64,
    pub anime_name: String,
    pub anilist_id: Option<i64>,
    pub season: i32,
    pub episode_number: f64,
    pub source_label: String,
    pub extension_id: String,
    /// "pending" | "downloading" | "completed" | "failed" | "cancelled"
    pub status: String,
    pub progress: f64,
    pub file_path: Option<String>,
    pub error_msg: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgressEvent {
    pub id: i64,
    pub progress: f64,
    pub status: String,
    pub error_msg: Option<String>,
    pub file_path: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_owned()
}

fn build_output_path(
    app: &AppHandle,
    anime_name: &str,
    season: i32,
    episode: f64,
    source_label: &str,
) -> PathBuf {
    let base = app
        .path()
        .download_dir()
        .unwrap_or_else(|_| PathBuf::from("downloads"))
        .join("zafkiel");

    let safe_anime = sanitize_filename(anime_name);
    let safe_source = sanitize_filename(source_label);

    let ep_int = episode.floor() as u64;
    let ep_frac = ((episode - episode.floor()) * 10.0).round() as u64;
    let ep_str = if ep_frac > 0 {
        format!("{ep_int}.{ep_frac}")
    } else {
        // Pad to at least 3 digits (handles 1000+ episodes in long series)
        format!("{ep_int:03}")
    };

    let filename = format!("S{season:02} Ep {ep_str} - {safe_source}.mp4");
    let dir = base.join(&safe_anime);
    std::fs::create_dir_all(&dir).ok();
    dir.join(filename)
}

fn insert_download_record(
    db: &Database,
    p: &StartDownloadParams,
    file_path: &str,
) -> Result<i64, String> {
    let conn = db.get();
    let now = now_secs();
    conn.execute(
        "INSERT INTO extension_downloads (
            anime_name, anilist_id, season, episode_number, source_label,
            extension_id, status, progress, file_path, url, created_at, updated_at
         ) VALUES (?1,?2,?3,?4,?5,?6,'pending',0.0,?7,?8,?9,?9)",
        params![
            p.anime_name,
            p.anilist_id,
            p.season,
            p.episode_number,
            p.source_label,
            p.extension_id,
            file_path,
            p.url,
            now,
        ],
    )
    .map_err(|e| format!("DB insert failed: {e}"))?;
    Ok(conn.last_insert_rowid())
}

fn db_update_status(db: &Database, id: i64, status: &str, progress: f64, error: Option<&str>) {
    let conn = db.get();
    let now = now_secs();
    let _ = conn.execute(
        "UPDATE extension_downloads SET status=?1, progress=?2, error_msg=?3, updated_at=?4 WHERE id=?5",
        params![status, progress, error, now, id],
    );
}

fn db_complete(db: &Database, id: i64, file_path: &str) {
    let conn = db.get();
    let now = now_secs();
    let _ = conn.execute(
        "UPDATE extension_downloads SET status='completed', progress=100.0, file_path=?1, updated_at=?2 WHERE id=?3",
        params![file_path, now, id],
    );
}

// Parse "  Duration: HH:MM:SS.xx, ..." from ffmpeg stderr
fn parse_ffmpeg_duration(line: &str) -> Option<f64> {
    let idx = line.find("Duration:")?;
    parse_hhmmss(line[idx + 9..].trim_start())
}

// Parse "... time=HH:MM:SS.xx ..." from ffmpeg progress lines
fn parse_ffmpeg_time(line: &str) -> Option<f64> {
    let idx = line.find("time=")?;
    parse_hhmmss(line[idx + 5..].trim_start())
}

fn parse_hhmmss(s: &str) -> Option<f64> {
    // Stop at the first comma, space, or newline so we don't grab trailing chars
    let s = s
        .split(|c| c == ',' || c == ' ' || c == '\n')
        .next()
        .unwrap_or("")
        .trim();
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    if parts.len() < 3 {
        return None;
    }
    let h: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let sec: f64 = parts[2].parse().ok()?;
    Some(h * 3600.0 + m * 60.0 + sec)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tauri commands
// ─────────────────────────────────────────────────────────────────────────────

/// Start downloading an episode in the background using ffmpeg.
///
/// Returns the DB row ID immediately; progress is reported via
/// `extension-download-progress` events.
#[command]
pub async fn start_extension_download(
    app: AppHandle,
    db: State<'_, Database>,
    params: StartDownloadParams,
) -> Result<i64, String> {
    let out_path = build_output_path(
        &app,
        &params.anime_name,
        params.season,
        params.episode_number,
        &params.source_label,
    );
    let out_path_str = out_path.to_string_lossy().to_string();

    let id = insert_download_record(&db, &params, &out_path_str)?;

    // Extract auth headers to pass to ffmpeg.
    let referer = params
        .headers
        .get("Referer")
        .or_else(|| params.headers.get("referer"))
        .map(|s| s.as_str());
    let cookie = params
        .headers
        .get("Cookie")
        .or_else(|| params.headers.get("cookie"))
        .map(|s| s.as_str());

    // Pass auth headers directly to ffmpeg via the -headers option ("Key: Value\r\n" format).
    let mut ffmpeg_headers = String::new();
    if let Some(r) = referer {
        ffmpeg_headers.push_str(&format!("Referer: {r}\r\n"));
    }
    if let Some(c) = cookie {
        ffmpeg_headers.push_str(&format!("Cookie: {c}\r\n"));
    }
    let url = params.url.clone();
    let db_arc = db.inner().clone();
    let app2 = app.clone();

    tokio::spawn(async move {
        let emit = |progress: f64, status: &str, error: Option<String>, file: Option<String>| {
            let _ = app2.emit(
                "extension-download-progress",
                DownloadProgressEvent {
                    id,
                    progress,
                    status: status.to_owned(),
                    error_msg: error,
                    file_path: file,
                },
            );
        };

        emit(0.0, "downloading", None, None);
        db_update_status(&db_arc, id, "downloading", 0.0, None);

        let mut ffmpeg_args: Vec<String> = vec!["-y".to_string()];
        if !ffmpeg_headers.is_empty() {
            ffmpeg_args.extend(["-headers".to_string(), ffmpeg_headers.clone()]);
        }
        // Note: aac_adtstoasc is intentionally omitted — fMP4 HLS segments already
        // use AAC-LC in ASC format; applying the filter on such streams aborts ffmpeg.
        ffmpeg_args.extend([
            "-i".to_string(),
            url.clone(),
            "-c".to_string(),
            "copy".to_string(),
            out_path_str.clone(),
        ]);

        let spawn_result = tokio::process::Command::new("ffmpeg")
            .args(&ffmpeg_args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match spawn_result {
            Err(e) => {
                let msg = format!(
                    "ffmpeg not found or failed to start: {e}. \
                     Make sure ffmpeg is installed and available in PATH."
                );
                log::error!("[Downloads] {msg}");
                emit(0.0, "failed", Some(msg.clone()), None);
                db_update_status(&db_arc, id, "failed", 0.0, Some(&msg));
            }

            Ok(mut child) => {
                let mut duration_secs: f64 = 0.0;
                let mut last_stderr_lines: Vec<String> = Vec::new();

                if let Some(stderr) = child.stderr.take() {
                    let mut lines = tokio::io::BufReader::new(stderr).lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        log::debug!("[ffmpeg] {line}");
                        // Keep last 8 lines for error reporting
                        if last_stderr_lines.len() >= 8 {
                            last_stderr_lines.remove(0);
                        }
                        last_stderr_lines.push(line.clone());

                        if duration_secs == 0.0 {
                            if let Some(d) = parse_ffmpeg_duration(&line) {
                                duration_secs = d;
                            }
                        }
                        if let Some(t) = parse_ffmpeg_time(&line) {
                            let pct = if duration_secs > 0.0 {
                                (t / duration_secs * 100.0).clamp(0.0, 99.0)
                            } else {
                                0.0
                            };
                            emit(pct, "downloading", None, None);
                            db_update_status(&db_arc, id, "downloading", pct, None);
                        }
                    }
                }

                match child.wait().await {
                    Ok(status) if status.success() => {
                        log::info!("[Downloads] Completed: {out_path_str}");
                        emit(100.0, "completed", None, Some(out_path_str.clone()));
                        db_complete(&db_arc, id, &out_path_str);
                    }
                    Ok(status) => {
                        let last_lines = last_stderr_lines.join(" | ");
                        let msg = format!(
                            "ffmpeg exited with code {} — {}",
                            status.code().unwrap_or(-1),
                            last_lines
                        );
                        log::error!("[Downloads] {msg}");
                        emit(0.0, "failed", Some(msg.clone()), None);
                        db_update_status(&db_arc, id, "failed", 0.0, Some(&msg));
                    }
                    Err(e) => {
                        let msg = format!("Process wait error: {e}");
                        log::error!("[Downloads] {msg}");
                        emit(0.0, "failed", Some(msg.clone()), None);
                        db_update_status(&db_arc, id, "failed", 0.0, Some(&msg));
                    }
                }
            }
        }
    });

    Ok(id)
}

/// Returns all download records ordered by newest first.
#[command]
pub fn get_extension_downloads(db: State<'_, Database>) -> Result<Vec<ExtensionDownload>, String> {
    let conn = db.get();
    let mut stmt = conn
        .prepare(
            "SELECT id, anime_name, anilist_id, season, episode_number, source_label,
                    extension_id, status, progress, file_path, error_msg, created_at, updated_at
             FROM extension_downloads ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let items = stmt
        .query_map([], |row| {
            Ok(ExtensionDownload {
                id: row.get(0)?,
                anime_name: row.get(1)?,
                anilist_id: row.get(2)?,
                season: row.get(3)?,
                episode_number: row.get(4)?,
                source_label: row.get(5)?,
                extension_id: row.get(6)?,
                status: row.get(7)?,
                progress: row.get(8)?,
                file_path: row.get(9)?,
                error_msg: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

/// Mark a download as cancelled. Does not kill the ffmpeg process
/// (process tracking would require storing PIDs; cancellation stops the DB record).
#[command]
pub fn cancel_extension_download(db: State<'_, Database>, id: i64) -> Result<(), String> {
    db_update_status(&db, id, "cancelled", 0.0, None);
    Ok(())
}

/// Delete a download record from the DB (does not delete the file on disk).
#[command]
pub fn remove_extension_download(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.get();
    conn.execute("DELETE FROM extension_downloads WHERE id=?1", params![id])
        .map_err(|e| format!("DB delete failed: {e}"))?;
    Ok(())
}
