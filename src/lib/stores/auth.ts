/**
 * Authentication Store
 * Manages authentication state and user profile
 */

import { writable, derived } from 'svelte/store';
import { checkAuthStatus, completeOAuthFlow, logout as authLogout } from '$lib/services/auth';
import { anilistApi } from '$lib/services/anilist';
import type { User } from '$lib/types/anilist';
import { browser } from '$app/environment';

interface AuthState {
	isAuthenticated: boolean;
	isLoading: boolean;
	user: User | null;
	error: string | null;
}

const STORAGE_KEY = 'zafkiel_auth_state';
const CACHE_DURATION = 5 * 60 * 1000; // 5 minutes

interface CachedAuthState {
	isAuthenticated: boolean;
	user: User | null;
	timestamp: number;
}

// Load cached state from sessionStorage
function loadCachedState(): CachedAuthState | null {
	if (!browser) return null;
	
	try {
		const cached = sessionStorage.getItem(STORAGE_KEY);
		if (!cached) return null;
		
		const state: CachedAuthState = JSON.parse(cached);
		
		// Check if cache is still valid (within CACHE_DURATION)
		const now = Date.now();
		if (now - state.timestamp > CACHE_DURATION) {
			sessionStorage.removeItem(STORAGE_KEY);
			return null;
		}
		
		console.log('[AuthStore] Loaded cached auth state');
		return state;
	} catch (error) {
		console.error('[AuthStore] Failed to load cached state:', error);
		return null;
	}
}

// Save state to sessionStorage
function saveCachedState(isAuthenticated: boolean, user: User | null) {
	if (!browser) return;
	
	try {
		const state: CachedAuthState = {
			isAuthenticated,
			user,
			timestamp: Date.now(),
		};
		sessionStorage.setItem(STORAGE_KEY, JSON.stringify(state));
		console.log('[AuthStore] Saved auth state to cache');
	} catch (error) {
		console.error('[AuthStore] Failed to save cached state:', error);
	}
}

// Clear cached state
function clearCachedState() {
	if (!browser) return;
	
	try {
		sessionStorage.removeItem(STORAGE_KEY);
		console.log('[AuthStore] Cleared cached auth state');
	} catch (error) {
		console.error('[AuthStore] Failed to clear cached state:', error);
	}
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
		 * Uses cached state to avoid unnecessary API calls
		 */
		async init() {
			console.log('[AuthStore] Initializing');
			
			// Try to load cached state first
			const cached = loadCachedState();
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
						saveCachedState(true, userResponse.data);
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
					saveCachedState(false, null);
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
					saveCachedState(true, userResponse.data);
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
				clearCachedState();
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
