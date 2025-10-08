import type { ContextMenuItem } from '$lib/stores/context-menu';
import { ConfigService } from '$lib/services/config';

/**
 * Default context menu items for the application
 * These will be shown when no specific context menu is set
 */
export function getDefaultContextMenuItems(
	navigate?: (path: string) => void | Promise<void>
): ContextMenuItem[] {
	return [
		{
			id: 'reload',
			label: 'Reload',
			icon: 'solar:refresh-bold',
			shortcut: '⌘R',
			onClick: () => {
				window.location.reload();
			},
		},
		{
			id: 'sep1',
			label: '',
			separator: true,
		},
		{
			id: 'back',
			label: 'Back',
			icon: 'solar:arrow-left-bold',
			onClick: () => {
				window.history.back();
			},
		},
		{
			id: 'forward',
			label: 'Forward',
			icon: 'solar:arrow-right-bold',
			onClick: () => {
				window.history.forward();
			},
		},
		{
			id: 'settings',
			label: 'Settings',
			icon: 'solar:settings-bold',
			onClick: () => {
				// Use SvelteKit's goto for client-side navigation (no reload)
				if (navigate) {
					navigate('/settings');
				}
			},
		},
		{
			id: 'sep2',
			label: '',
			separator: true,
		},
		{
			id: 'devtools',
			label: 'Inspect Element',
			icon: 'solar:code-bold',
			shortcut: '⌘⌥I',
			onClick: async () => {
				try {
					await ConfigService.openDevtools();
				} catch (error) {
					console.warn('Failed to open devtools:', error);
					// Silently fail in browser mode
				}
			},
		},
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
