# Context Menu & UI Scale Features

## Overview

This document describes the custom context menu system and UI scale feature implemented in Zafkiel.

## ✨ Features Implemented

### 1. Custom Context Menu System

A fully-featured right-click context menu that works globally throughout the application.

**Key Features:**

- 🎨 **Themed**: Automatically matches app theme (light/dark mode)
- 🌐 **Global**: Works anywhere in the app with a default menu
- 🔧 **Customizable**: Easy to add custom menus to specific elements
- 🎯 **Solar Icons**: Professional icons from Iconify Solar icon set
- ⚡ **Smart Positioning**: Auto-adjusts to stay within viewport
- 🔍 **Inspect Element**: Opens developer tools (Tauri desktop only)

**Default Context Menu Items:**

- **Reload**: Refresh the current page (⌘R)
- **Back**: Navigate to previous page
- **Forward**: Navigate to next page
- **Inspect Element**: Open devtools (⌘⌥I)

**Technical Implementation:**

- `ContextMenu.svelte`: Main menu component with Solar icon support
- `ContextMenuProvider.svelte`: Global event handler
- `context-menu.ts`: Global state management store
- `useContextMenu.ts`: Composable hooks for easy integration
- `default-context-menu.ts`: Default menu items

**Usage Example:**

```svelte
<script lang="ts">
	import { useContextMenu } from '$lib/hooks/useContextMenu';

	const items = [
		{
			id: 'copy',
			label: 'Copy',
			icon: 'solar:copy-bold',
			shortcut: '⌘C',
			onClick: () => console.log('Copy'),
		},
	];

	const { onContextMenu } = useContextMenu(items);
</script>

<div oncontextmenu={onContextMenu} data-has-context-menu>Right-click me!</div>
```

### 2. UI Scale Feature

A new configuration option to scale the entire UI from 50% to 200%.

**Key Features:**

- 🎚️ **Adjustable Range**: Scale from 50% (smaller) to 200% (larger)
- 💾 **Persistent**: Saved in config and applied on app start
- ⚡ **Real-time**: Changes apply immediately
- 🔄 **Reset Button**: Quick reset to default 100%

**Technical Implementation:**

- **Backend** (`src-tauri/`):
  - Added `ui_scale: f32` field to `UiConfig` struct
  - Created `update_ui_scale()` command in Rust
  - Config stored in `~/.config/zafkiel/config.ron`

- **Frontend** (`src/lib/`):
  - Updated `ConfigService.ts` with `updateUiScale()` method
  - Created `useUiScale.svelte.ts` hook for easy integration
  - Applied CSS zoom to `document.documentElement`
  - Integrated into `+layout.svelte` for automatic loading

**Configuration:**

```ron
// ~/.config/zafkiel/config.ron
(
  anilist: (access_token: None),
  security: (encryption_key: ""),
  ui: (
    theme: "catppuccin",
    glow_effects: true,
    animations: true,
    smooth_scroll: true,
    ui_scale: 1.0,  // 50% to 200% (0.5 to 2.0)
  ),
)
```

**Usage Example:**

```svelte
<script lang="ts">
	import { useUiScale } from '$lib/hooks/useUiScale';

	const uiScale = useUiScale();

	// Read current scale
	console.log(uiScale.scale); // 1.0 (100%)

	// Update scale
	await uiScale.setScale(1.5); // 150%

	// Reset to default
	await uiScale.resetScale(); // 100%
</script>
```

## 🎯 How to Use

### Testing the Context Menu

1. **Global Menu**: Right-click anywhere in the app to see the default menu
   - Try "Reload", "Back", "Forward"
   - Try "Inspect Element" to open devtools (desktop only)

2. **Custom Menu**: Visit `/context-menu-demo` to see custom context menus
   - Right-click on different cards to see different menus
   - All menus now use Solar icons instead of emojis

### Testing UI Scale

1. **Main Page**: Visit the homepage (`/`)
2. **UI Scale Card**: Use the slider to adjust from 50% to 200%
3. **Real-time Changes**: Watch the entire UI scale instantly
4. **Persistence**: Reload the page - your scale setting is remembered
5. **Reset**: Click "Reset to Default" to go back to 100%

## 🛠️ Tauri Commands Added

### `open_devtools`

Opens the developer tools window (only works in Tauri desktop app).

```typescript
await ConfigService.openDevtools();
```

**Behavior:**

- **Debug builds**: Always works
- **Production builds**: Only works if devtools are enabled in `tauri.conf.json`
- **Web mode**: Silently fails (no error thrown)

### `update_ui_scale`

Updates the UI scale factor and saves to config.

```typescript
await ConfigService.updateUiScale(1.25); // 125%
```

**Parameters:**

- `scale: number` - Scale factor from 0.5 to 2.0 (automatically clamped)

## 📁 Files Modified/Created

### New Files:

- `src/lib/hooks/useUiScale.svelte.ts` - UI scale hook
- `src/lib/utils/default-context-menu.ts` - Default menu items
- `src/lib/providers/context-menu.svelte` - Global context menu provider
- `docs/CONTEXT_MENU_FEATURES.md` - This documentation

### Modified Files:

- `src-tauri/src/config/types.rs` - Added `ui_scale` field
- `src-tauri/src/config/loader.rs` - Added `update_ui_scale()` method
- `src-tauri/src/commands.rs` - Added `update_ui_scale()` and `open_devtools()` commands
- `src-tauri/src/lib.rs` - Registered new commands
- `src/lib/services/config.ts` - Added UI scale and devtools methods
- `src/lib/components/ContextMenu.svelte` - Added Solar icon support
- `src/lib/hooks/useContextMenu.ts` - Updated documentation
- `src/routes/+layout.svelte` - Integrated UI scale hook
- `src/routes/+page.svelte` - Added UI scale demo
- `src/routes/context-menu-demo/+page.svelte` - Updated to Solar icons
- `src/stories/ContextMenu.story.svelte` - Updated to Solar icons
- `docs/CONTEXT_MENU.md` - Updated documentation with Solar icons

## 🎨 Solar Icons

All context menu items now use professional icons from the Iconify Solar icon set.

**Examples:**

- `solar:copy-bold` - Copy action
- `solar:clipboard-bold` - Paste action
- `solar:trash-bin-trash-bold` - Delete action
- `solar:eye-bold` - View/Show action
- `solar:code-bold` - Inspect Element
- `solar:refresh-bold` - Reload
- `solar:arrow-left-bold` - Back
- `solar:arrow-right-bold` - Forward

Browse all icons at: https://icones.js.org/collection/solar

**Icon Format:**

```typescript
{
  id: 'copy',
  label: 'Copy',
  icon: 'solar:copy-bold',  // Iconify format
  onClick: () => { /* ... */ }
}
```

## 🔒 Type Safety

All new features are fully typed:

```typescript
interface UiConfig {
	theme: string;
	glow_effects: boolean;
	animations: boolean;
	smooth_scroll: boolean;
	ui_scale: number; // NEW: 0.5 to 2.0
}

interface ContextMenuItem {
	id: string;
	label: string;
	icon?: string; // Supports Iconify icons and HTML
	shortcut?: string;
	disabled?: boolean;
	separator?: boolean;
	onClick?: () => void | Promise<void>;
}
```

## 🚀 Performance

- **UI Scale**: Uses CSS `zoom` for native browser performance
- **Context Menu**: Lazy-rendered only when opened
- **Icons**: Loaded on-demand from `@iconify/svelte`
- **Config**: Cached in Svelte stores, saved to disk only on changes

## 🎯 Best Practices

1. **Always use Solar icons** for consistency
2. **Add `data-has-context-menu` attribute** to elements with custom menus
3. **Use descriptive menu item IDs** for debugging
4. **Provide keyboard shortcuts** for common actions
5. **Clamp UI scale** to reasonable bounds (0.5-2.0)
6. **Test both light and dark themes** when adding context menus

## 📝 Future Enhancements

Potential improvements:

- [ ] Custom keyboard shortcut handler integration
- [ ] Nested/submenu support
- [ ] Context menu positioning preferences (left/right/top/bottom)
- [ ] Per-element UI scale (zoom specific components)
- [ ] UI scale presets (Small/Medium/Large/Extra Large)
- [ ] Context menu item search/filter
- [ ] Recent items in context menu
- [ ] Context menu animations

## 🐛 Known Limitations

1. **Inspect Element**: Only works in Tauri desktop app, not in browser
2. **UI Scale**: Uses CSS zoom which may affect some CSS calculations
3. **Context Menu**: Cannot override browser context menu on some system elements (like inputs)

## ✅ Testing Checklist

- [x] Context menu works globally (right-click anywhere)
- [x] Context menu shows default items (Reload, Back, Forward, Inspect)
- [x] Inspect Element opens devtools
- [x] Custom context menus work on demo page
- [x] Solar icons render correctly
- [x] UI scale adjusts entire interface
- [x] UI scale persists after reload
- [x] UI scale slider updates in real-time
- [x] Reset button returns to 100%
- [x] Config saved to disk
- [x] No TypeScript errors
- [x] No console errors
- [x] Works in light and dark mode

---

**Documentation updated**: October 7, 2025
**Version**: 1.0.0
**Features**: Context Menu System + UI Scale Configuration
