/**
 * Authentication Store
 * Manages authentication state and user profile
 */

import { writable, derived } from 'svelte/store';
import { checkAuthStatus, completeOAuthFlow, logout as authLogout } from '$lib/services/auth';
import { anilistApi } from '$lib/services/anilist';
import type { User } from '$lib/types/anilist';

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

	return {
		subscribe,

		/**
		 * Initialize auth store
		 * Checks authentication status on app startup
		 */
		async init() {
			console.log('[AuthStore] Initializing');
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
