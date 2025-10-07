<script lang="ts">
	import { onMount } from 'svelte';
	import { contextMenuStore } from '$lib/stores/context-menu';
	import { getDefaultContextMenuItems } from '$lib/utils/default-context-menu';

	const { children } = $props();

	onMount(() => {
		// Global context menu handler
		const handleGlobalContextMenu = (e: MouseEvent) => {
			// Check if the target or any parent has a custom context menu handler
			const target = e.target as HTMLElement;
			const hasCustomHandler = target.closest('[data-has-context-menu]');

			// If no custom handler, show default context menu
			if (!hasCustomHandler) {
				e.preventDefault();
				contextMenuStore.open(e.clientX, e.clientY, getDefaultContextMenuItems());
			}
		};

		// Add global context menu handler
		document.addEventListener('contextmenu', handleGlobalContextMenu);

		// Cleanup
		return () => {
			document.removeEventListener('contextmenu', handleGlobalContextMenu);
		};
	});
</script>

{@render children()}
