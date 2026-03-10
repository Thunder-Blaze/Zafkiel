/**
 * Tracks whether the fullscreen mpv player overlay is active.
 * When true, the layout hides non-player UI so the native video
 * layer can show through the transparent WebView.
 */
let isPlayerActive = $state(false);

export const playerStore = {
	get active() {
		return isPlayerActive;
	},
	show() {
		isPlayerActive = true;
	},
	hide() {
		isPlayerActive = false;
	},
};
