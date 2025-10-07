/**
 * Authentication Service
 * Handles AniList OAuth flow on the frontend
 */

import { invoke } from '@tauri-apps/api/core';

export interface AuthCallbackData {
	authUrl: string;
	port: number;
}

/**
 * Start OAuth flow
 * Returns the authorization URL and callback port
 */
export async function startOAuthFlow(): Promise<AuthCallbackData> {
	console.log('[Auth] Starting OAuth flow');
	const [authUrl, port] = await invoke<[string, number]>('start_oauth_flow');
	console.log('[Auth] OAuth flow started, port:', port);
	return { authUrl, port };
}

/**
 * Open authorization URL in browser
 * Falls back to webview if no browser available
 */
export async function openAuthBrowser(authUrl: string): Promise<void> {
	console.log('[Auth] Opening browser for authorization');
	await invoke('open_auth_browser', { authUrl });
}

/**
 * Wait for OAuth callback and exchange code for token
 * This will block until the user completes authorization or timeout (5 minutes)
 */
export async function waitForOAuthCallback(): Promise<string> {
	console.log('[Auth] Waiting for OAuth callback');
	const token = await invoke<string>('wait_for_oauth_callback');
	console.log('[Auth] OAuth callback received, token obtained');
	return token;
}

/**
 * Check if user is authenticated
 * Makes a test request to get user profile
 */
export async function checkAuthStatus(): Promise<boolean> {
	console.log('[Auth] Checking authentication status');
	try {
		const isAuthed = await invoke<boolean>('check_auth_status');
		console.log('[Auth] Authentication status:', isAuthed);
		return isAuthed;
	} catch (error) {
		console.error('[Auth] Failed to check auth status:', error);
		return false;
	}
}

/**
 * Logout - clear stored token
 */
export async function logout(): Promise<void> {
	console.log('[Auth] Logging out');
	await invoke('logout');
	console.log('[Auth] Logout successful');
}

/**
 * Complete OAuth flow
 * - Starts OAuth flow
 * - Opens browser
 * - Waits for callback
 * - Returns token
 */
export async function completeOAuthFlow(): Promise<string> {
	console.log('[Auth] Starting complete OAuth flow');

	// Start OAuth flow and get authorization URL
	const { authUrl } = await startOAuthFlow();

	// Open browser (or fallback to webview)
	await openAuthBrowser(authUrl);

	// Wait for callback and exchange code for token
	const token = await waitForOAuthCallback();

	console.log('[Auth] Complete OAuth flow finished successfully');
	return token;
}
