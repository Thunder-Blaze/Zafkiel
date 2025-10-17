/**
 * Keybindings System
 *
 * Centralized keyboard shortcut management for the application
 * Cross-platform support: Automatically maps modifier keys based on platform
 * - macOS: meta (⌘) <-> Ctrl on Windows/Linux
 * - macOS: alt (⌥) <-> Alt on Windows/Linux
 */

// ============================================================================
// Platform Detection
// ============================================================================

const isMac = typeof navigator !== 'undefined' && /Mac|iPod|iPhone|iPad/.test(navigator.platform);

/**
 * Cross-platform modifier key that represents the primary modifier
 * - macOS: Cmd (⌘)
 * - Windows/Linux: Ctrl
 */
export type PrimaryModifier = 'primary';

/**
 * Cross-platform modifier key that represents the secondary modifier
 * - macOS: Option (⌥)
 * - Windows/Linux: Alt
 */
export type SecondaryModifier = 'secondary';

// ============================================================================
// Types
// ============================================================================

export interface KeyBinding {
	key: string;
	ctrl?: boolean;
	alt?: boolean;
	shift?: boolean;
	meta?: boolean;
	primary?: boolean; // Cross-platform: Cmd on Mac, Ctrl on Win/Linux
	secondary?: boolean; // Cross-platform: Option on Mac, Alt on Win/Linux
	description: string;
	action: () => void | Promise<void>;
}

export interface KeyBindingCategory {
	name: string;
	bindings: KeyBinding[];
}

// ============================================================================
// Key Binding Registry
// ============================================================================

const registeredBindings: KeyBinding[] = [];

/**
 * Normalize key binding to platform-specific modifiers
 * Converts 'primary' and 'secondary' to actual platform keys
 */
function normalizeBinding(binding: KeyBinding): KeyBinding {
	const normalized = { ...binding };

	// Handle 'primary' modifier (Cmd on Mac, Ctrl elsewhere)
	if (binding.primary) {
		if (isMac) {
			normalized.meta = true;
		} else {
			normalized.ctrl = true;
		}
		delete normalized.primary;
	}

	// Handle 'secondary' modifier (Option on Mac, Alt elsewhere)
	if (binding.secondary) {
		normalized.alt = true;
		delete normalized.secondary;
	}

	return normalized;
}

/**
 * Register a new key binding
 */
export function registerKeyBinding(binding: KeyBinding): void {
	// Normalize cross-platform modifiers
	const normalizedBinding = normalizeBinding(binding);

	// Check for conflicts
	const existing = registeredBindings.find((b) => isConflicting(b, normalizedBinding));
	if (existing) {
		console.warn(
			`[KeyBindings] Conflicting key binding: ${formatKeyBinding(normalizedBinding)} already bound to ${existing.description}`,
		);
	}

	registeredBindings.push(normalizedBinding);
	console.log(
		`[KeyBindings] Registered: ${formatKeyBinding(normalizedBinding)} - ${normalizedBinding.description}`,
	);
}

/**
 * Unregister a key binding
 */
export function unregisterKeyBinding(binding: KeyBinding): void {
	const index = registeredBindings.indexOf(binding);
	if (index !== -1) {
		registeredBindings.splice(index, 1);
		console.log(
			`[KeyBindings] Unregistered: ${formatKeyBinding(binding)} - ${binding.description}`,
		);
	}
}

/**
 * Check if two bindings conflict
 * Normalizes both bindings before comparing to ensure accurate conflict detection
 */
function isConflicting(a: KeyBinding, b: KeyBinding): boolean {
	const normA = normalizeBinding(a);
	const normB = normalizeBinding(b);

	return (
		normA.key === normB.key &&
		normA.ctrl === normB.ctrl &&
		normA.alt === normB.alt &&
		normA.shift === normB.shift &&
		normA.meta === normB.meta
	);
}

/**
 * Format key binding for display
 */
export function formatKeyBinding(binding: KeyBinding): string {
	const parts: string[] = [];

	// Show platform-specific symbols
	if (binding.primary) {
		parts.push(isMac ? '⌘' : 'Ctrl');
	}
	if (binding.meta) {
		parts.push(isMac ? '⌘' : 'Win');
	}
	if (binding.ctrl) {
		parts.push('Ctrl');
	}
	if (binding.secondary || binding.alt) {
		parts.push(isMac ? '⌥' : 'Alt');
	}
	if (binding.shift) {
		parts.push('Shift');
	}
	parts.push(binding.key.toUpperCase());

	return parts.join('+');
}

// ============================================================================
// Event Handler
// ============================================================================

/**
 * Handle keyboard event and trigger matching bindings
 */
export function handleKeyPress(event: KeyboardEvent): boolean {
	const key = event.key.toLowerCase();
	const ctrl = event.ctrlKey;
	const alt = event.altKey;
	const shift = event.shiftKey;
	const meta = event.metaKey;

	// Create event binding (normalized bindings are stored in registeredBindings)
	const binding = registeredBindings.find(
		(b) =>
			b.key.toLowerCase() === key &&
			!!b.ctrl === ctrl &&
			!!b.alt === alt &&
			!!b.shift === shift &&
			!!b.meta === meta,
	);

	if (binding) {
		event.preventDefault();
		event.stopPropagation();

		console.log(`[KeyBindings] Triggered: ${formatKeyBinding(binding)} - ${binding.description}`);
		binding.action();

		return true;
	}

	return false;
}

// ============================================================================
// Application Key Bindings
// ============================================================================

export const APP_KEYBINDINGS: KeyBindingCategory[] = [
	{
		name: 'Navigation',
		bindings: [
			{
				key: 'ArrowLeft',
				secondary: true, // Alt on all platforms (Option on Mac)
				description: 'Navigate back',
				action: () => {
					// Will be set dynamically
				},
			},
			{
				key: 'ArrowRight',
				secondary: true, // Alt on all platforms (Option on Mac)
				description: 'Navigate forward',
				action: () => {
					// Will be set dynamically
				},
			},
			{
				key: 'r',
				primary: true, // Cmd on Mac, Ctrl on Windows/Linux
				description: 'Reload current page',
				action: () => {
					// Will be set dynamically
				},
			},
		],
	},
	{
		name: 'Search',
		bindings: [
			{
				key: 'k',
				primary: true, // Cmd on Mac, Ctrl on Windows/Linux
				description: 'Focus search bar',
				action: () => {
					// Will be set dynamically
				},
			},
		],
	},
];

// ============================================================================
// Initialization
// ============================================================================

/**
 * Initialize keybindings system
 * Sets up global event listener
 */
export function initializeKeybindings(): void {
	if (typeof window === 'undefined') return;

	// Register global event listener
	window.addEventListener('keydown', handleKeyPress);
	console.log('[KeyBindings] Initialized');

	// Register all app keybindings
	APP_KEYBINDINGS.forEach((category) => {
		category.bindings.forEach((binding) => {
			registerKeyBinding(binding);
		});
	});
}

/**
 * Cleanup keybindings system
 */
export function cleanupKeybindings(): void {
	if (typeof window === 'undefined') return;

	window.removeEventListener('keydown', handleKeyPress);
	registeredBindings.length = 0;
	console.log('[KeyBindings] Cleaned up');
}

// ============================================================================
// Helper Functions
// ============================================================================

/**
 * Update action for a specific key binding
 */
export function updateKeyBindingAction(
	key: string,
	modifiers: {
		ctrl?: boolean;
		alt?: boolean;
		shift?: boolean;
		meta?: boolean;
		primary?: boolean;
		secondary?: boolean;
	},
	action: () => void | Promise<void>,
): void {
	// Normalize modifiers to match stored bindings
	const tempBinding: KeyBinding = {
		key,
		...modifiers,
		description: '',
		action: () => {}
	};
	const normalized = normalizeBinding(tempBinding);

	const binding = registeredBindings.find(
		(b) =>
			b.key.toLowerCase() === normalized.key.toLowerCase() &&
			!!b.ctrl === !!normalized.ctrl &&
			!!b.alt === !!normalized.alt &&
			!!b.shift === !!normalized.shift &&
			!!b.meta === !!normalized.meta,
	);

	if (binding) {
		binding.action = action;
		console.log(`[KeyBindings] Updated action for: ${formatKeyBinding(binding)}`);
	} else {
		console.warn(
			`[KeyBindings] No binding found for: ${key} with modifiers`,
			modifiers,
		);
	}
}

/**
 * Get all registered bindings
 */
export function getAllBindings(): KeyBinding[] {
	return [...registeredBindings];
}

/**
 * Get bindings by category
 */
export function getBindingsByCategory(): KeyBindingCategory[] {
	return APP_KEYBINDINGS;
}
