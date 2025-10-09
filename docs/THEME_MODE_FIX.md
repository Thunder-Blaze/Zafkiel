# Theme Mode Management Fix

## Problem Statement

The theme mode (light/dark/system) was experiencing two critical issues:

1. **Late Loading**: Theme mode was loading significantly later than the theme itself, causing a visible flash where the wrong color mode would briefly appear before being corrected
2. **Mode Reversion**: When changing themes, the theme mode would revert to system preference instead of persisting the user's explicit choice

## Root Causes

### 1. Separated Initialization

- **Theme Store**: Initialized in `+layout.svelte` during `onMount()`, loading theme immediately
- **Theme Mode**: Loaded separately in individual components via `$effect()`, which runs after mount
- This created a timing gap where theme loaded first with default system preference, then theme_mode applied later

### 2. Conflicting State Management

- Theme store had its own `setDarkMode()` method that directly managed the `dark` class
- Components had duplicate `applyThemeMode()` functions
- Theme mode config was stored in backend but not synchronized with theme store state
- When switching themes, the store's system preference detection would override the user's theme_mode preference

## Solution Architecture

### Centralized Theme Mode Management

Made the **theme store** the single source of truth for theme mode by:

1. **Loading theme_mode alongside theme initialization**
2. **Storing theme_mode in theme store state**
3. **Providing `setThemeMode()` and `applyThemeMode()` methods in theme store**
4. **Removing duplicate logic from components**

## Implementation Details

### 1. Theme Store (`src/lib/stores/theme.svelte.ts`)

#### Added to State Interface

```typescript
interface ThemeState {
	currentTheme: string;
	isDark: boolean;
	themeMode: 'light' | 'dark' | 'system'; // NEW
	availableThemes: Theme[];
	loadedThemes: SvelteSet<string>;
	isLoading: boolean;
	initialized: boolean;
}
```

#### Enhanced Initialization

```typescript
async initialize() {
    // Load theme_mode from config FIRST
    const config = await ConfigService.getUiConfig();
    state.themeMode = config.theme_mode;

    // Determine isDark based on theme_mode
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    if (state.themeMode === 'system') {
        state.isDark = prefersDark;
    } else {
        state.isDark = state.themeMode === 'dark';
    }

    // Apply theme mode to document IMMEDIATELY
    this.applyThemeMode(state.themeMode);

    // Then initialize theme manager with correct isDark value
    await themeManager.initialize(state.isDark);

    // ... rest of initialization
}
```

#### Added Public Methods

```typescript
/**
 * Apply theme mode to document
 * Single source of truth for light/dark mode classes
 */
applyThemeMode(mode: 'light' | 'dark' | 'system') {
    const root = document.documentElement;

    if (mode === 'system') {
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        root.classList.remove('light', 'dark');
        root.classList.add(prefersDark ? 'dark' : 'light');
        state.isDark = prefersDark;
    } else {
        root.classList.remove('light', 'dark');
        root.classList.add(mode);
        state.isDark = mode === 'dark';
    }
}

/**
 * Update theme mode and persist to config
 */
async setThemeMode(mode: 'light' | 'dark' | 'system') {
    if (state.themeMode === mode) return;

    try {
        await ConfigService.updateThemeMode(mode);
        state.themeMode = mode;
        this.applyThemeMode(mode);
        console.log('[ThemeStore] ✓ Theme mode updated to:', mode);
    } catch (error) {
        console.error('[ThemeStore] ✗ Failed to update theme mode:', error);
        throw error;
    }
}
```

#### Added Getter

```typescript
get themeMode() {
    return state.themeMode;
}
```

### 2. Interface Settings (`src/lib/components/settings/InterfaceSettings.svelte`)

**Before:**

```typescript
let themeMode = $state<'light' | 'dark' | 'system'>('dark');

// Load from config
$effect(() => {
	ConfigService.getUiConfig().then((config) => {
		themeMode = config.theme_mode;
	});
});

async function handleThemeModeChange(mode) {
	await ConfigService.updateThemeMode(mode);
	themeMode = mode;
	applyThemeMode(mode); // Local function
}

function applyThemeMode(mode) {
	// Duplicate logic
}
```

**After:**

```typescript
// No local state needed!
import { themeStore } from '$lib/stores/theme.svelte';

async function handleThemeModeChange(mode: 'light' | 'dark' | 'system') {
    try {
        await themeStore.setThemeMode(mode);  // Delegates to store
        const modeText = mode === 'system' ? 'system preference' : `${mode} mode`;
        toast.success(`Theme mode set to ${modeText}`);
    } catch (error) {
        toast.error('Failed to update theme mode');
    }
}

// Use themeStore.themeMode directly in template
<Button variant={themeStore.themeMode === 'light' ? 'default' : 'outline'} ...>
```

### 3. Theme Switcher (`src/lib/components/ThemeSwitcher.svelte`)

**Same simplification as Interface Settings:**

**Before:**

- Separate `themeMode` state
- Duplicate `applyThemeMode()` logic
- `$effect()` to load from config
- Direct `ConfigService.updateThemeMode()` calls

**After:**

- Uses `themeStore.themeMode` directly
- Calls `themeStore.setThemeMode()` to update
- No duplicate state or logic

## Benefits

### 1. ✅ Synchronized Loading

- Theme and theme_mode load together during app initialization
- No visible flash or delay between theme and color mode application
- Single `onMount()` in `+layout.svelte` handles everything

### 2. ✅ Persistent Theme Mode

- Theme mode is stored in theme store state
- Survives theme switches
- System preference listener only activates when `theme_mode === 'system'`

### 3. ✅ Single Source of Truth

- All theme mode logic centralized in theme store
- Components just delegate to store methods
- No duplicate state management
- Reduced code complexity

### 4. ✅ Better Developer Experience

- Clear ownership: theme store manages both theme and theme_mode
- Components are simpler, just UI containers
- Type-safe throughout with TypeScript
- Consistent API: `themeStore.setThemeMode()` mirrors `themeStore.switchTheme()`

## Data Flow

```
App Initialization (+layout.svelte onMount)
    ↓
themeStore.initialize()
    ↓
1. Load config.theme_mode → state.themeMode
2. Determine isDark from theme_mode
3. applyThemeMode(theme_mode) → document classes
4. Initialize theme manager with correct isDark
5. Load theme
    ↓
UI Rendered (correct theme + mode immediately)
```

```
User Changes Theme Mode (Settings/ThemeSwitcher)
    ↓
handleThemeModeChange(mode)
    ↓
themeStore.setThemeMode(mode)
    ↓
1. ConfigService.updateThemeMode() → persist to backend
2. state.themeMode = mode
3. applyThemeMode(mode) → update document classes
    ↓
UI Updates Reactively (themeStore.themeMode getter)
```

## Testing Checklist

- [x] Theme mode loads immediately on app start
- [x] No flash of wrong color mode
- [x] Theme mode persists when switching themes
- [x] Light mode works correctly
- [x] Dark mode works correctly
- [x] System mode detects OS preference correctly
- [x] System mode responds to OS preference changes
- [x] Settings page theme mode buttons show correct active state
- [x] Theme switcher color mode buttons show correct active state
- [x] Both UIs stay synchronized
- [x] No TypeScript errors
- [x] No console errors
- [x] Toast notifications work

## Files Modified

1. **src/lib/stores/theme.svelte.ts** (Enhanced)
   - Added `themeMode` to state interface
   - Modified `initialize()` to load theme_mode from config first
   - Added `applyThemeMode()` method
   - Added `setThemeMode()` method
   - Added `themeMode` getter
   - Updated system preference listener to only apply when mode is 'system'

2. **src/lib/components/settings/InterfaceSettings.svelte** (Simplified)
   - Removed local `themeMode` state variable
   - Removed `applyThemeMode()` function
   - Removed `$effect()` for loading theme mode
   - Updated `handleThemeModeChange()` to use `themeStore.setThemeMode()`
   - Updated template to use `themeStore.themeMode`

3. **src/lib/components/ThemeSwitcher.svelte** (Simplified)
   - Removed local `themeMode` state variable
   - Removed `applyThemeMode()` function
   - Removed `$effect()` for loading theme mode
   - Updated `handleThemeModeChange()` to use `themeStore.setThemeMode()`
   - Updated template to use `themeStore.themeMode`

## Migration Notes

No breaking changes for users. Config format remains identical:

```ron
// config.ron (unchanged)
(
    ui: (
        theme: "catppuccin",
        theme_mode: "dark",  // "light" | "dark" | "system"
        // ... other fields
    )
)
```

## Future Enhancements

1. **Smooth Transitions**: Add CSS transitions for theme mode changes
2. **Preview Mode**: Allow temporary theme mode preview without persisting
3. **Scheduled Modes**: Auto-switch between light/dark at specific times
4. **Per-Theme Defaults**: Remember preferred mode for each theme

---

**Status**: ✅ Implemented and Tested  
**Performance Impact**: Positive - eliminates duplicate renders and state management overhead  
**User Experience**: Significantly improved - no loading delays or mode reversion
