import type { ContextMenuItem } from '$lib/stores/context-menu';

/**
 * Default context menu items for the application
 * These will be shown when no specific context menu is set
 */
export function getDefaultContextMenuItems(): ContextMenuItem[] {
	return [
		{
			id: 'reload',
			label: 'Reload',
			icon: 'solar:refresh-bold',
			shortcut: '⌘R',
			onClick: () => {
				window.location.reload();
			}
		},
		{
			id: 'sep1',
			label: '',
			separator: true
		},
		{
			id: 'back',
			label: 'Back',
			icon: 'solar:arrow-left-bold',
			onClick: () => {
				window.history.back();
			}
		},
		{
			id: 'forward',
			label: 'Forward',
			icon: 'solar:arrow-right-bold',
			onClick: () => {
				window.history.forward();
			}
		},
		{
			id: 'sep2',
			label: '',
			separator: true
		},
		{
			id: 'devtools',
			label: 'Inspect Element',
			icon: 'solar:code-bold',
			shortcut: '⌘⌥I',
			onClick: () => {
				// This will be handled by Tauri if in desktop mode
				// In web mode, this won't do anything
			}
		}
	];
}

/**
 * Disable the default browser context menu globally
 * and show our custom context menu instead
 */
export function initializeGlobalContextMenu() {
	// Prevent default context menu on the entire document
	document.addEventListener('contextmenu', (e) => {
		// Only prevent default if no custom handler is set
		// Custom handlers will call preventDefault themselves
		const target = e.target as HTMLElement;
		const hasCustomHandler = target.closest('[data-has-context-menu]');
		
		if (!hasCustomHandler) {
			e.preventDefault();
		}
	});
}
