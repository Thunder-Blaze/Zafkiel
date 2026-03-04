# Title Bar Layout Reference

## Visual Structure

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ 🕐 Zafkiel ᴬᴸᴾᴴᴬ │ ◀ ▶ ⟳ │ Home Anime Demo Settings │  🔍 Search  ⌘K  │ 👤 🌓 ─ □ ✕ │
└─────────────────────────────────────────────────────────────────────────────────────┘
  Logo + Name      Navigation    Nav Items                Search Bar      Controls
```

## Sections Breakdown

### Left Section (Flex)

```
┌──────────────┬────────────┬──────────────────────────────────┐
│ Logo + Name  │  Nav Btns  │        Nav Items                 │
└──────────────┴────────────┴──────────────────────────────────┘
```

#### 1. Logo & Branding

- **Logo**: 28x28px, gradient background, clock icon
- **Name**: "Zafkiel" (text-lg, font-bold)
- **Badge**: "ALPHA" (superscript, small border)
- **Spacing**: gap-2, px-2

#### 2. Navigation Buttons (NEW)

- **Border**: Left border separator (border-l)
- **Buttons**: Back, Forward, Reload
- **Icons**: Iconify Solar icons
- **Size**: 28x28px (h-7 w-7)
- **States**:
  - Enabled: Full opacity
  - Disabled: Muted foreground color
- **Spacing**: gap-1

#### 3. Navigation Items

- **Items**: Home, Anime, Demo, Settings
- **Active State**: Primary color, primary/10 background
- **Inactive State**: foreground/70 color
- **Size**: h-8 (32px height)
- **Spacing**: gap-1

### Center Section (Absolute)

```
┌────────────────────────────────────┐
│  🔍  Search             ⌘K         │
└────────────────────────────────────┘
```

#### Search Bar (NEW)

- **Position**: Absolute center (left-1/2, -translate-x-1/2)
- **Width**: 16rem (256px)
- **Border**: border-border/60, rounded-lg
- **Background**: background/50 (semi-transparent)
- **Shadow**: shadow-sm
- **Components**:
  - Search icon (left): `solar:magnifer-bold`
  - Input field: w-64, transparent background
  - Shortcut badge (right): `⌘K`
- **States**:
  - Hover: border-border, bg-background
  - Focus: border-primary, ring-2 ring-primary/20

### Right Section (Flex)

```
┌─────────────────────────────────────┐
│  👤  │  🌓  │  ─  │  □  │  ✕      │
└─────────────────────────────────────┘
```

#### Controls (Unchanged)

- **Profile Dropdown**: px-2
- **Theme Switcher**: px-2
- **Window Controls**: Minimize, Maximize, Close
- **Width**: w-11 each (44px)

---

## Component Spacing

```css
/* Overall container */
height: 3rem (48px)
padding-left: 0.75rem (12px)

/* Logo section */
gap: 0.5rem (8px)
padding: 0.5rem (8px)

/* Navigation buttons */
border-left: 1px
padding-left: 0.5rem (8px)
gap: 0.25rem (4px)

/* Nav items */
margin-left: 0.5rem (8px)
gap: 0.25rem (4px)

/* Search bar */
padding: 0.375rem 0.75rem (6px 12px)
gap: 0.5rem (8px)

/* Profile + Theme */
padding: 0.5rem (8px)

/* Window controls */
width: 2.75rem (44px)
```

---

## Icons Reference

### Navigation Buttons

```typescript
{
  back: 'solar:alt-arrow-left-bold',
  forward: 'solar:alt-arrow-right-bold',
  reload: 'solar:refresh-bold',
}
```

### Nav Items

```typescript
{
  home: 'solar:home-bold',
  anime: 'solar:video-library-bold',
  demo: 'solar:gallery-bold',
  settings: 'solar:settings-bold',
}
```

### Search

```typescript
{
  search: 'solar:magnifer-bold',
}
```

### Window Controls

```typescript
{
  minimize: 'solar:minus-circle-bold',
  maximize: 'solar:full-screen-square-bold',
  restore: 'solar:quit-full-screen-square-bold',
  close: 'solar:close-circle-bold',
}
```

---

## Color System

### Navigation Buttons

```typescript
// Enabled
text: 'text-foreground';
hover: 'hover:bg-foreground/5';
active: 'active:bg-foreground/10';

// Disabled
text: 'text-muted-foreground';
cursor: 'cursor-not-allowed';
opacity: 'opacity-50';
```

### Search Bar

```typescript
// Default
border: 'border-border/60';
background: 'bg-background/50';
shadow: 'shadow-sm';

// Hover
border: 'border-border';
background: 'bg-background';

// Focus
border: 'border-primary';
ring: 'ring-2 ring-primary/20';

// Icon
text: 'text-muted-foreground';

// Badge
background: 'bg-muted/30';
border: 'border-border/50';
text: 'text-muted-foreground';
```

### Nav Items

```typescript
// Active
background: 'bg-primary/10';
text: 'text-primary';

// Inactive
text: 'text-foreground/70';
hover: 'hover:text-foreground';
```

---

## Responsive Behavior

### Desktop (Default)

- All sections visible
- Search bar 256px wide
- Navigation buttons always shown
- Nav items display full labels

### Considerations for Future

- Mobile: Hide nav items, show hamburger menu
- Tablet: Reduce search bar width to 192px
- Small screens: Stack or hide less important elements

---

## Accessibility

### ARIA Labels

```svelte
<button aria-label="Go back (Alt+←)" />
<button aria-label="Go forward (Alt+→)" />
<button aria-label="Reload (⌘+R)" />
<input aria-label="Search" placeholder="Search" />
```

### Keyboard Navigation

- Tab order: Logo → Nav buttons → Nav items → Search → Profile → Theme → Window controls
- Focus indicators: Ring on focus
- Disabled state: `disabled` attribute prevents focus

### Screen Reader Support

- Button titles for tooltips
- Descriptive labels for all controls
- State announcements (disabled/enabled)

---

## Z-Index Layers

```
Title Bar: z-[999999]
  ├─ Search Bar: (no z-index, uses absolute position)
  ├─ Navigation: (no z-index, flex order)
  └─ Controls: (no z-index, flex order)
```

---

## Animation & Transitions

### Buttons

```css
transition: colors
duration: default (150ms)
```

### Search Bar

```css
transition: all
duration: 200ms
```

### States

- Hover: Smooth color transition
- Focus: Instant ring appearance
- Disabled: Instant opacity change

---

## Code Structure

```svelte
<div class="title-bar">
	<!-- Left Section -->
	<div class="left-section">
		<!-- Logo & Name -->
		<div class="logo-brand">
			<div class="logo">🕐</div>
			<div class="name">
				Zafkiel
				<sup>ALPHA</sup>
			</div>
		</div>

		<!-- Navigation Buttons -->
		<div class="nav-buttons">
			<Button>◀</Button>
			<Button>▶</Button>
			<Button>⟳</Button>
		</div>

		<!-- Nav Items -->
		<div class="nav-items">
			<Button>Home</Button>
			<Button>Anime</Button>
			<Button>Demo</Button>
			<Button>Settings</Button>
		</div>
	</div>

	<!-- Center Section -->
	<div class="center-section">
		<form class="search-bar">
			<Icon>🔍</Icon>
			<input placeholder="Search" />
			<Badge>⌘K</Badge>
		</form>
	</div>

	<!-- Right Section -->
	<div class="right-section">
		<ProfileDropdown />
		<ThemeSwitcher />
		<WindowControls />
	</div>
</div>
```

---

## State Management

### Navigation History

```typescript
navigationHistory: string[] = ['/', '/anime', '/settings']
currentHistoryIndex: number = 2
canGoBack: boolean = currentHistoryIndex > 0
canGoForward: boolean = currentHistoryIndex < history.length - 1
```

### Search

```typescript
searchInput: HTMLInputElement | null;
searchQuery: string = '';
```

### Window

```typescript
isMaximized: boolean;
isFullscreen: boolean;
```

---

## Testing Checklist

### Visual

- [ ] Logo renders correctly
- [ ] Badge displays "ALPHA"
- [ ] Navigation buttons aligned
- [ ] Search bar centered
- [ ] Icons load properly
- [ ] Spacing consistent

### Functional

- [ ] Back button works
- [ ] Forward button works
- [ ] Reload button works
- [ ] Search input focuses
- [ ] Enter submits search
- [ ] Nav items navigate correctly

### States

- [ ] Disabled buttons muted
- [ ] Active nav highlighted
- [ ] Search hover effect
- [ ] Search focus ring
- [ ] Button hover effects

### Keyboard

- [ ] Alt+← goes back
- [ ] Alt+→ goes forward
- [ ] ⌘+R reloads
- [ ] ⌘+K focuses search
- [ ] Tab navigation works

---

**Reference Version**: 1.0
**Last Updated**: October 11, 2025
**Component**: `src/lib/components/TitleBar.svelte`
