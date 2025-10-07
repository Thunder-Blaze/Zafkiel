# Theme System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
├─────────────────────────────────────────────────────────────────┤
│  Settings Page (src/routes/settings/+page.svelte)              │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐         │  │
│  │  │Default │  │ Ocean  │  │ Forest │  │ Sunset │  ...    │  │
│  │  │ [███]  │  │ [███]  │  │ [███]  │  │ [███]  │         │  │
│  │  └────────┘  └────────┘  └────────┘  └────────┘         │  │
│  │  Theme Cards (click to switch)                           │  │
│  │                                                           │  │
│  │  Dark Mode: [Toggle Switch]                              │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Svelte 5 Store Layer                       │
├─────────────────────────────────────────────────────────────────┤
│  themeStore (src/lib/stores/theme.ts)                          │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Reactive State ($state runes):                          │  │
│  │  • currentTheme: string                                  │  │
│  │  • isDark: boolean                                       │  │
│  │  • availableThemes: ThemeMetadata[]                      │  │
│  │  • isLoading: boolean                                    │  │
│  │                                                           │  │
│  │  Methods:                                                │  │
│  │  • switchTheme(id)                                       │  │
│  │  • toggleDarkMode()                                      │  │
│  │  • initialize()                                          │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Service Layer                              │
├─────────────────────────────────────────────────────────────────┤
│  themeManager (src/lib/services/theme.ts)                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Theme Cache: Map<string, LoadedTheme>                   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐               │  │
│  │  │  Theme   │  │  Theme   │  │  Theme   │               │  │
│  │  │ Metadata │  │ Metadata │  │ Metadata │               │  │
│  │  │  + CSS   │  │  + CSS   │  │  + CSS   │               │  │
│  │  │  <link>  │  │  <link>  │  │  <link>  │               │  │
│  │  └──────────┘  └──────────┘  └──────────┘               │  │
│  │                                                           │  │
│  │  Operations:                                             │  │
│  │  • Load CSS dynamically (<link> tag)                     │  │
│  │  • Unload CSS (remove <link>)                            │  │
│  │  • Switch (change data-theme attribute)                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Tauri IPC Bridge                            │
├─────────────────────────────────────────────────────────────────┤
│  Frontend ←→ Backend Communication                              │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  invoke('list_themes')                                   │  │
│  │  invoke('get_theme_metadata', { themeId })               │  │
│  │  invoke('save_theme_preference', { themeId })            │  │
│  │  invoke('get_theme_preference')                          │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Rust Backend                               │
├─────────────────────────────────────────────────────────────────┤
│  theme_commands.rs (src-tauri/src/theme_commands.rs)           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Commands:                                               │  │
│  │  • list_themes() → scan static/themes/                  │  │
│  │  • get_theme_metadata(id) → read theme.json             │  │
│  │  • save_theme_preference(id) → update config            │  │
│  │  • get_theme_preference() → read config                 │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Filesystem Layer                             │
├─────────────────────────────────────────────────────────────────┤
│  static/themes/                  ~/.config/zafkiel/            │
│  ├── default/                    └── config.ron                │
│  │   ├── index.css               (                             │
│  │   └── theme.json                ui: (                       │
│  ├── ocean/                          theme: "ocean",           │
│  │   ├── index.css                   ...                       │
│  │   └── theme.json                )                           │
│  ├── forest/                      )                            │
│  ├── sunset/                                                   │
│  └── midnight/                                                 │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         DOM/CSS                                 │
├─────────────────────────────────────────────────────────────────┤
│  <html data-theme="ocean" class="dark">                        │
│  <head>                                                         │
│    <link rel="stylesheet" href="/themes/ocean/index.css"       │
│          data-theme-id="ocean">                                 │
│  </head>                                                        │
│                                                                 │
│  CSS Variables Applied:                                         │
│  :root[data-theme="ocean"] {                                    │
│    --primary: oklch(...);      ← Light mode colors             │
│  }                                                              │
│  .dark[data-theme="ocean"] {                                    │
│    --primary: oklch(...);      ← Dark mode colors              │
│  }                                                              │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow Diagrams

### Theme Initialization (App Startup)

```
App Mount
    │
    ├─→ themeStore.initialize()
    │       │
    │       ├─→ Detect system dark mode preference
    │       │       window.matchMedia('(prefers-color-scheme: dark)')
    │       │
    │       ├─→ Load saved theme from config
    │       │       invoke('get_theme_preference')
    │       │           │
    │       │           └─→ Backend reads ~/.config/zafkiel/config.ron
    │       │               Returns: "ocean" (or "default")
    │       │
    │       ├─→ Load available themes
    │       │       invoke('list_themes')
    │       │           │
    │       │           └─→ Backend scans static/themes/
    │       │               Reads all theme.json files
    │       │               Returns: ThemeMetadata[]
    │       │
    │       └─→ Switch to saved theme
    │               themeManager.switchTheme("ocean", isDark)
    │                   │
    │                   ├─→ Load CSS if not loaded
    │                   │       Create <link> element
    │                   │       Set href="/themes/ocean/index.css"
    │                   │       Append to <head>
    │                   │
    │                   └─→ Update data-theme attribute
    │                           document.documentElement
    │                             .setAttribute('data-theme', 'ocean')
    │
    └─→ Theme applied! ✅
```

### Theme Switch (User Action)

```
User clicks theme card
    │
    └─→ handleThemeSwitch("sunset")
            │
            ├─→ Check if already active
            │       if (currentTheme === "sunset") return
            │
            ├─→ themeStore.switchTheme("sunset")
            │       │
            │       ├─→ themeManager.switchTheme("sunset", isDark)
            │       │       │
            │       │       ├─→ Check if theme loaded
            │       │       │       if (!loadedThemes.has("sunset"))
            │       │       │           │
            │       │       │           └─→ Load theme CSS
            │       │       │                   Create <link>
            │       │       │                   href="/themes/sunset/index.css"
            │       │       │                   Wait for onload event
            │       │       │                   Store in loadedThemes Map
            │       │       │
            │       │       ├─→ Update DOM attribute
            │       │       │       document.documentElement
            │       │       │         .setAttribute('data-theme', 'sunset')
            │       │       │       (Browser instantly re-renders with new colors)
            │       │       │
            │       │       └─→ Save preference
            │       │               invoke('save_theme_preference', { themeId: 'sunset' })
            │       │                   │
            │       │                   └─→ Backend updates config.ron
            │       │                           ui.theme = "sunset"
            │       │
            │       └─→ Update store state
            │               currentTheme = "sunset"
            │
            └─→ Show success toast
                    "Switched to sunset theme" ✅
```

### Dark Mode Toggle

```
User toggles dark mode switch
    │
    └─→ toggleDarkMode()
            │
            └─→ themeStore.toggleDarkMode()
                    │
                    ├─→ Update isDark state
                    │       isDark = !isDark
                    │
                    ├─→ Update DOM class
                    │       if (isDark)
                    │         document.documentElement.classList.add('dark')
                    │       else
                    │         document.documentElement.classList.remove('dark')
                    │
                    └─→ Re-apply current theme
                            (CSS cascade automatically uses .dark[data-theme] rules)
```

## Performance Characteristics

### Initial Load
```
App Startup
  ├─ Read config: ~1ms
  ├─ Scan themes dir: ~5ms
  ├─ Load theme CSS: ~10ms (network cached after first load)
  └─ Apply data-attribute: <1ms
  ─────────────────────────
  Total: ~15ms ✅ Fast!
```

### Theme Switch
```
User clicks theme
  ├─ Check if loaded: <1ms
  ├─ Load CSS (if needed): ~10ms (first time only)
  ├─ Change data-attribute: <1ms
  └─ Browser re-render: ~5ms
  ─────────────────────────
  Total: ~15ms (or ~1ms if cached) ✅ Instant!
```

### Memory Usage
```
Per Theme:
  ├─ CSS file: ~5KB
  ├─ Parsed CSSOM: ~10KB
  └─ Metadata object: ~1KB
  ─────────────────────────
  Total per theme: ~16KB

With 5 themes loaded: ~80KB
Only active theme: ~16KB ✅ Minimal!
```

## Key Features Illustrated

### ✅ On-Demand Loading
```
Startup:        [Default] loaded
                [Ocean]   unloaded
                [Forest]  unloaded
                [Sunset]  unloaded
                [Midnight] unloaded

User previews:  [Default] loaded
                [Ocean]   ← Load CSS now
                [Forest]  unloaded
                [Sunset]  unloaded
                [Midnight] unloaded

User switches:  [Default] loaded (can unload)
                [Ocean]   ← Active theme
                [Forest]  unloaded
                [Sunset]  unloaded
                [Midnight] unloaded
```

### ✅ Fast Switching
```
Before:  <html data-theme="default">
         CSS: :root[data-theme="default"] { --primary: blue; }

After:   <html data-theme="ocean">
         CSS: :root[data-theme="ocean"] { --primary: cyan; }

Result: Browser recalculates styles, instant visual change!
        No page reload, no JavaScript needed!
```

### ✅ Dark Mode Support
```
Light:  <html data-theme="ocean">
        CSS: :root[data-theme="ocean"] { --primary: light-blue; }

Dark:   <html data-theme="ocean" class="dark">
        CSS: .dark[data-theme="ocean"] { --primary: dark-cyan; }

Result: Same theme, different color palette!
```

---

**Legend:**
- `→` Data flow direction
- `├─` Branch in flow
- `└─` End of branch
- `✅` Success state
- `[Theme]` Loaded theme
- `unloaded` Theme not in memory
