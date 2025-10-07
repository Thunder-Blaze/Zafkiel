# Theme System - Quick Start Guide

## For Users

### Switching Themes

1. Open Settings (⚙️ icon)
2. Navigate to **Appearance** section
3. Browse available themes with visual previews
4. Click on any theme card to apply it
5. Toggle **Dark Mode** switch for light/dark variant

**Available Themes:**
- 🔵 **Default** - Clean blue tones (professional)
- 🌊 **Ocean** - Deep blue/cyan (calming)
- 🌲 **Forest** - Natural greens (eye-friendly)
- 🌅 **Sunset** - Warm orange/pink (vibrant)
- 🌙 **Midnight** - Deep purple/black (night mode)

### Theme Preview

Each theme card shows 3 color bars representing:
1. **Primary** - Main accent color
2. **Background** - Page background
3. **Accent** - Secondary highlights

### Persistence

Your theme choice is automatically saved and will be restored when you restart the app.

## For Developers

### Creating a New Theme

1. **Create theme directory:**
```bash
mkdir static/themes/mytheme
```

2. **Create `index.css`:**
```css
:root[data-theme="mytheme"] {
  --background: oklch(0.98 0.01 0);
  --foreground: oklch(0.15 0.05 0);
  --primary: oklch(0.55 0.15 270);
  /* ... add all CSS custom properties */
}

.dark[data-theme="mytheme"] {
  --background: oklch(0.12 0.04 0);
  --foreground: oklch(0.95 0.01 0);
  --primary: oklch(0.65 0.14 270);
  /* ... add all dark mode variables */
}
```

3. **Create `theme.json`:**
```json
{
  "id": "mytheme",
  "name": "My Theme",
  "description": "A beautiful custom theme",
  "author": "Your Name",
  "version": "1.0.0",
  "colors": {
    "light": {
      "primary": "oklch(0.55 0.15 270)",
      "background": "oklch(0.98 0.01 0)",
      "accent": "oklch(0.88 0.06 250)"
    },
    "dark": {
      "primary": "oklch(0.65 0.14 270)",
      "background": "oklch(0.12 0.04 0)",
      "accent": "oklch(0.35 0.08 250)"
    }
  }
}
```

4. **Test your theme:**
- Restart the app
- Open Settings → Appearance
- Your theme appears automatically!

### Required CSS Variables

Your theme must define these variables (copy from `static/themes/default/index.css`):

```css
/* Colors */
--background
--foreground
--card
--card-foreground
--popover
--popover-foreground
--primary
--primary-foreground
--secondary
--secondary-foreground
--muted
--muted-foreground
--accent
--accent-foreground
--destructive
--destructive-foreground
--border
--input
--ring

/* Charts */
--chart-1
--chart-2
--chart-3
--chart-4
--chart-5

/* Sidebar */
--sidebar
--sidebar-foreground
--sidebar-primary
--sidebar-primary-foreground
--sidebar-accent
--sidebar-accent-foreground
--sidebar-border
--sidebar-ring

/* Typography */
--font-sans
--font-serif
--font-mono

/* Layout */
--radius

/* Shadows */
--shadow-2xs
--shadow-xs
--shadow-sm
--shadow
--shadow-md
--shadow-lg
--shadow-xl
--shadow-2xl
```

### Using the Theme API

**In Svelte components:**

```typescript
import { themeStore } from '$lib/stores/theme';

// Get current theme
const currentTheme = themeStore.currentTheme;

// Check dark mode
const isDark = themeStore.isDark;

// Switch theme
await themeStore.switchTheme('ocean');

// Toggle dark mode
await themeStore.toggleDarkMode();

// Get all themes
const themes = themeStore.availableThemes;

// Preview theme metadata
const metadata = await themeStore.previewTheme('forest');
```

**In plain TypeScript:**

```typescript
import { themeManager } from '$lib/services/theme';

// List themes
const themes = await themeManager.listThemes();

// Load a theme
await themeManager.loadTheme('sunset');

// Switch theme
await themeManager.switchTheme('midnight', true); // true = dark mode

// Unload theme
themeManager.unloadTheme('ocean');
```

### Color System

We use **OKLCH** color space for better perceptual uniformity:

```css
/* Format: oklch(lightness chroma hue) */
oklch(0.55 0.15 270)
     │    │    └─ Hue (0-360°)
     │    └─ Chroma (saturation)
     └─ Lightness (0-1)
```

**Tips:**
- Keep lightness consistent within mode (light/dark)
- Adjust hue for different color families
- Higher chroma = more saturated
- Lower chroma = more muted

### Theme Testing Checklist

- [ ] Light mode looks good
- [ ] Dark mode looks good
- [ ] All UI components visible
- [ ] Text is readable
- [ ] Buttons have good contrast
- [ ] Cards stand out from background
- [ ] Hover states are visible
- [ ] Focus indicators are clear
- [ ] Preview colors match theme

### Advanced: Programmatic Theme Creation

You can generate themes programmatically:

```typescript
import { invoke } from '@tauri-apps/api/core';

// Save theme files via backend
await invoke('create_theme', {
  themeId: 'auto-generated',
  css: generatedCss,
  metadata: {
    id: 'auto-generated',
    name: 'Auto Theme',
    // ...
  }
});
```

### Debugging

**Theme not appearing?**
1. Check `static/themes/{id}/theme.json` exists
2. Validate JSON syntax
3. Check theme ID matches directory name
4. Restart app to refresh theme list

**Colors not applying?**
1. Verify CSS variable names match exactly
2. Check `data-theme` attribute is set on `<html>`
3. Inspect element to see computed styles
4. Clear browser cache

**Performance issues?**
1. Unload unused themes: `themeManager.unloadInactiveThemes()`
2. Check for CSS conflicts in DevTools
3. Limit number of loaded themes

## API Reference

### ThemeStore (Svelte 5 Store)

```typescript
class ThemeStore {
  // Reactive getters
  get currentTheme(): string
  get isDark(): boolean
  get availableThemes(): ThemeMetadata[]
  get isLoading(): boolean
  get initialized(): boolean

  // Methods
  initialize(): Promise<void>
  switchTheme(themeId: string): Promise<void>
  toggleDarkMode(): Promise<void>
  setDarkMode(isDark: boolean): Promise<void>
  previewTheme(themeId: string): Promise<ThemeMetadata>
  getTheme(themeId: string): ThemeMetadata | undefined
  isThemeLoaded(themeId: string): boolean
}
```

### ThemeManager (Service)

```typescript
class ThemeManager {
  listThemes(): Promise<ThemeMetadata[]>
  loadTheme(themeId: string): Promise<ThemeMetadata>
  unloadTheme(themeId: string): void
  switchTheme(themeId: string, isDark: boolean): Promise<void>
  previewTheme(themeId: string): Promise<ThemeMetadata>
  getCurrentTheme(): string
  getLoadedThemes(): string[]
  isThemeLoaded(themeId: string): boolean
  initialize(isDark: boolean): Promise<void>
  unloadInactiveThemes(): void
}
```

### Tauri Commands

```rust
// Rust backend commands
list_themes() -> Vec<ThemeMetadata>
get_theme_metadata(theme_id: String) -> ThemeMetadata
save_theme_preference(theme_id: String) -> Result<(), String>
get_theme_preference() -> Result<String, String>
```

## Support

For issues or questions:
1. Check `THEME_SYSTEM.md` for detailed documentation
2. Inspect browser DevTools Console for errors
3. Check `~/.config/zafkiel/config.ron` for saved theme
4. Open an issue on GitHub

---

**Happy Theming! 🎨**
