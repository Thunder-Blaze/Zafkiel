mod config;
mod commands;

use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Initialize config loader
      let config_loader = config::ConfigLoader::new()
        .expect("Failed to initialize config loader");

      // Store config loader in app state
      app.manage(Arc::new(config_loader));

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_config,
      commands::get_anilist_token,
      commands::set_anilist_token,
      commands::clear_anilist_token,
      commands::get_ui_config,
      commands::update_ui_config,
      commands::update_theme,
      commands::update_glow_effects,
      commands::update_animations,
      commands::update_smooth_scroll,
      commands::update_ui_scale,
      commands::apply_ui_scale,
      commands::open_devtools,
      commands::get_config_path,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
