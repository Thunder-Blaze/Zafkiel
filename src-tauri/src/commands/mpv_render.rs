use parking_lot::Mutex;
use serde_json::Value;
use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter, Manager, State};

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

// ── FFI Types ─────────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpvRenderParam {
    pub type_: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpvOpenglInitParams {
    pub get_proc_address:
        Option<unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char) -> *mut c_void>,
    pub get_proc_address_ctx: *mut c_void,
    pub extra_exts: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpvOpenglFbo {
    pub fbo: c_int,
    pub w: c_int,
    pub h: c_int,
    pub format: c_int,
}

#[repr(C)]
pub struct MpvNode {
    pub val: MpvNodeVal,
    pub format: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MpvNodeVal {
    pub string: *mut c_char,
    pub flag: c_int,
    pub int64: i64,
    pub double: f64,
    pub list: *mut MpvNodeList,
    pub ba: *mut MpvByteArray,
}

#[repr(C)]
pub struct MpvNodeList {
    pub num: c_int,
    pub values: *mut MpvNode,
    pub keys: *mut *mut c_char,
}

#[repr(C)]
pub struct MpvByteArray {
    pub data: *mut c_void,
    pub size: usize,
}

#[repr(C)]
pub struct MpvEvent {
    pub event_id: c_int,
    pub error: c_int,
    pub reply_userdata: u64,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct MpvEventProperty {
    pub name: *const c_char,
    pub format: c_int,
    pub data: *mut c_void,
}

// ── Send Wrappers for Raw Window Handles and Mpv Handles ──────────────────────

struct SendDisplay(RawDisplayHandle);
unsafe impl Send for SendDisplay {}
unsafe impl Sync for SendDisplay {}

struct SendWindow(RawWindowHandle);
unsafe impl Send for SendWindow {}
unsafe impl Sync for SendWindow {}

// Allows a raw pointer to cross the thread boundary into the render thread.
struct SendPtr(*mut c_void);
unsafe impl Send for SendPtr {}

#[derive(Copy, Clone)]
struct MpvHandleWrapper(*mut c_void);
unsafe impl Send for MpvHandleWrapper {}
unsafe impl Sync for MpvHandleWrapper {}

// ── Dynamic Loader ────────────────────────────────────────────────────────────

pub struct MpvApi {
    pub _lib: libloading::Library,
    pub create: unsafe extern "C" fn() -> *mut c_void,
    pub initialize: unsafe extern "C" fn(*mut c_void) -> c_int,
    pub command: unsafe extern "C" fn(*mut c_void, *mut *const c_char) -> c_int,
    pub set_option_string: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int,
    pub set_property: unsafe extern "C" fn(*mut c_void, *const c_char, c_int, *mut c_void) -> c_int,
    pub get_property: unsafe extern "C" fn(*mut c_void, *const c_char, c_int, *mut c_void) -> c_int,
    pub set_property_string:
        unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int,
    pub get_property_string: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    pub terminate_destroy: unsafe extern "C" fn(*mut c_void),
    pub free: unsafe extern "C" fn(*mut c_void),
    pub wait_event: unsafe extern "C" fn(*mut c_void, f64) -> *mut MpvEvent,
    pub observe_property: unsafe extern "C" fn(*mut c_void, u64, *const c_char, c_int) -> c_int,

    // Render API
    pub render_context_create:
        unsafe extern "C" fn(*mut *mut c_void, *mut c_void, *mut MpvRenderParam) -> c_int,
    pub render_context_set_update_callback:
        unsafe extern "C" fn(*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>, *mut c_void),
    pub render_context_update: unsafe extern "C" fn(*mut c_void) -> u64,
    pub render_context_render: unsafe extern "C" fn(*mut c_void, *mut MpvRenderParam) -> c_int,
    pub render_context_free: unsafe extern "C" fn(*mut c_void),
    pub free_node_contents: unsafe extern "C" fn(*mut MpvNode),
}

impl MpvApi {
    pub unsafe fn load() -> Result<Self, String> {
        let names = if cfg!(target_os = "windows") {
            vec!["libmpv-2.dll", "libmpv-1.dll", "libmpv.dll", "mpv-2.dll"]
        } else if cfg!(target_os = "macos") {
            vec!["libmpv.2.dylib", "libmpv.1.dylib", "libmpv.dylib"]
        } else {
            vec!["libmpv.so.2", "libmpv.so.1", "libmpv.so"]
        };

        let mut loaded_lib = None;
        for name in names {
            if let Ok(lib) = unsafe { libloading::Library::new(name) } {
                loaded_lib = Some(lib);
                break;
            }
        }

        let lib = loaded_lib.ok_or_else(|| "Could not load libmpv library".to_string())?;

        let create = *unsafe { lib.get(b"mpv_create\0").map_err(|e| e.to_string())? };
        let initialize = *unsafe { lib.get(b"mpv_initialize\0").map_err(|e| e.to_string())? };
        let command = *unsafe { lib.get(b"mpv_command\0").map_err(|e| e.to_string())? };
        let set_option_string = *unsafe {
            lib.get(b"mpv_set_option_string\0")
                .map_err(|e| e.to_string())?
        };
        let set_property = *unsafe { lib.get(b"mpv_set_property\0").map_err(|e| e.to_string())? };
        let get_property = *unsafe { lib.get(b"mpv_get_property\0").map_err(|e| e.to_string())? };
        let set_property_string = *unsafe {
            lib.get(b"mpv_set_property_string\0")
                .map_err(|e| e.to_string())?
        };
        let get_property_string = *unsafe {
            lib.get(b"mpv_get_property_string\0")
                .map_err(|e| e.to_string())?
        };
        let terminate_destroy = *unsafe {
            lib.get(b"mpv_terminate_destroy\0")
                .map_err(|e| e.to_string())?
        };
        let free = *unsafe { lib.get(b"mpv_free\0").map_err(|e| e.to_string())? };
        let wait_event = *unsafe { lib.get(b"mpv_wait_event\0").map_err(|e| e.to_string())? };
        let observe_property = *unsafe {
            lib.get(b"mpv_observe_property\0")
                .map_err(|e| e.to_string())?
        };

        let render_context_create = *unsafe {
            lib.get(b"mpv_render_context_create\0")
                .map_err(|e| e.to_string())?
        };
        let render_context_set_update_callback = *unsafe {
            lib.get(b"mpv_render_context_set_update_callback\0")
                .map_err(|e| e.to_string())?
        };
        let render_context_update = *unsafe {
            lib.get(b"mpv_render_context_update\0")
                .map_err(|e| e.to_string())?
        };
        let render_context_render = *unsafe {
            lib.get(b"mpv_render_context_render\0")
                .map_err(|e| e.to_string())?
        };
        let render_context_free = *unsafe {
            lib.get(b"mpv_render_context_free\0")
                .map_err(|e| e.to_string())?
        };
        let free_node_contents = *unsafe {
            lib.get(b"mpv_free_node_contents\0")
                .map_err(|e| e.to_string())?
        };

        Ok(Self {
            _lib: lib,
            create,
            initialize,
            command,
            set_option_string,
            set_property,
            get_property,
            set_property_string,
            get_property_string,
            terminate_destroy,
            free,
            wait_event,
            observe_property,
            render_context_create,
            render_context_set_update_callback,
            render_context_update,
            render_context_render,
            render_context_free,
            free_node_contents,
        })
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

pub unsafe fn prop_to_json(format: c_int, data: *const c_void) -> Value {
    if data.is_null() {
        return Value::Null;
    }
    unsafe {
        match format {
            0 => Value::Null, // MPV_FORMAT_NONE
            1 | 2 => {
                // MPV_FORMAT_STRING | MPV_FORMAT_OSD_STRING
                let string_ptr = *(data as *const *const c_char);
                if string_ptr.is_null() {
                    Value::Null
                } else {
                    Value::String(CStr::from_ptr(string_ptr).to_string_lossy().into_owned())
                }
            }
            3 => Value::Bool(*(data as *const c_int) != 0), // MPV_FORMAT_FLAG
            4 => Value::Number((*(data as *const i64)).into()), // MPV_FORMAT_INT64
            5 => {
                // MPV_FORMAT_DOUBLE
                if let Some(n) = serde_json::Number::from_f64(*(data as *const f64)) {
                    Value::Number(n)
                } else {
                    Value::Null
                }
            }
            6 => {
                // MPV_FORMAT_NODE
                node_to_json(data as *const MpvNode)
            }
            _ => Value::Null,
        }
    }
}

pub unsafe fn node_to_json(node: *const MpvNode) -> Value {
    if node.is_null() {
        return Value::Null;
    }
    let node = unsafe { &*node };
    match node.format {
        0 => Value::Null, // MPV_FORMAT_NONE
        1 | 2 => {
            // MPV_FORMAT_STRING | MPV_FORMAT_OSD_STRING
            let string_ptr = unsafe { node.val.string };
            if string_ptr.is_null() {
                Value::Null
            } else {
                Value::String(
                    unsafe { CStr::from_ptr(string_ptr) }
                        .to_string_lossy()
                        .into_owned(),
                )
            }
        }
        3 => Value::Bool(unsafe { node.val.flag } != 0), // MPV_FORMAT_FLAG
        4 => Value::Number(unsafe { node.val.int64 }.into()), // MPV_FORMAT_INT64
        5 => {
            // MPV_FORMAT_DOUBLE
            if let Some(n) = serde_json::Number::from_f64(unsafe { node.val.double }) {
                Value::Number(n)
            } else {
                Value::Null
            }
        }
        7 => {
            // MPV_FORMAT_NODE_ARRAY
            let list = unsafe { node.val.list };
            if list.is_null() {
                Value::Null
            } else {
                let list = unsafe { &*list };
                let mut arr = Vec::new();
                for i in 0..list.num {
                    let val_ptr = unsafe { list.values.add(i as usize) };
                    arr.push(unsafe { node_to_json(val_ptr) });
                }
                Value::Array(arr)
            }
        }
        8 => {
            // MPV_FORMAT_NODE_MAP
            let list = unsafe { node.val.list };
            if list.is_null() {
                Value::Null
            } else {
                let list = unsafe { &*list };
                let mut map = serde_json::Map::new();
                for i in 0..list.num {
                    let key_ptr = unsafe { *list.keys.add(i as usize) };
                    if !key_ptr.is_null() {
                        let key = unsafe { CStr::from_ptr(key_ptr) }
                            .to_string_lossy()
                            .into_owned();
                        let val_ptr = unsafe { list.values.add(i as usize) };
                        map.insert(key, unsafe { node_to_json(val_ptr) });
                    }
                }
                Value::Object(map)
            }
        }
        _ => Value::Null,
    }
}

fn map_format_name(name: &str) -> c_int {
    match name {
        "none" => 0,
        "string" => 1,
        "flag" => 3,
        "int64" => 4,
        "double" => 5,
        "node" => 6,
        _ => 1,
    }
}

// ── Glutin Context ────────────────────────────────────────────────────────────

pub struct GlContext {
    pub display: Display,
    pub surface: glutin::surface::Surface<glutin::surface::PbufferSurface>,
    // NOT made current yet — make_current is called on the render thread.
    pub context: glutin::context::NotCurrentContext,
}

// SAFETY: GlContext is moved to the render thread before any GL calls.
// All GL operations happen exclusively on the render thread.
unsafe impl Send for GlContext {}

pub unsafe fn create_headless_context(
    display_handle: RawDisplayHandle,
    // window_handle no longer used — headless EGL contexts must not bind a wl_surface.
    _window_handle: RawWindowHandle,
) -> Result<GlContext, String> {
    let preference = {
        #[cfg(target_os = "windows")]
        {
            DisplayApiPreference::Wgl(None)
        }
        #[cfg(target_os = "macos")]
        {
            DisplayApiPreference::Cgl
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            DisplayApiPreference::Egl
        }
    };

    let display = unsafe { Display::new(display_handle, preference) }
        .map_err(|e| format!("Failed to create Glutin display: {:?}", e))?;

    let template = ConfigTemplateBuilder::new().build();
    let config = unsafe { display.find_configs(template) }
        .map_err(|e| format!("Failed to find configs: {:?}", e))?
        .next()
        .ok_or_else(|| "No compatible configs found".to_string())?;

    // IMPORTANT: Pass None here. On Wayland the window_handle is a wl_surface*;
    // binding it during context creation (not surface creation) is UB and crashes EGL.
    let context_attributes = ContextAttributesBuilder::new().build(None);

    let context = unsafe { display.create_context(&config, &context_attributes) }
        .map_err(|e| format!("Failed to create OpenGL context: {:?}", e))?;

    let surface_attributes =
        glutin::surface::SurfaceAttributesBuilder::<glutin::surface::PbufferSurface>::new().build(
            std::num::NonZero::new(1).unwrap(),
            std::num::NonZero::new(1).unwrap(),
        );
    let surface = unsafe { display.create_pbuffer_surface(&config, &surface_attributes) }
        .map_err(|e| format!("Failed to create PBuffer surface: {:?}", e))?;

    // Do NOT call make_current here — the context must be made current on the
    // render thread, not the main thread. EGL contexts have strict thread affinity.
    Ok(GlContext {
        display,
        surface,
        context,
    })
}

// ── Active Player ─────────────────────────────────────────────────────────────

pub struct ActiveMpvPlayer {
    pub handle: *mut c_void,
    pub channel: Arc<Mutex<Option<Channel>>>,
    pub signal_tx: std::sync::mpsc::SyncSender<()>,
    pub shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    pub shutdown_flag: Arc<AtomicBool>,
    pub thread_handles: Option<(thread::JoinHandle<()>, thread::JoinHandle<()>)>,
}

unsafe impl Send for ActiveMpvPlayer {}
unsafe impl Sync for ActiveMpvPlayer {}

// ── Tauri State ───────────────────────────────────────────────────────────────

pub struct MpvPlayerState {
    pub inner: Arc<Mutex<Option<ActiveMpvPlayer>>>,
    pub api: Option<Arc<MpvApi>>,
}

// ── Callbacks ─────────────────────────────────────────────────────────────────

unsafe extern "C" fn get_proc_address_callback(
    ctx: *mut c_void,
    name: *const c_char,
) -> *mut c_void {
    let display = unsafe { &*(ctx as *const Display) };
    unsafe { display.get_proc_address(CStr::from_ptr(name)) as *mut c_void }
}

unsafe extern "C" fn mpv_update_callback(ctx: *mut c_void) {
    let tx = unsafe { &*(ctx as *const std::sync::mpsc::SyncSender<()>) };
    let _ = tx.try_send(());
}

// ── Commands ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn init(
    mpvConfig: Value,
    windowLabel: String,
    state: State<'_, MpvPlayerState>,
    app: AppHandle,
) -> Result<String, String> {
    log::info!(
        "[MPV Render] Initializing player for window: {}",
        windowLabel
    );

    let api = state
        .api
        .as_ref()
        .ok_or_else(|| "libmpv API not loaded".to_string())?
        .clone();

    // Destroy any existing player first
    let mut lock = state.inner.lock();
    if let Some(mut old_player) = lock.take() {
        log::info!("[MPV Render] Stopping existing player...");
        old_player.shutdown_flag.store(true, Ordering::SeqCst);
        if let Some(shutdown_tx) = old_player.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
        if let Some((t1, t2)) = old_player.thread_handles.take() {
            let _ = t1.join();
            let _ = t2.join();
        }
    }

    // Get display and window handle
    // CRITICAL: Keep borrow guards alive so the raw pointers inside
    // RawDisplayHandle/RawWindowHandle remain valid during GL context creation.
    let window = app
        .get_webview_window(&windowLabel)
        .ok_or_else(|| format!("Window not found: {}", windowLabel))?;

    let display_handle_guard = window
        .display_handle()
        .map_err(|e| format!("Failed to get display handle: {:?}", e))?;
    let window_handle_guard = window
        .window_handle()
        .map_err(|e| format!("Failed to get window handle: {:?}", e))?;
    let display_handle = display_handle_guard.as_raw();
    let window_handle = window_handle_guard.as_raw();

    let (signal_tx, signal_rx) = std::sync::mpsc::sync_channel::<()>(1);
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let channel_ptr: Arc<Mutex<Option<Channel>>> = Arc::new(Mutex::new(None));
    let channel_ptr_clone = channel_ptr.clone();
    log::info!("[MPV Render] Calling mpv_create()...");

    // mpv_create requires LC_NUMERIC="C" to be set in the process
    #[cfg(unix)]
    unsafe {
        libc::setlocale(libc::LC_NUMERIC, b"C\0".as_ptr() as *const _);
    }

    // Raw mpv handle creation
    let handle = unsafe { (api.create)() };
    log::info!("[MPV Render] mpv_create() returned {:?}", handle);
    if handle.is_null() {
        return Err("Failed to create raw mpv handle".to_string());
    }

    let initial_options = mpvConfig
        .get("properties")
        .or_else(|| mpvConfig.get("initialOptions"))
        .and_then(|o| o.as_object());
    if let Some(initial_options) = initial_options {
        for (k, v) in initial_options {
            // DO NOT allow frontend to override `vo` or we break the Render API
            if k == "vo" {
                continue;
            }

            // Force hwdec=no for headless safety to prevent X11 crashes,
            // or use auto-safe if preferable, but let's test `no` first to guarantee no crash.
            let val_str = match v {
                Value::String(s) => s.clone(),
                Value::Bool(b) => {
                    if *b {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    }
                }
                other => other.to_string(),
            };

            let name_c = CString::new(k.as_str()).unwrap();
            let val_c = CString::new(val_str).unwrap();
            unsafe {
                (api.set_option_string)(handle, name_c.as_ptr(), val_c.as_ptr());
            }
        }
    }

    // Set vo=libmpv MUST be the very last option before initialize so it renders offscreen
    let vo_name = CString::new("vo").unwrap();
    let vo_val = CString::new("libmpv").unwrap();
    unsafe {
        (api.set_option_string)(handle, vo_name.as_ptr(), vo_val.as_ptr());
    }

    // Initialize handle
    let init_err = unsafe { (api.initialize)(handle) };
    if init_err < 0 {
        unsafe {
            (api.terminate_destroy)(handle);
        }
        return Err(format!("Failed to initialize mpv handle: {}", init_err));
    }

    // Setup observed properties
    if let Some(observed) = mpvConfig
        .get("observedProperties")
        .and_then(|o| o.as_object())
    {
        for (k, v) in observed {
            let name_c = CString::new(k.as_str()).unwrap();
            let format_str = v.as_str().unwrap_or("string");
            let format = map_format_name(format_str);
            unsafe {
                (api.observe_property)(handle, 0, name_c.as_ptr(), format);
            }
        }
    }

    let api_clone1 = api.clone();
    let api_clone2 = api.clone();
    let app_clone1 = app.clone();
    let window_label_clone2 = windowLabel.clone();

    let signal_tx_clone = signal_tx.clone();

    // Extract wl_display pointer BEFORE handle guards drop (Wayland only).
    // libmpv requires MPV_RENDER_PARAM_WL_DISPLAY = 9 on Wayland for correct
    // OpenGL initialisation; without it libmpv may try X11 paths and crash.
    #[cfg(target_os = "linux")]
    let wl_display_send: SendPtr = match display_handle {
        RawDisplayHandle::Wayland(h) => SendPtr(h.display.as_ptr() as *mut c_void),
        _ => SendPtr(std::ptr::null_mut()),
    };
    #[cfg(not(target_os = "linux"))]
    let wl_display_send: SendPtr = SendPtr(std::ptr::null_mut());

    // Create the GL context on the main thread while display/window handle guards
    // are alive. Context is NOT made current here — the render thread does that.
    let gl_context =
        unsafe { create_headless_context(display_handle, window_handle) }.map_err(|e| {
            unsafe {
                (api.terminate_destroy)(handle);
            }
            format!("Headless GL context failed: {}", e)
        })?;
    // Heap-allocate so the Display pointer given to mpv's get_proc_address callback
    // has a stable address for the lifetime of the render loop.
    let gl_context = Box::new(gl_context);
    log::info!("[MPV Render] Headless GL context created on main thread.");

    let handle_wrapper_render = MpvHandleWrapper(handle);
    let handle_wrapper_event = MpvHandleWrapper(handle);
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_render = shutdown_flag.clone();
    let shutdown_flag_event = shutdown_flag.clone();

    // Spawn Render Thread — GL context moved in as NotCurrentContext
    let render_thread = thread::spawn(move || {
        let handle_wrapper_render = handle_wrapper_render;
        let wl_display_send = wl_display_send;
        let gl_context = gl_context; // Force whole-struct capture to respect our unsafe impl Send

        log::info!("[MPV Render Thread] Starting...");

        let handle = handle_wrapper_render.0;
        let wl_display_ptr: *mut c_void = wl_display_send.0;

        // Make the EGL context current on THIS thread. This is required because
        // EGL contexts have strict thread affinity — making it current on the main
        // thread and then issuing GL calls here is undefined behaviour.
        let current_context = match gl_context.context.make_current(&gl_context.surface) {
            Ok(c) => c,
            Err(e) => {
                log::error!(
                    "[MPV Render Thread] Failed to make GL context current: {:?}",
                    e
                );
                return;
            }
        };
        // Keep current_context alive for the whole render loop.
        let _ = &current_context;

        // Load GL function pointers now that the context is current on this thread.
        gl::load_with(|symbol| {
            let symbol_c = CString::new(symbol).unwrap();
            gl_context.display.get_proc_address(&symbol_c) as *const _
        });

        let gl_init_params = MpvOpenglInitParams {
            get_proc_address: Some(get_proc_address_callback),
            get_proc_address_ctx: &gl_context.display as *const _ as *mut c_void,
            extra_exts: ptr::null(),
        };

        let api_type_c = CString::new("opengl").unwrap();
        let api_type_param = MpvRenderParam {
            type_: 1, // MPV_RENDER_PARAM_API_TYPE
            data: api_type_c.as_ptr() as *mut c_void,
        };
        let init_params_param = MpvRenderParam {
            type_: 2, // MPV_RENDER_PARAM_OPENGL_INIT_PARAMS
            data: &gl_init_params as *const _ as *mut c_void,
        };
        // MPV_RENDER_PARAM_WL_DISPLAY = 9, required on Wayland.
        // The pointer is null on non-Wayland/non-Linux so libmpv ignores it.
        let wl_display_param = MpvRenderParam {
            type_: if !wl_display_ptr.is_null() { 9 } else { 0 },
            data: wl_display_ptr,
        };

        // Build param list. Terminate with type_=0.
        // If wl_display is null we emit a no-op (type_=0) sentinel early and
        // the real sentinel closes the array.
        let mut params = if !wl_display_ptr.is_null() {
            [
                api_type_param,
                init_params_param,
                wl_display_param,
                MpvRenderParam {
                    type_: 0,
                    data: ptr::null_mut(),
                },
            ]
        } else {
            [
                api_type_param,
                init_params_param,
                MpvRenderParam {
                    type_: 0,
                    data: ptr::null_mut(),
                },
                MpvRenderParam {
                    type_: 0,
                    data: ptr::null_mut(),
                }, // padding slot
            ]
        };

        let mut render_ctx: *mut c_void = ptr::null_mut();
        let err = unsafe {
            (api_clone1.render_context_create)(&mut render_ctx, handle, params.as_mut_ptr())
        };
        if err < 0 {
            log::error!(
                "[MPV Render Thread] Failed to create render context: {}",
                err
            );
            return;
        }

        // Signal Sender Box — MUST be leaked so mpv's C callback always has a valid pointer.
        // We reclaim it during cleanup below to avoid a true leak.
        let signal_tx_box = Box::leak(Box::new(signal_tx_clone));
        unsafe {
            (api_clone1.render_context_set_update_callback)(
                render_ctx,
                Some(mpv_update_callback),
                signal_tx_box as *const _ as *mut c_void,
            );
        }

        let mut fbo = 0;
        let mut texture = 0;
        let mut current_w = 0;
        let mut current_h = 0;

        let width_name = CString::new("width").unwrap();
        let height_name = CString::new("height").unwrap();

        log::info!("[MPV Render Thread] Render context created successfully.");

        loop {
            // Check shutdown signal non-blocking
            if shutdown_flag_render.load(Ordering::Relaxed) {
                break;
            }

            // Wait for render signal
            match signal_rx.recv_timeout(Duration::from_millis(8)) {
                Ok(()) => {
                    // Update mpv render context
                    let update_flags = unsafe { (api_clone1.render_context_update)(render_ctx) };

                    // MPV_RENDER_UPDATE_FRAME is 1. Render only if the frame update flag is set.
                    if (update_flags & 1) == 0 {
                        continue;
                    }

                    // Update and Render!
                    let mut w: i64 = 0;
                    let mut h: i64 = 0;
                    unsafe {
                        (api_clone1.get_property)(
                            handle,
                            width_name.as_ptr(),
                            4,
                            &mut w as *mut _ as *mut c_void,
                        );
                        (api_clone1.get_property)(
                            handle,
                            height_name.as_ptr(),
                            4,
                            &mut h as *mut _ as *mut c_void,
                        );
                    }

                    if w > 0 && h > 0 {
                        // No resolution limiter. w and h are used directly.

                        if w != current_w || h != current_h {
                            log::info!(
                                "[MPV Render Thread] Re-allocating FBO/texture size: {}x{}",
                                w,
                                h
                            );
                            current_w = w;
                            current_h = h;

                            unsafe {
                                if fbo != 0 {
                                    gl::DeleteFramebuffers(1, &fbo);
                                }
                                if texture != 0 {
                                    gl::DeleteTextures(1, &texture);
                                }

                                gl::GenFramebuffers(1, &mut fbo);
                                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

                                gl::GenTextures(1, &mut texture);
                                gl::BindTexture(gl::TEXTURE_2D, texture);
                                gl::TexImage2D(
                                    gl::TEXTURE_2D,
                                    0,
                                    gl::RGBA8 as i32,
                                    w as i32,
                                    h as i32,
                                    0,
                                    gl::RGBA,
                                    gl::UNSIGNED_BYTE,
                                    ptr::null(),
                                );

                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MIN_FILTER,
                                    gl::LINEAR as i32,
                                );
                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MAG_FILTER,
                                    gl::LINEAR as i32,
                                );

                                gl::FramebufferTexture2D(
                                    gl::FRAMEBUFFER,
                                    gl::COLOR_ATTACHMENT0,
                                    gl::TEXTURE_2D,
                                    texture,
                                    0,
                                );

                                let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
                                if status != gl::FRAMEBUFFER_COMPLETE {
                                    log::info!(
                                        "[MPV Render Thread] Framebuffer is not complete: {}",
                                        status
                                    );
                                }
                            }
                        }

                        let opengl_fbo = MpvOpenglFbo {
                            fbo: fbo as i32,
                            w: w as i32,
                            h: h as i32,
                            format: 0x8058, // GL_RGBA8
                        };

                        let fbo_param = MpvRenderParam {
                            type_: 3, // MPV_RENDER_PARAM_FB_INFO
                            data: &opengl_fbo as *const _ as *mut c_void,
                        };
                        let flip_y_val: c_int = 1;
                        let flip_y_param = MpvRenderParam {
                            type_: 4, // MPV_RENDER_PARAM_FLIP_Y
                            data: &flip_y_val as *const _ as *mut c_void,
                        };
                        let mut render_params = [
                            fbo_param,
                            flip_y_param,
                            MpvRenderParam {
                                type_: 0,
                                data: ptr::null_mut(),
                            },
                        ];

                        unsafe {
                            let render_err = (api_clone1.render_context_render)(
                                render_ctx,
                                render_params.as_mut_ptr(),
                            );
                            if render_err >= 0 {
                                // Read pixels
                                let rgba_len = (w * h * 4) as usize;
                                // Allocate message with 8-byte prefix (width and height as u32 little endian)
                                let mut message = vec![0u8; 8 + rgba_len];

                                let w_u32 = w as u32;
                                let h_u32 = h as u32;
                                message[0..4].copy_from_slice(&w_u32.to_le_bytes());
                                message[4..8].copy_from_slice(&h_u32.to_le_bytes());

                                gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
                                gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
                                gl::ReadPixels(
                                    0,
                                    0,
                                    w as i32,
                                    h as i32,
                                    gl::RGBA,
                                    gl::UNSIGNED_BYTE,
                                    message[8..].as_mut_ptr() as *mut c_void,
                                );

                                // Send to channel if available
                                // Send to channel if available
                                if let Some(chan) = &*channel_ptr_clone.lock() {
                                    let chan_clone = chan.clone();
                                    tauri::async_runtime::spawn(async move {
                                        let _ = chan_clone
                                            .send(tauri::ipc::InvokeResponseBody::Raw(message));
                                    });
                                }
                            } else {
                                log::info!(
                                    "[MPV Render Thread] mpv_render_context_render failed: {}",
                                    render_err
                                );
                            }
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Check shutdown and proceed
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }

        log::info!("[MPV Render Thread] Shutting down, cleaning up GL resources...");
        // Cleanup context
        unsafe {
            // Clear the callback first, so mpv stops calling into our sender
            (api_clone1.render_context_set_update_callback)(render_ctx, None, ptr::null_mut());
            // Reclaim the leaked sender to free it properly
            drop(Box::from_raw(
                signal_tx_box as *mut std::sync::mpsc::SyncSender<()>,
            ));
            (api_clone1.render_context_free)(render_ctx);
            if fbo != 0 {
                gl::DeleteFramebuffers(1, &fbo);
            }
            if texture != 0 {
                gl::DeleteTextures(1, &texture);
            }
        }

        log::info!("[MPV Render Thread] Stopped.");
    });

    // Spawn Event Thread (Observe properties & other event loop stuff)
    let event_thread = thread::spawn(move || {
        log::info!("[MPV Event Thread] Starting event loop...");
        let hw = handle_wrapper_event;
        let handle = hw.0;
        loop {
            // Check shutdown flag before blocking wait
            if shutdown_flag_event.load(Ordering::Relaxed) {
                break;
            }
            let event_ptr = unsafe { (api_clone2.wait_event)(handle, 0.05) };
            if event_ptr.is_null() {
                continue;
            }
            // Re-check after blocking call — handle may have been destroyed
            if shutdown_flag_event.load(Ordering::Relaxed) {
                break;
            }

            let event = unsafe { &*event_ptr };
            if event.event_id == 1 {
                // MPV_EVENT_SHUTDOWN
                break;
            }

            // Map event to JSON payload
            let event_name = format!("mpv-event-{}", window_label_clone2);
            let mut payload = serde_json::Map::new();

            match event.event_id {
                8 => {
                    // MPV_EVENT_FILE_LOADED
                    payload.insert(
                        "event".to_string(),
                        Value::String("file-loaded".to_string()),
                    );
                    let _ = app_clone1.emit(&event_name, payload);
                }
                21 => {
                    // MPV_EVENT_PLAYBACK_RESTART
                    payload.insert(
                        "event".to_string(),
                        Value::String("playback-restart".to_string()),
                    );
                    let _ = app_clone1.emit(&event_name, payload);
                }
                7 => {
                    // MPV_EVENT_END_FILE
                    payload.insert("event".to_string(), Value::String("end-file".to_string()));
                    let mut data_map = serde_json::Map::new();
                    data_map.insert("reason".to_string(), Value::String("eof".to_string()));
                    payload.insert("data".to_string(), Value::Object(data_map));
                    let _ = app_clone1.emit(&event_name, payload);
                }
                22 => {
                    // MPV_EVENT_PROPERTY_CHANGE
                    let prop = unsafe { &*(event.data as *const MpvEventProperty) };
                    if !prop.name.is_null() {
                        let prop_name = unsafe { CStr::from_ptr(prop.name) }
                            .to_string_lossy()
                            .into_owned();
                        let json_val = unsafe { prop_to_json(prop.format, prop.data) };
                        payload.insert(
                            "event".to_string(),
                            Value::String("property-change".to_string()),
                        );
                        payload.insert("name".to_string(), Value::String(prop_name));
                        payload.insert("data".to_string(), json_val);
                        let _ = app_clone1.emit(&event_name, payload);
                    }
                }
                _ => {}
            }
        }
        log::info!("[MPV Event Thread] Stopped.");
    });

    // Save state
    *lock = Some(ActiveMpvPlayer {
        handle,
        channel: channel_ptr,
        signal_tx,
        shutdown_tx: Some(shutdown_tx),
        shutdown_flag,
        thread_handles: Some((render_thread, event_thread)),
    });

    Ok(windowLabel)
}

#[tauri::command]
pub fn destroy(windowLabel: String, state: State<'_, MpvPlayerState>) -> Result<(), String> {
    log::info!("[MPV Render] Destroying player for window: {}", windowLabel);
    let mut lock = state.inner.lock();
    if let Some(mut player) = lock.take() {
        // Signal shutdown to both threads via AtomicBool
        player.shutdown_flag.store(true, Ordering::SeqCst);

        if let Some(shutdown_tx) = player.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        // MUST join threads BEFORE destroying the mpv handle.
        // The render thread cleans up the render_context internally,
        // and the event thread stops processing events.
        if let Some((t1, t2)) = player.thread_handles.take() {
            let _ = t1.join();
            let _ = t2.join();
        }

        // Now safe — no threads are using the handle
        let api = state
            .api
            .as_ref()
            .ok_or_else(|| "libmpv API not loaded".to_string())?;
        unsafe {
            (api.terminate_destroy)(player.handle);
        }
    }
    Ok(())
}

#[tauri::command]
pub fn command(
    name: String,
    args: Vec<Value>,
    windowLabel: String,
    state: State<'_, MpvPlayerState>,
) -> Result<(), String> {
    let lock = state.inner.lock();
    let player = lock
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    let api = state
        .api
        .as_ref()
        .ok_or_else(|| "libmpv API not loaded".to_string())?;

    log::info!(
        "[MPV Render] Sending command '{}' to window: {}",
        name,
        windowLabel
    );

    let mut c_strs = Vec::new();
    c_strs.push(CString::new(name).unwrap());
    for arg in args {
        let s = match arg {
            Value::String(s) => s,
            other => other.to_string(),
        };
        c_strs.push(CString::new(s).unwrap());
    }

    let mut ptrs: Vec<*const c_char> = c_strs.iter().map(|c| c.as_ptr()).collect();
    ptrs.push(ptr::null());

    let err = unsafe { (api.command)(player.handle, ptrs.as_mut_ptr()) };
    if err < 0 {
        return Err(format!("Command failed with code: {}", err));
    }
    Ok(())
}

#[tauri::command]
pub fn set_property(
    name: String,
    value: Value,
    windowLabel: String,
    state: State<'_, MpvPlayerState>,
) -> Result<(), String> {
    let lock = state.inner.lock();
    let player = lock
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    let api = state
        .api
        .as_ref()
        .ok_or_else(|| "libmpv API not loaded".to_string())?;

    log::info!(
        "[MPV Render] Setting property '{}' = {} on window: {}",
        name,
        value,
        windowLabel
    );

    let s = match value {
        Value::String(s) => s,
        other => other.to_string(),
    };

    let name_c = CString::new(name).unwrap();
    let val_c = CString::new(s).unwrap();

    let err = unsafe { (api.set_property_string)(player.handle, name_c.as_ptr(), val_c.as_ptr()) };
    if err < 0 {
        return Err(format!("Set property failed with code: {}", err));
    }
    Ok(())
}

#[tauri::command]
pub fn get_property(
    name: String,
    _format: Option<String>,
    _windowLabel: String,
    state: State<'_, MpvPlayerState>,
) -> Result<Value, String> {
    let lock = state.inner.lock();
    let player = lock
        .as_ref()
        .ok_or_else(|| "Player not initialized".to_string())?;
    let api = state
        .api
        .as_ref()
        .ok_or_else(|| "libmpv API not loaded".to_string())?;

    let name_c = CString::new(name).unwrap();

    let mut node = MpvNode {
        val: MpvNodeVal { int64: 0 },
        format: 0,
    };

    let err = unsafe {
        (api.get_property)(
            player.handle,
            name_c.as_ptr(),
            6,
            &mut node as *mut _ as *mut c_void,
        )
    };
    if err < 0 {
        let val_ptr = unsafe { (api.get_property_string)(player.handle, name_c.as_ptr()) };
        if val_ptr.is_null() {
            return Err(format!("Get property failed with code: {}", err));
        }
        let val = unsafe { CStr::from_ptr(val_ptr).to_string_lossy().into_owned() };
        unsafe {
            (api.free)(val_ptr as *mut c_void);
        }
        return Ok(Value::String(val));
    }

    let json_val = unsafe { node_to_json(&node) };
    unsafe {
        (api.free_node_contents)(&mut node);
    }

    Ok(json_val)
}

#[tauri::command]
pub fn start_mpv_frame_stream(
    onFrame: Channel,
    state: State<'_, MpvPlayerState>,
) -> Result<(), String> {
    log::info!("[MPV Render] Registering frame stream channel");
    let lock = state.inner.lock();
    if let Some(player) = &*lock {
        let mut chan_lock = player.channel.lock();
        *chan_lock = Some(onFrame);
        Ok(())
    } else {
        Err("Player not initialized".to_string())
    }
}
