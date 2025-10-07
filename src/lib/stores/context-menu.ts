import { writable } from 'svelte/store';

export interface ContextMenuItem {
	id: string;
	label: string;
	icon?: string;
	shortcut?: string;
	disabled?: boolean;
	separator?: boolean;
	submenu?: ContextMenuItem[];
	onClick?: () => void | Promise<void>;
}

export interface ContextMenuState {
	isOpen: boolean;
	x: number;
	y: number;
	items: ContextMenuItem[];
}

/**
 * Create a context menu store for managing custom right-click menus
 */
function createContextMenuStore() {
	const { subscribe, set, update } = writable<ContextMenuState>({
		isOpen: false,
		x: 0,
		y: 0,
		items: []
	});

	return {
		subscribe,

		/**
		 * Open context menu at specific coordinates
		 */
		open(x: number, y: number, items: ContextMenuItem[]) {
			set({
				isOpen: true,
				x,
				y,
				items
			});
		},

		/**
		 * Close the context menu
		 */
		close() {
			update(state => ({
				...state,
				isOpen: false
			}));
		},

		/**
		 * Handle menu item click
		 */
		async handleItemClick(item: ContextMenuItem) {
			if (item.disabled) return;

			if (item.onClick) {
				await item.onClick();
			}

			// Close menu after action
			this.close();
		}
	};
}

export const contextMenuStore = createContextMenuStore();
