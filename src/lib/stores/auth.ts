/**
 * Authentication Store
 * Manages authentication state and user profile
 */

import { writable, derived } from 'svelte/store';
import { checkAuthStatus, completeOAuthFlow, logout as authLogout } from '$lib/services/auth';
import { anilistApi } from '$lib/services/anilist';
import type { User } from '$lib/types/anilist';
import { loadAuthCache, saveAuthCache, clearAuthCache } from './sessionCache';

interface AuthState {
	isAuthenticated: boolean;
	isLoading: boolean;
	user: User | null;
	error: string | null;
}

const initialState: AuthState = {
	isAuthenticated: false,
	isLoading: true,
	user: null,
	error: null,
};

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	// Guard to prevent multiple initializations
	let isInitialized = false;

	return {
		subscribe,

		/**
		 * Initialize auth store
		 * Checks authentication status on app startup
		 * Uses cached state to avoid unnecessary API calls
		 */
		async init() {
			// Prevent multiple initializations
			if (isInitialized) {
				console.log('[AuthStore] Already initialized, skipping');
				return;
			}

			console.log('[AuthStore] Initializing');
			isInitialized = true;

			// Try to load cached state first
			const cached = loadAuthCache();
			if (cached) {
				set({
					isAuthenticated: cached.isAuthenticated,
					isLoading: false,
					user: cached.user,
					error: null,
				});
				console.log('[AuthStore] Using cached auth state');
				return; // Skip API call
			}

			// No cache, check auth status
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				const isAuthed = await checkAuthStatus();

				if (isAuthed) {
					// Fetch user profile
					const userResponse = await anilistApi.user.getCurrent();
					if (userResponse.success && userResponse.data) {
						set({
							isAuthenticated: true,
							isLoading: false,
							user: userResponse.data,
							error: null,
						});
						// Save to cache
						saveAuthCache(true, userResponse.data);
						console.log('[AuthStore] User authenticated:', userResponse.data.name);
					} else {
						throw new Error(userResponse.error || 'Failed to fetch user profile');
					}
				} else {
					set({
						isAuthenticated: false,
						isLoading: false,
						user: null,
						error: null,
					});
					// Save to cache
					saveAuthCache(false, null);
					console.log('[AuthStore] User not authenticated');
				}
			} catch (error) {
				console.error('[AuthStore] Initialization error:', error);
				set({
					isAuthenticated: false,
					isLoading: false,
					user: null,
					error: error instanceof Error ? error.message : 'Unknown error',
				});
			}
		},

		/**
		 * Start OAuth login flow
		 */
		async login() {
			console.log('[AuthStore] Starting login');
			update((state) => ({ ...state, isLoading: true, error: null }));

			try {
				// Complete OAuth flow (opens browser, waits for callback)
				await completeOAuthFlow();

				// Fetch user profile
				const userResponse = await anilistApi.user.getCurrent();
				if (userResponse.success && userResponse.data) {
					set({
						isAuthenticated: true,
						isLoading: false,
						user: userResponse.data,
						error: null,
					});
					// Save to cache
					saveAuthCache(true, userResponse.data);
					console.log('[AuthStore] Login successful:', userResponse.data.name);
				} else {
					throw new Error(userResponse.error || 'Failed to fetch user profile');
				}
			} catch (error) {
				console.error('[AuthStore] Login error:', error);
				update((state) => ({
					...state,
					isLoading: false,
					error: error instanceof Error ? error.message : 'Login failed',
				}));
				throw error;
			}
		},

		/**
		 * Logout user
		 */
		async logout() {
			console.log('[AuthStore] Logging out');
			try {
				await authLogout();
				set({
					isAuthenticated: false,
					isLoading: false,
					user: null,
					error: null,
				});
				// Clear cached state
				clearAuthCache();
				console.log('[AuthStore] Logout successful');
			} catch (error) {
				console.error('[AuthStore] Logout error:', error);
				throw error;
			}
		},

		/**
		 * Clear error
		 */
		clearError() {
			update((state) => ({ ...state, error: null }));
		},
	};
}

export const authStore = createAuthStore();

// Derived stores for convenience
export const isAuthenticated = derived(authStore, ($auth) => $auth.isAuthenticated);
export const currentUser = derived(authStore, ($auth) => $auth.user);
export const authLoading = derived(authStore, ($auth) => $auth.isLoading);
export const authError = derived(authStore, ($auth) => $auth.error);
