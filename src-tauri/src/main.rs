// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // tauri-plugin-libmpv embeds mpv via XID (X11 window ID), which only works
    // when GTK/WebKitGTK is running on X11 (XWayland).  This MUST be set before
    // any GTK code runs — setting it inside run() is too late because GTK may
    // initialise as part of dynamic library loading before run() is reached.
    #[cfg(target_os = "linux")]
    unsafe {
        // Force WebKitGTK onto XWayland so the Tauri window gets an Xlib handle.
        std::env::set_var("GDK_BACKEND", "x11");
        // Make libmpv itself also prefer X11 so --wid embedding works.
        // (When WAYLAND_DISPLAY is present mpv opens a native Wayland surface
        // which becomes a separate tiled window instead of embedding.)
        std::env::remove_var("WAYLAND_DISPLAY");
    }

    app_lib::run();
}
