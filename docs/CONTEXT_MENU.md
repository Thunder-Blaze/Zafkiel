# Custom Context Menu System

A fully-featured, customizable right-click context menu system for Zafkiel, built with Svelte 5 and styled with shadcn-svelte components.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [With Icons](#with-icons)
  - [With Keyboard Shortcuts](#with-keyboard-shortcuts)
  - [With Disabled Items](#with-disabled-items)
  - [Dynamic Context Menus](#dynamic-context-menus)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Styling](#styling)
- [Best Practices](#best-practices)

## Overview

The custom context menu system replaces the browser's default right-click menu with a beautiful, themed menu that matches your application's design system. It's built with:

- **Svelte 5** with runes for reactivity
- **shadcn-svelte** for consistent styling
- **TypeScript** for type safety
- **Composable hooks** for easy integration

## Features

✨ **Feature Highlights**:

- 🎨 **Themed**: Automatically matches your app's theme (light/dark)
- ⚡ **High Performance**: Efficient rendering with Svelte 5 runes
- 🔧 **Highly Customizable**: Icons, shortcuts, separators, disabled states
- 📱 **Smart Positioning**: Auto-adjusts to stay within viewport bounds
- ⌨️ **Keyboard Support**: Close with Escape, navigate with Tab
- 🎯 **Easy API**: Simple hook-based interface
- ♿ **Accessible**: Proper ARIA roles and keyboard navigation
- 🔒 **Type Safe**: Full TypeScript support

## Installation

The context menu system is already integrated into the Zafkiel project. All components are available globally:

```typescript
// Already included in +layout.svelte
import ContextMenu from '$lib/components/ContextMenu.svelte';
```

## Usage

### Basic Usage

The simplest way to add a context menu is using the `useContextMenu` hook:

```svelte
<script lang="ts">
	import { useContextMenu } from '$lib/hooks/useContextMenu';

	const items = [
		{
			id: 'copy',
			label: 'Copy',
			onClick: () => console.log('Copy clicked'),
		},
		{
			id: 'paste',
			label: 'Paste',
			onClick: () => console.log('Paste clicked'),
		},
	];

	const { onContextMenu } = useContextMenu(items);
</script>

<div oncontextmenu={onContextMenu}>Right-click me!</div>
```

### With Icons

Add icons from Iconify (recommended) or HTML/emoji icons to your menu items:

```svelte
<script lang="ts">
	import { useContextMenu } from '$lib/hooks/useContextMenu';

	const items = [
		{
			id: 'copy',
			label: 'Copy',
			icon: 'solar:copy-bold', // Iconify Solar icon (recommended)
			onClick: () => navigator.clipboard.writeText('...'),
		},
		{
			id: 'paste',
			label: 'Paste',
			icon: 'solar:clipboard-bold',
			onClick: () => console.log('Paste'),
		},
		{
			id: 'delete',
			label: 'Delete',
			icon: 'solar:trash-bin-trash-bold',
			onClick: () => console.log('Delete'),
		},
	];

	const { onContextMenu } = useContextMenu(items);
</script>

<div oncontextmenu={onContextMenu} data-has-context-menu>Right-click for menu with icons</div>
```

**Icon Types**:

- **Iconify icons** (recommended): Use the format `'iconset:icon-name'`, e.g., `'solar:copy-bold'`
- **HTML/Emoji**: Use emoji strings or HTML directly, e.g., `'📋'` or `'<svg>...</svg>'`

**Available Solar Icon Sets**:
The project is configured to use Solar icons from Iconify. Browse available icons at [icones.js.org/collection/solar](https://icones.js.org/collection/solar).

### With Keyboard Shortcuts

Display keyboard shortcuts for menu items:

```svelte
<script lang="ts">
	const items = [
		{
			id: 'copy',
			label: 'Copy',
			icon: 'solar:copy-bold',
			shortcut: '⌘C', // Displayed on the right
			onClick: () => navigator.clipboard.writeText('...'),
		},
		{
			id: 'paste',
			label: 'Paste',
			icon: 'solar:clipboard-bold',
			shortcut: '⌘V',
			onClick: () => console.log('Paste'),
		},
	];

	const { onContextMenu } = useContextMenu(items);
</script>

<div oncontextmenu={onContextMenu} data-has-context-menu>Right-click for menu with shortcuts</div>
```

**Note**: Shortcuts are visual only. You need to implement the actual keyboard handlers separately.

### With Disabled Items

Some items can be disabled based on application state:

```svelte
<script lang="ts">
	let canPaste = $state(false);

	const items = $derived([
		{
			id: 'copy',
			label: 'Copy',
			onClick: () => console.log('Copy'),
		},
		{
			id: 'paste',
			label: 'Paste',
			disabled: !canPaste, // Dynamically disabled
			onClick: () => console.log('Paste'),
		},
	]);

	const { onContextMenu } = useContextMenu(items);
</script>
```

### With Separators

Use separators to group related items:

```svelte
<script lang="ts">
	const items = [
		{
			id: 'copy',
			label: 'Copy',
			onClick: () => console.log('Copy'),
		},
		{
			id: 'paste',
			label: 'Paste',
			onClick: () => console.log('Paste'),
		},
		{
			id: 'sep1',
			label: '',
			separator: true, // This creates a separator line
		},
		{
			id: 'delete',
			label: 'Delete',
			onClick: () => console.log('Delete'),
		},
	];

	const { onContextMenu } = useContextMenu(items);
</script>
```

### Dynamic Context Menus

Create context menus that change based on what was clicked:

```svelte
<script lang="ts">
	import { useContextMenuDynamic } from '$lib/hooks/useContextMenu';

	const { onContextMenu } = useContextMenuDynamic((element) => {
		const text = element.textContent;

		return [
			{
				id: 'copy-text',
				label: `Copy "${text}"`,
				icon: 'solar:copy-bold',
				onClick: () => navigator.clipboard.writeText(text),
			},
			{
				id: 'search',
				label: `Search for "${text}"`,
				icon: 'solar:magnifer-bold',
				onClick: () => {
					window.open(`https://google.com/search?q=${encodeURIComponent(text)}`, '_blank');
				},
			},
		];
	});
</script>

<div oncontextmenu={onContextMenu}>
	{anime.title}
</div>
```

## API Reference

### `ContextMenuItem` Interface

```typescript
interface ContextMenuItem {
	id: string; // Unique identifier
	label: string; // Display text
	icon?: string; // Icon (emoji or HTML string)
	shortcut?: string; // Keyboard shortcut display (e.g., "⌘C")
	disabled?: boolean; // Whether the item is disabled
	separator?: boolean; // Whether this is a separator
	onClick?: () => void | Promise<void>; // Click handler
}
```

### `useContextMenu(items)` Hook

Creates a context menu with static items.

**Parameters**:

- `items`: `ContextMenuItem[]` or `() => ContextMenuItem[]` - Menu items or factory function

**Returns**:

- `{ onContextMenu }` - Event handler for `oncontextmenu` event

**Example**:

```typescript
const { onContextMenu } = useContextMenu([{ id: 'copy', label: 'Copy', onClick: () => {} }]);
```

### `useContextMenuDynamic(factory)` Hook

Creates a context menu with items based on the clicked element.

**Parameters**:

- `factory`: `(element: HTMLElement) => ContextMenuItem[]` - Function that returns items based on the element

**Returns**:

- `{ onContextMenu }` - Event handler for `oncontextmenu` event

**Example**:

```typescript
const { onContextMenu } = useContextMenuDynamic((element) => [
	{ id: 'info', label: `Info for ${element.id}`, onClick: () => {} },
]);
```

### `contextMenuStore`

Global store for managing context menu state.

**Methods**:

- `open(x, y, items)` - Open menu at coordinates with items
- `close()` - Close the menu
- `handleItemClick(item)` - Handle item click (internal)

**State**:

```typescript
{
	isOpen: boolean;
	x: number;
	y: number;
	items: ContextMenuItem[];
}
```

## Examples

### Anime List Context Menu

```svelte
<script lang="ts">
	import { useContextMenu } from '$lib/hooks/useContextMenu';
	import { toast } from 'svelte-sonner';

	const createAnimeMenu = (anime: Anime) => [
		{
			id: 'view',
			label: 'View Details',
			icon: 'solar:eye-bold',
			onClick: () => router.push(`/anime/${anime.id}`),
		},
		{
			id: 'sep1',
			label: '',
			separator: true,
		},
		{
			id: 'watching',
			label: 'Mark as Watching',
			icon: 'solar:play-circle-bold',
			onClick: async () => {
				await updateAnimeStatus(anime.id, 'WATCHING');
				toast.success('Status updated!');
			},
		},
		{
			id: 'completed',
			label: 'Mark as Completed',
			icon: 'solar:check-circle-bold',
			onClick: async () => {
				await updateAnimeStatus(anime.id, 'COMPLETED');
				toast.success('Marked as completed!');
			},
		},
		{
			id: 'sep2',
			label: '',
			separator: true,
		},
		{
			id: 'favorite',
			label: 'Add to Favorites',
			icon: 'solar:star-bold',
			onClick: async () => {
				await addToFavorites(anime.id);
				toast.success('Added to favorites!');
			},
		},
	];

	const { onContextMenu } = useContextMenu(() => createAnimeMenu(anime));
</script>

<div class="anime-card" oncontextmenu={onContextMenu}>
	<!-- Anime card content -->
</div>
```

### Text Selection Menu

```svelte
<script lang="ts">
	import { contextMenuStore } from '$lib/stores/context-menu';

	const handleTextSelection = (e: MouseEvent) => {
		e.preventDefault();

		const selection = window.getSelection();
		const text = selection?.toString() || '';

		if (!text) return;

		const items = [
			{
				id: 'copy',
				label: 'Copy',
				icon: 'solar:copy-bold',
				shortcut: '⌘C',
				onClick: async () => {
					await navigator.clipboard.writeText(text);
				},
			},
			{
				id: 'search',
				label: `Search "${text.substring(0, 20)}..."`,
				icon: 'solar:magnifer-bold',
				onClick: () => {
					window.open(`https://google.com/search?q=${encodeURIComponent(text)}`, '_blank');
				},
			},
			{
				id: 'translate',
				label: 'Translate',
				icon: 'solar:global-bold',
				onClick: () => {
					// Open translation service
				},
			},
		];

		contextMenuStore.open(e.clientX, e.clientY, items);
	};
</script>

<article oncontextmenu={handleTextSelection}>
	<!-- Article content -->
</article>
```

## Styling

The context menu uses Tailwind CSS classes and shadcn-svelte design tokens. It automatically adapts to your app's theme.

### Custom Styling

You can customize the appearance by modifying `ContextMenu.svelte`:

```svelte
<style>
	.context-menu-content {
		/* Override default styles */
		@apply rounded-lg border bg-popover p-1 shadow-2xl;
	}

	.context-menu-item {
		/* Customize item appearance */
		@apply px-3 py-2 text-sm;
	}
</style>
```

### Theme Integration

The menu automatically uses your theme colors:

- `bg-popover` / `text-popover-foreground` for the menu background
- `bg-accent` / `text-accent-foreground` for hover states
- `border` for borders and separators
- `text-muted-foreground` for shortcuts

## Best Practices

### ✅ DO

- **Use meaningful IDs** for menu items
- **Provide visual feedback** with toast notifications
- **Group related actions** with separators
- **Show keyboard shortcuts** for common actions
- **Disable unavailable actions** instead of hiding them
- **Keep menus concise** (5-8 items max)
- **Use icons consistently** across your app

```svelte
// ✅ Good
const items = [
	{
		id: 'copy-title',
		label: 'Copy Title',
		icon: 'solar:copy-bold',
		shortcut: '⌘C',
		onClick: async () => {
			await navigator.clipboard.writeText(anime.title);
			toast.success('Title copied!');
		}
	}
];
```

### ❌ DON'T

- **Don't hardcode menu items** - make them dynamic
- **Don't forget error handling** in onClick handlers
- **Don't create deeply nested menus** - keep it flat
- **Don't use technical terms** - use user-friendly labels
- **Don't show empty menus** - check conditions first

```svelte
// ❌ Bad
const items = [
	{
		id: '1',
		label: 'executeClipboardWriteOperation',
		onClick: () => {
			navigator.clipboard.writeText(anime.title); // No error handling, no feedback
		}
	}
];
```

### Performance Tips

1. **Use `$derived`** for dynamic menu items:

   ```typescript
   const items = $derived(createMenuItems(selectedItems));
   ```

2. **Memoize expensive calculations**:

   ```typescript
   const items = $derived.by(() => {
   	// Expensive computation
   	return processedItems;
   });
   ```

3. **Close menu after actions**:
   ```typescript
   // The store automatically closes after onClick
   // No need to manually close
   ```

## Accessibility

The context menu implements proper accessibility features:

- ✅ **ARIA roles**: `role="menu"` and `role="menuitem"`
- ✅ **Keyboard navigation**: Tab, Shift+Tab, Escape
- ✅ **Focus management**: Auto-focus first item
- ✅ **Screen reader support**: Proper labels and descriptions
- ✅ **Disabled state**: `disabled` attribute and `aria-disabled`

## Testing

### Unit Tests

```typescript
import { describe, it, expect } from 'vitest';
import { contextMenuStore } from '$lib/stores/context-menu';

describe('Context Menu Store', () => {
	it('should open menu at coordinates', () => {
		const items = [{ id: 'test', label: 'Test', onClick: () => {} }];

		contextMenuStore.open(100, 200, items);

		// Assert menu state
	});

	it('should close menu', () => {
		contextMenuStore.close();
		// Assert closed state
	});
});
```

### Storybook Stories

Context menu stories are available in Storybook:

```bash
bun run storybook
```

Navigate to **Components > ContextMenu** to see all variants.

## Demo

See the context menu in action:

```bash
bun run tauri dev
```

Navigate to `/context-menu-demo` to see:

- Simple context menu
- Anime-specific actions
- Text selection menu
- Disabled items
- Complex nested menus

## Related Documentation

- [Svelte 5 Documentation](https://svelte.dev/docs/svelte/overview)
- [shadcn-svelte Components](https://shadcn-svelte.com/docs)
- [Tauri Window API](https://tauri.app/develop/calling-frontend/)

## Troubleshooting

### Menu appears in wrong position

The menu auto-adjusts to viewport bounds. If issues persist:

```typescript
// Manually adjust in ContextMenu.svelte
let adjustedX = Math.min(menuState.x, window.innerWidth - menuWidth);
let adjustedY = Math.min(menuState.y, window.innerHeight - menuHeight);
```

### Menu doesn't close on click outside

Ensure the `ContextMenu` component is mounted in the layout:

```svelte
<!-- +layout.svelte -->
<ContextMenu />
```

### TypeScript errors

Ensure all menu items have the required properties:

```typescript
const items: ContextMenuItem[] = [
	{
		id: 'unique-id', // Required
		label: 'Label', // Required
		onClick: () => {}, // Required (unless separator)
	},
];
```

## Future Enhancements

Planned features:

- [ ] Sub-menus (nested menus)
- [ ] Custom icons (Lucide icons integration)
- [ ] Menu animations customization
- [ ] Touch device support
- [ ] Multi-select context menus
- [ ] Context menu templates

---

**Built with ❤️ for Zafkiel**
