//! X11 window stacking helper for tauri-plugin-libmpv on Linux.
//!
//! When mpv is initialised via `--wid` (X11 window ID), it creates a sub-window
//! inside the Tauri parent window.  Because this sub-window is created *after*
//! the WebKitGTK rendering window, it ends up at the top of the X11 stacking
//! order and obscures the WebView — including the player controls overlay.
//!
//! `lower_mpv_subwindow` calls `XQueryTree` to find the topmost child of the
//! Tauri X11 window (that is the mpv sub-window) and lowers it to the bottom of
//! the stack, making the WebView visible on top of the video.

// ── X11 bindings (Linux only) ─────────────────────────────────────────────────
#[cfg(target_os = "linux")]
use std::os::raw::{c_char, c_int, c_ulong, c_void};

#[cfg(target_os = "linux")]
type XWindow = c_ulong;
#[cfg(target_os = "linux")]
type XDisplay = *mut c_void;

#[cfg(target_os = "linux")]
#[link(name = "X11")]
unsafe extern "C" {
    fn XOpenDisplay(display_name: *const c_char) -> XDisplay;
    fn XCloseDisplay(display: XDisplay) -> c_int;
    /// Returns children in bottom-to-top stacking order.
    fn XQueryTree(
        display: XDisplay,
        w: XWindow,
        root_return: *mut XWindow,
        parent_return: *mut XWindow,
        children_return: *mut *mut XWindow,
        nchildren_return: *mut u32,
    ) -> c_int;
    fn XLowerWindow(display: XDisplay, w: XWindow) -> c_int;
    fn XFlush(display: XDisplay) -> c_int;
    fn XFree(data: *mut c_void) -> c_int;
}

// ── Tauri command ─────────────────────────────────────────────────────────────

/// Lower the mpv X11 sub-window below the WebKit window so controls are visible.
///
/// Must be called from the frontend **after** `plugin:libmpv|init` returns.
/// On non-Linux platforms this is a silent no-op.
#[allow(unused_variables)]
#[tauri::command]
pub fn lower_mpv_subwindow(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        use tauri::Manager;

        let window = app
            .get_webview_window("main")
            .ok_or_else(|| "lower_mpv_subwindow: main window not found".to_string())?;

        let wh = window.window_handle().map_err(|e| e.to_string())?;

        let RawWindowHandle::Xlib(xlib) = wh.as_raw() else {
            // Wayland native handle — WID embedding not available; nothing to do.
            log::debug!("[mpv_window] Non-Xlib handle; skipping window restack");
            return Ok(());
        };

        let parent_xid = xlib.window as XWindow;

        unsafe {
            let display = XOpenDisplay(std::ptr::null());
            if display.is_null() {
                return Err(
                    "[mpv_window] XOpenDisplay failed — is $DISPLAY set?".to_string(),
                );
            }

            let mut root: XWindow = 0;
            let mut parent: XWindow = 0;
            let mut children: *mut XWindow = std::ptr::null_mut();
            let mut n_children: u32 = 0;

            let status = XQueryTree(
                display,
                parent_xid,
                &mut root,
                &mut parent,
                &mut children,
                &mut n_children,
            );

            if status != 0 && n_children > 0 {
                // Children are returned bottom-to-top; the last one is the
                // topmost (the most recently created) sub-window = mpv.
                let mpv_win = *children.add(n_children as usize - 1);
                XLowerWindow(display, mpv_win);
                XFlush(display);
                log::info!(
                    "[mpv_window] Lowered X11 child {mpv_win:#010x} (mpv) \
                     below WebKit in parent {parent_xid:#010x}"
                );
            } else {
                log::warn!(
                    "[mpv_window] XQueryTree found no children for window {parent_xid:#010x} \
                     (status={status})"
                );
            }

            if !children.is_null() {
                XFree(children as *mut _);
            }

            XCloseDisplay(display);
        }
    }

    Ok(())
}
