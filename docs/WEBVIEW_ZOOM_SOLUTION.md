# UI Scaling with Tauri Webview Zoom

## ✅ Final Solution: Native Webview Zoom

After testing CSS transform scaling, we discovered it doesn't scale **native browser elements** like:

- Default context menus (right-click menus)
- Browser dialogs and alerts
- Scrollbars
- Form controls (in some browsers)
- Developer tools overlays

**Solution:** Use Tauri's `WebviewWindow::set_zoom()` API to scale the **entire webview** at the OS level.

## How It Works

### 1. Tauri Backend (Rust)

**File: `src-tauri/src/commands.rs`**

```rust
/// Update UI scale factor and apply webview zoom
#[tauri::command]
pub fn update_ui_scale(scale: f32, config: State<ConfigState>, app: tauri::AppHandle) -> ConfigResponse<()> {
    let clamped_scale = scale.max(0.5).min(2.0);

    // Save to config
    match config.update_ui_scale(clamped_scale) {
        Ok(_) => {
            // Apply zoom to webview
            if let Some(window) = app.get_webview_window("main") {
                window.set_zoom(clamped_scale as f64)?;
            }
            ConfigResponse::success(())
        }
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}

/// Set webview zoom level on startup
#[tauri::command]
pub fn apply_ui_scale(app: tauri::AppHandle, config: State<ConfigState>) -> ConfigResponse<()> {
    match config.get_ui_config() {
        Ok(ui_config) => {
            if let Some(window) = app.get_webview_window("main") {
                window.set_zoom(ui_config.ui_scale as f64)?;
            }
            ConfigResponse::success(())
        }
        Err(e) => ConfigResponse::error(e.to_string()),
    }
}
```

### 2. Frontend Service (TypeScript)

**File: `src/lib/services/config.ts`**

```typescript
/**
 * Update UI scale factor (0.5 to 2.0) and apply webview zoom
 */
static async updateUiScale(scale: number): Promise<void> {
    const response = await invoke<ConfigResponse<void>>('update_ui_scale', { scale });
    if (!response.success) {
        throw new Error(response.error || 'Failed to update UI scale');
    }
}

/**
 * Apply UI scale from config (call on app startup)
 */
static async applyUiScale(): Promise<void> {
    const response = await invoke<ConfigResponse<void>>('apply_ui_scale');
    if (!response.success) {
        throw new Error(response.error || 'Failed to apply UI scale');
    }
}
```

### 3. Frontend Hook (Svelte 5)

**File: `src/lib/hooks/useUiScale.svelte.ts`**

```typescript
export function useUiScale() {
	let currentScale = $state(1.0);
	let isInitialized = $state(false);

	// Apply zoom on app startup
	$effect(() => {
		if (typeof window !== 'undefined' && !isInitialized) {
			isInitialized = true;

			ConfigService.applyUiScale()
				.then(() => ConfigService.getUiConfig())
				.then((config) => {
					currentScale = config.ui_scale;
				})
				.catch((error) => {
					console.warn('Failed to apply UI scale:', error);
				});
		}
	});

	async function setScale(scale: number): Promise<void> {
		const clampedScale = Math.max(0.5, Math.min(2.0, scale));
		await ConfigService.updateUiScale(clampedScale);
		currentScale = clampedScale;
	}

	return {
		get scale() {
			return currentScale;
		},
		setScale,
		resetScale: () => setScale(1.0),
	};
}
```

## Benefits of Webview Zoom

### ✅ Scales Everything

- ✅ All HTML/CSS content
- ✅ Native context menus (right-click)
- ✅ Browser form controls
- ✅ Scrollbars
- ✅ Developer tools
- ✅ All JavaScript-rendered content
- ✅ Canvas and WebGL content

### ✅ No Positioning Issues

- ✅ Floating UI calculates correctly
- ✅ No transform coordinate problems
- ✅ Dropdowns position perfectly
- ✅ Tooltips work as expected
- ✅ No need for portal workarounds

### ✅ Better Performance

- ✅ GPU-accelerated at OS level
- ✅ No JavaScript calculations needed
- ✅ No CSS transform overhead
- ✅ Handled by native webview

### ✅ Simpler Code

- ❌ No CSS transform wrappers
- ❌ No portal positioning fixes
- ❌ No manual scale application
- ❌ No --ui-scale CSS variables
- ✅ Single API call

## What Was Changed

### Removed

1. **CSS Transform Scaling** - `#app-scale-wrapper` div removed
2. **CSS Variables** - `--ui-scale` no longer needed
3. **Inline Styles** - No more `transform: scale()` on dropdowns
4. **Portal Positioning** - No more `to="#app-scale-wrapper"`
5. **+layout.ts Preload** - No longer needed
6. **app.html Inline Script** - Removed scale preloading

### Added

1. **Tauri Command** - `update_ui_scale` with zoom application
2. **Tauri Command** - `apply_ui_scale` for startup
3. **Service Method** - `ConfigService.applyUiScale()`
4. **Hook Effect** - Calls `applyUiScale()` on mount

### Fixed

1. **Overflow Issue** - Changed `overflow-hidden` to `overflow-y-auto` in dropdowns
2. **Submenu Clipping** - Submenus no longer cut off
3. **Native Elements** - Now scale with everything else

## Migration from CSS Scaling

If you have existing CSS scaling code:

```svelte
<!-- ❌ OLD: CSS Transform Scaling -->
<div style="transform: scale(var(--ui-scale))">
	<Portal to="#app-scale-wrapper">
		<Dropdown />
	</Portal>
</div>
```

```svelte
<!-- ✅ NEW: Webview Zoom (no changes needed!) -->
<div>
	<Portal>
		<Dropdown />
	</Portal>
</div>
```

Just remove all scaling code and call `ConfigService.applyUiScale()` on startup!

## Testing

1. **Start the app** - Scale is automatically applied from config
2. **Change scale** - Use the slider in UI settings
3. **Test dropdowns** - Position correctly at all scales
4. **Right-click** - Native context menus scale too
5. **Open dev tools** - Even dev tools are scaled!

## API Reference

### Tauri Commands

```rust
// Apply scale and save to config
update_ui_scale(scale: f32, config: State<ConfigState>, app: AppHandle) -> ConfigResponse<()>

// Apply scale from saved config (on startup)
apply_ui_scale(app: AppHandle, config: State<ConfigState>) -> ConfigResponse<()>
```

### Frontend Service

```typescript
// Update scale (saves + applies zoom)
await ConfigService.updateUiScale(1.5);

// Apply scale from config
await ConfigService.applyUiScale();
```

### Svelte Hook

```typescript
const uiScale = useUiScale();

// Get current scale
console.log(uiScale.scale); // 1.0 - 2.0

// Set new scale
await uiScale.setScale(1.5);

// Reset to default
await uiScale.resetScale();
```

## Browser vs Tauri

This solution is **Tauri-specific**. In a regular browser:

- `window.set_zoom()` doesn't exist
- Fall back to CSS `transform: scale()` or `zoom` property
- Or just don't support UI scaling in web mode

## Performance

Webview zoom is **GPU-accelerated** at the OS level:

- Faster than CSS transforms
- No JavaScript overhead
- Native rendering pipeline
- Optimal memory usage

## Conclusion

**Webview zoom is the correct solution for Tauri apps** because it:

1. Scales everything including native elements
2. Has no positioning issues
3. Is faster and simpler
4. Provides better UX

CSS scaling should only be used in web browsers where webview zoom isn't available.
