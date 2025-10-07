import { contextMenuStore, type ContextMenuItem } from '$lib/stores/context-menu';

/**
 * Hook for adding context menu to elements
 * 
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useContextMenu } from '$lib/hooks/useContextMenu';
 *   
 *   const items = [
 *     { id: 'copy', label: 'Copy', icon: 'solar:copy-bold', onClick: () => console.log('Copy') },
 *     { id: 'paste', label: 'Paste', icon: 'solar:clipboard-bold', onClick: () => console.log('Paste') },
 *   ];
 *   
 *   const { onContextMenu } = useContextMenu(items);
 * </script>
 * 
 * <div oncontextmenu={onContextMenu} data-has-context-menu>
 *   Right click me!
 * </div>
 * ```
 */
export function useContextMenu(items: ContextMenuItem[] | (() => ContextMenuItem[])) {
	const onContextMenu = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();

		const menuItems = typeof items === 'function' ? items() : items;

		contextMenuStore.open(event.clientX, event.clientY, menuItems);
	};

	return {
		onContextMenu
	};
}

/**
 * Hook for adding context menu with dynamic items based on event target
 * 
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useContextMenuDynamic } from '$lib/hooks/useContextMenu';
 *   
 *   const { onContextMenu } = useContextMenuDynamic((element) => {
 *     const text = element.textContent;
 *     return [
 *       { id: 'copy', label: `Copy "${text}"`, onClick: () => navigator.clipboard.writeText(text) }
 *     ];
 *   });
 * </script>
 * 
 * <div oncontextmenu={onContextMenu}>
 *   Right click me!
 * </div>
 * ```
 */
export function useContextMenuDynamic(
	itemsFactory: (element: HTMLElement) => ContextMenuItem[]
) {
	const onContextMenu = (event: MouseEvent) => {
		event.preventDefault();
		event.stopPropagation();

		const target = event.currentTarget as HTMLElement;
		const menuItems = itemsFactory(target);

		contextMenuStore.open(event.clientX, event.clientY, menuItems);
	};

	return {
		onContextMenu
	};
}
