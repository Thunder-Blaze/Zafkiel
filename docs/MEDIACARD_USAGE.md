# MediaCard Usage Guide 🎯

## Quick Start

```svelte
<script>
  import MediaCard from '$lib/components/MediaCard.svelte';
  import type { MediaData } from '$lib/types/media';
</script>

<MediaCard {mediaData} variant="grid" size="md" position="center" />
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `mediaData` | `MediaData` | **required** | Media information object |
| `variant` | `'grid' \| 'list' \| 'compact'` | `'grid'` | Display style |
| `size` | `'sm' \| 'md' \| 'lg'` | `'md'` | Card size (grid only) |
| `position` | `'left' \| 'center' \| 'right'` | `'center'` | Preview overflow position |
| `onProgressUpdate` | `(newProgress: number) => void` | `undefined` | Progress change callback |
| `onStatusChange` | `(newStatus: string) => void` | `undefined` | Status change callback |

## View Variants

### Grid View
Best for: Browse pages, search results, recommendations

```svelte
<!-- Small cards (w-32) -->
<MediaCard {mediaData} variant="grid" size="sm" />

<!-- Medium cards (w-40) - Default -->
<MediaCard {mediaData} variant="grid" size="md" />

<!-- Large cards (w-48) -->
<MediaCard {mediaData} variant="grid" size="lg" />
```

**Features:**
- Vertical card with cover image
- Minimal overlays (score, 18+ badge)
- Thin progress bar at bottom
- Compact title + year/format
- Detailed preview on hover

### List View
Best for: My list page, search with details, library view

```svelte
<MediaCard {mediaData} variant="list" />
```

**Features:**
- Horizontal layout with thumbnail
- Inline metadata (score, year, format)
- Progress bar and status badge
- Quick increment button
- Same detailed preview on hover

### Compact View
Best for: Continue watching, sidebar, recently viewed

```svelte
<MediaCard {mediaData} variant="compact" />
```

**Features:**
- Ultra-minimal design
- Small thumbnail (88px height)
- Single line title
- Progress bar only
- No preview on hover

## Position Prop

Use `position` to control preview card overflow direction:

```svelte
<!-- First item in row - overflow right -->
<MediaCard {mediaData} position="left" />

<!-- Middle items - overflow both sides (centered) -->
<MediaCard {mediaData} position="center" />

<!-- Last item in row - overflow left -->
<MediaCard {mediaData} position="right" />
```

## Common Layouts

### Browse Grid (5 columns)
```svelte
<div class="grid grid-cols-5 gap-4">
  {#each animeList as anime, i}
    <MediaCard 
      mediaData={anime}
      variant="grid"
      size="md"
      position={i % 5 === 0 ? 'left' : (i + 1) % 5 === 0 ? 'right' : 'center'}
    />
  {/each}
</div>
```

### My List
```svelte
<div class="space-y-2">
  {#each myList as item}
    <MediaCard 
      mediaData={item}
      variant="list"
      position="center"
      onProgressUpdate={(progress) => updateProgress(item.id, progress)}
      onStatusChange={(status) => updateStatus(item.id, status)}
    />
  {/each}
</div>
```

### Continue Watching Sidebar
```svelte
<div class="w-64 space-y-1.5">
  <h3 class="text-sm font-semibold mb-2">Continue Watching</h3>
  {#each continueWatching as item}
    <MediaCard 
      mediaData={item}
      variant="compact"
    />
  {/each}
</div>
```

### Responsive Grid
```svelte
<div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3">
  {#each items as item, index}
    {@const cols = getColumnCount()} <!-- Get current breakpoint columns -->
    {@const position = 
      index % cols === 0 ? 'left' : 
      (index + 1) % cols === 0 ? 'right' : 
      'center'
    }
    <MediaCard 
      mediaData={item}
      variant="grid"
      size="md"
      {position}
    />
  {/each}
</div>
```

### Mixed Layout (Hero + Grid)
```svelte
<!-- Featured/Hero -->
<div class="mb-8">
  <MediaCard mediaData={featured} variant="grid" size="lg" position="left" />
</div>

<!-- Trending Grid -->
<div class="grid grid-cols-6 gap-3">
  {#each trending as item, i}
    <MediaCard 
      mediaData={item}
      variant="grid"
      size="sm"
      position={i === 0 ? 'left' : i === 5 ? 'right' : 'center'}
    />
  {/each}
</div>
```

## Event Handlers

### Progress Updates
```svelte
<script>
  async function handleProgressUpdate(mediaId: number, newProgress: number) {
    await updateUserProgress(mediaId, newProgress);
    // Refresh data
    invalidate('mediaList');
  }
</script>

<MediaCard 
  {mediaData}
  onProgressUpdate={(progress) => handleProgressUpdate(mediaData.id, progress)}
/>
```

### Status Changes
```svelte
<script>
  async function handleStatusChange(mediaId: number, newStatus: string) {
    await updateUserStatus(mediaId, newStatus);
    // Refresh data
    invalidate('mediaList');
  }
</script>

<MediaCard 
  {mediaData}
  onStatusChange={(status) => handleStatusChange(mediaData.id, status)}
/>
```

## Styling & Spacing

### Grid Spacing
```svelte
<!-- Tight -->
<div class="grid grid-cols-5 gap-2">

<!-- Normal -->
<div class="grid grid-cols-5 gap-4">

<!-- Loose -->
<div class="grid grid-cols-5 gap-6">
```

### List Spacing
```svelte
<!-- Tight -->
<div class="space-y-1">

<!-- Normal -->
<div class="space-y-2">

<!-- Loose -->
<div class="space-y-3">
```

### Compact Spacing
```svelte
<!-- Ultra-tight -->
<div class="space-y-1">

<!-- Tight -->
<div class="space-y-1.5">
```

## Performance Tips

1. **Use appropriate variant**: Don't use grid view in dense lists
2. **Limit preview hover**: Consider disabling for mobile
3. **Lazy load images**: Use loading="lazy" on cover images
4. **Virtual scrolling**: For long lists (>100 items)
5. **Pagination**: Don't render 1000+ cards at once

## Accessibility

- All cards have `role="article"`
- Images have proper alt text
- Interactive elements are keyboard accessible
- Hover previews don't interfere with keyboard navigation

## Theme Support

All cards automatically adapt to:
- Light/Dark mode
- Custom theme colors
- User animation preferences
- User blur effect preferences
- User glow effect preferences

## Examples by Page Type

### Homepage
```svelte
<!-- Trending -->
<div class="grid grid-cols-6 gap-3">
  <MediaCard variant="grid" size="sm" />
</div>

<!-- Continue Watching -->
<div class="space-y-2">
  <MediaCard variant="list" />
</div>
```

### Search Results
```svelte
<!-- With filters -->
<div class="grid grid-cols-4 gap-4">
  <MediaCard variant="grid" size="md" />
</div>
```

### User Profile
```svelte
<!-- Favorites -->
<div class="grid grid-cols-5 gap-4">
  <MediaCard variant="grid" size="md" />
</div>

<!-- Recently Watched -->
<div class="space-y-1.5">
  <MediaCard variant="compact" />
</div>
```

---

**Pro Tip**: Always set `position` prop based on item position in grid to prevent preview cards from overflowing viewport edges!
