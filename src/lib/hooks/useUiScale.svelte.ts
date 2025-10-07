import { ConfigService } from '$lib/services/config';

/**
 * Hook to manage UI scale using Tauri's webview zoom
 * This scales EVERYTHING including native dropdowns and context menus
 */
export function useUiScale() {
	let currentScale = $state(1.0);
	let isInitialized = $state(false);

	// Apply scale on initialization
	$effect(() => {
		if (typeof window !== 'undefined' && !isInitialized) {
			isInitialized = true;
			
			// Apply zoom from config
			ConfigService.applyUiScale()
				.then(() => {
					// Get the current scale value for display
					return ConfigService.getUiConfig();
				})
				.then(config => {
					currentScale = config.ui_scale;
				})
				.catch(error => {
					console.warn('Failed to apply UI scale on startup:', error);
				});
		}
	});

	/**
	 * Update the UI scale (saves to config and applies webview zoom)
	 */
	async function setScale(scale: number): Promise<void> {
		const clampedScale = Math.max(0.5, Math.min(2.0, scale));
		
		try {
			// This will update config AND apply zoom via Tauri
			await ConfigService.updateUiScale(clampedScale);
			currentScale = clampedScale;
		} catch (error) {
			console.error('Failed to update UI scale:', error);
			throw error;
		}
	}

	/**
	 * Reset to default scale (1.0)
	 */
	async function resetScale(): Promise<void> {
		await setScale(1.0);
	}

	return {
		get scale() {
			return currentScale;
		},
		setScale,
		resetScale
	};
}
