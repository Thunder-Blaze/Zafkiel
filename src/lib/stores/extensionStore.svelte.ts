/**
 * Extension store – reactive state for the extension lifecycle.
 *
 * Svelte 5 runes store that tracks:
 * - The status of every known extension (catalog + installed)
 * - Install / uninstall / reinstall operations with progress
 * - Automatic error toasts via svelte-sonner
 *
 * # Usage
 * ```svelte
 * <script>
 *   import { extensionStore } from '$lib/stores/extensionStore.svelte';
 *   extensionStore.init();   // call once in layout
 * </script>
 * ```
 */

import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import { ExtensionLoader, type ExtensionLoadError } from '$lib/services/ExtensionLoader';
import { EXTENSION_CATALOG } from '$lib/services/extensionCatalog';
import type { ExtensionIndexEntry, ExtensionStatus } from '$lib/types/extensions';

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

function toastError(title: string, description: string, actionLabel?: string, onAction?: () => void) {
	if (actionLabel && onAction) {
		toast.error(title, {
			description,
			action: { label: actionLabel, onClick: onAction },
			duration: 8000,
		});
	} else {
		toast.error(title, { description, duration: 8000 });
	}
}

function toastSuccess(message: string) {
	toast.success(message, { duration: 3000 });
}

// ─────────────────────────────────────────────────────────────────────────────
// Store
// ─────────────────────────────────────────────────────────────────────────────

class ExtensionStore {
	/** Status keyed by extension id. */
	statuses = $state<Map<string, ExtensionStatus>>(new Map());

	/** Whether the initial load from index.json has completed. */
	initialized = $state(false);

	/** Ids of extensions that are currently being acted on (for spinner). */
	busy = $state<Set<string>>(new Set());

	// ── Init ──────────────────────────────────────────────────────────────────

	/**
	 * Populates status from the on-disk index.  Should be called once
	 * (e.g. in the root layout's `onMount`).
	 */
	async init(): Promise<void> {
		if (this.initialized) return;

		// Pre-populate catalog extensions as 'uninstalled'
		for (const ext of EXTENSION_CATALOG) {
			if (!this.statuses.has(ext.id)) {
				this.statuses.set(ext.id, { kind: 'uninstalled' });
			}
		}

		// Read installed extensions from Rust index
		try {
			const installed = await invoke<ExtensionIndexEntry[]>('get_installed_extensions');
			for (const entry of installed) {
				this.statuses.set(entry.id, { kind: 'installed', entry });
			}
		} catch (err) {
			console.error('[ExtensionStore] Failed to read installed extensions index:', err);
			toast.warning('Extensions', {
				description: 'Could not read extension index. Some extensions may be unavailable.',
				duration: 5000,
			});
		}

		this.initialized = true;
	}

	// ── Queries ───────────────────────────────────────────────────────────────

	getStatus(id: string): ExtensionStatus {
		return this.statuses.get(id) ?? { kind: 'uninstalled' };
	}

	isInstalled(id: string): boolean {
		const s = this.getStatus(id);
		return s.kind === 'installed' || s.kind === 'ready' || s.kind === 'loading' || s.kind === 'error';
	}

	isBusy(id: string): boolean {
		return this.busy.has(id);
	}

	// ── Install ───────────────────────────────────────────────────────────────

	async install(id: string, downloadUrl: string): Promise<boolean> {
		const catalog = EXTENSION_CATALOG.find((e) => e.id === id);
		const label = catalog?.name ?? id;

		if (this.busy.has(id)) return false;

		this.busy.add(id);
		this.statuses.set(id, { kind: 'downloading', progress: null });

		try {
			const entry = await invoke<ExtensionIndexEntry>('install_extension', {
				id,
				downloadUrl,
			});

			this.statuses.set(id, { kind: 'installed', entry });
			toastSuccess(`${label} installed successfully`);
			return true;
		} catch (err) {
			const message = err instanceof Error ? err.message : String(err);
			this.statuses.set(id, {
				kind: 'error',
				message,
				canReinstall: false, // no bundle yet since install failed
			});
			toastError(
				`Failed to install ${label}`,
				message,
				'Retry',
				() => void this.install(id, downloadUrl),
			);
			return false;
		} finally {
			this.busy.delete(id);
		}
	}

	// ── Uninstall ─────────────────────────────────────────────────────────────

	async uninstall(id: string): Promise<boolean> {
		const catalog = EXTENSION_CATALOG.find((e) => e.id === id);
		const label = catalog?.name ?? id;

		if (this.busy.has(id)) return false;

		this.busy.add(id);
		try {
			await invoke<void>('uninstall_extension', { id });
			ExtensionLoader.evict(id);
			this.statuses.set(id, { kind: 'uninstalled' });
			toastSuccess(`${label} uninstalled`);
			return true;
		} catch (err) {
			const message = err instanceof Error ? err.message : String(err);
			toastError(`Failed to uninstall ${label}`, message);
			return false;
		} finally {
			this.busy.delete(id);
		}
	}

	// ── Reinstall ─────────────────────────────────────────────────────────────

	async reinstall(id: string): Promise<boolean> {
		const catalog = EXTENSION_CATALOG.find((e) => e.id === id);
		const label = catalog?.name ?? id;

		if (this.busy.has(id)) return false;

		this.busy.add(id);
		// Show 'loading' state while re-extracting
		const prev = this.statuses.get(id);
		const prevEntry = prev && 'entry' in prev ? prev.entry : undefined;
		this.statuses.set(id, { kind: 'loading' });
		ExtensionLoader.evict(id);

		try {
			const entry = await invoke<ExtensionIndexEntry>('reinstall_extension', { id });
			this.statuses.set(id, { kind: 'installed', entry });
			toastSuccess(`${label} reinstalled`);
			return true;
		} catch (err) {
			const message = err instanceof Error ? err.message : String(err);
			// Restore previous status if possible
			if (prevEntry) {
				this.statuses.set(id, { kind: 'error', entry: prevEntry, message, canReinstall: false });
			} else {
				this.statuses.set(id, { kind: 'error', message, canReinstall: false });
			}
			toastError(`Failed to reinstall ${label}`, message);
			return false;
		} finally {
			this.busy.delete(id);
		}
	}

	// ── Load into memory (lazy) ───────────────────────────────────────────────

	/**
	 * Dynamically loads the extension JS module into memory if not already done.
	 *
	 * Call this just before the extension is actually needed
	 * (e.g. when the user taps "Watch" on an anime page).
	 */
	async loadExtension(id: string): Promise<boolean> {
		const status = this.getStatus(id);
		if (status.kind === 'ready') return true;
		if (status.kind !== 'installed') return false;

		const { entry } = status;
		this.statuses.set(id, { kind: 'loading' });

		try {
			await ExtensionLoader.load(entry);
			this.statuses.set(id, { kind: 'ready', entry });
			return true;
		} catch (err) {
			const loadErr = err as ExtensionLoadError;
			const message = loadErr.message ?? String(err);
			const canReinstall = loadErr.canReinstall ?? true;
			const catalog = EXTENSION_CATALOG.find((e) => e.id === id);
			const downloadUrl = catalog?.downloadUrl;

			this.statuses.set(id, { kind: 'error', entry, message, canReinstall });

			toastError(
				`${entry.name} failed to load`,
				message,
				canReinstall ? 'Reinstall' : undefined,
				canReinstall
					? () => {
							// For reinstall: try re-extracting from cached bundle first,
							// then fall back to downloading again if the bundle is gone.
							void this.reinstall(id).then((ok) => {
								if (!ok && downloadUrl) void this.install(id, downloadUrl);
							});
					  }
					: undefined,
			);

			return false;
		}
	}

	// ── Derived queries ───────────────────────────────────────────────────────

	get installedCount(): number {
		let count = 0;
		for (const [, s] of this.statuses) {
			if (s.kind === 'installed' || s.kind === 'ready' || s.kind === 'loading' || s.kind === 'error') {
				count++;
			}
		}
		return count;
	}
}

export const extensionStore = new ExtensionStore();
