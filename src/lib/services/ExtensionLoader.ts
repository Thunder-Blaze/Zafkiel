/**
 * ExtensionLoader – loads installed JS extensions from disk at runtime.
 *
 * Each extension's `extension.js` file is fetched via Tauri's asset
 * protocol (`asset://`) and evaluated as an ES module.  Errors are
 * surfaced to callers so the store can fire actionable Sonner toasts.
 *
 * # Loading lifecycle
 * 1. `ExtensionLoader.load(id)` checks the extension's installed entry-path
 *    via Rust (`get_extension_entry_path`).
 * 2. The JS module is dynamically imported.
 * 3. The module must export a `default` object (or named export matching
 *    the extension id in camelCase) implementing `SourceExtension` or similar.
 * 4. On failure, a structured `ExtensionLoadError` is thrown.
 */

import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import type { ExtensionIndexEntry, SourceExtension } from '$lib/types/extensions';

// ─────────────────────────────────────────────────────────────────────────────
// Error types
// ─────────────────────────────────────────────────────────────────────────────

export type ExtensionErrorKind =
	| 'entry-missing'     // manifest.entry file not found on disk
	| 'import-failed'     // dynamic import threw (syntax / runtime error)
	| 'no-export'         // JS module has no `default` export
	| 'version-mismatch'  // minAppVersion > current app version
	| 'unknown';

export class ExtensionLoadError extends Error {
	constructor(
		public readonly extensionId: string,
		public readonly kind: ExtensionErrorKind,
		message: string,
		public readonly canReinstall: boolean = true,
	) {
		super(message);
		this.name = 'ExtensionLoadError';
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

class ExtensionLoaderService {
	/** Already-loaded JS module instances, keyed by extension id. */
	private readonly loaded = new Map<string, SourceExtension>();

	/**
	 * Loads an installed extension by ID.
	 *
	 * Returns the cached instance on subsequent calls.
	 * Throws `ExtensionLoadError` on any failure.
	 */
	async load(entry: ExtensionIndexEntry): Promise<SourceExtension> {
		if (this.loaded.has(entry.id)) {
			return this.loaded.get(entry.id)!;
		}

		// 1. Resolve absolute entry-point path via Tauri
		let absolutePath: string;
		try {
			absolutePath = await invoke<string>('get_extension_entry_path', { id: entry.id });
		} catch (err) {
			throw new ExtensionLoadError(
				entry.id,
				'entry-missing',
				`Extension "${entry.name}": entry file is missing. Try reinstalling. (${err})`,
				true,
			);
		}

		// 2. Convert to an asset:// URL the webview can fetch
		const assetUrl = convertFileSrc(absolutePath);

		// 3. Dynamically import the module
		let mod: Record<string, unknown>;
		try {
			mod = await import(/* @vite-ignore */ assetUrl);
		} catch (err) {
			const msg = err instanceof Error ? err.message : String(err);
			throw new ExtensionLoadError(
				entry.id,
				'import-failed',
				`Extension "${entry.name}": failed to load module — ${msg}`,
				true,
			);
		}

		// 4. Expect a `default` export
		const instance = (mod.default ?? mod[entry.id]) as SourceExtension | undefined;
		if (!instance || typeof instance !== 'object') {
			throw new ExtensionLoadError(
				entry.id,
				'no-export',
				`Extension "${entry.name}": module has no valid default export. ` +
					`Make sure extension.js exports a class instance as \`export default\`.`,
				false,
			);
		}

		this.loaded.set(entry.id, instance);
		return instance;
	}

	/** Evicts a cached instance (e.g. after reinstall). */
	evict(id: string): void {
		this.loaded.delete(id);
	}

	/** Whether an extension has already been loaded into memory. */
	isLoaded(id: string): boolean {
		return this.loaded.has(id);
	}

	/** Returns the already-loaded instance, or null if not yet loaded. */
	get(id: string): SourceExtension | null {
		return this.loaded.get(id) ?? null;
	}
}

export const ExtensionLoader = new ExtensionLoaderService();
