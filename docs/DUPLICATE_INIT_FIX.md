# Duplicate Initialization Fix

## Problem

The authentication check (and other store initializations) were running twice on app load:

```
[Auth Command] Checking authentication status
[Auth Command] Checking authentication status  // <-- Duplicate!
[Auth Command] User is authenticated
[Auth Command] User is authenticated          // <-- Duplicate!
```

## Root Cause

**Hot Module Replacement (HMR)** in development mode was causing `+layout.svelte` to remount, triggering the `onMount` callback multiple times. Since the stores didn't have initialization guards, they would reinitialize every time the layout mounted.

```typescript
// +layout.svelte
onMount(async () => {
	await themeStore.initialize(); // Called again on HMR
	await configStore.init(); // Called again on HMR
	await authStore.init(); // Called again on HMR ❌
});
```

## Solution

Added **initialization guards** to all three stores to prevent multiple initializations:

### 1. Auth Store (`src/lib/stores/auth.ts`)

```typescript
function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>(initialState);

	// ✅ Guard to prevent multiple initializations
	let isInitialized = false;

	return {
		subscribe,
		async init() {
			// Prevent multiple initializations
			if (isInitialized) {
				console.log('[AuthStore] Already initialized, skipping');
				return;
			}

			console.log('[AuthStore] Initializing');
			isInitialized = true;
			// ... rest of initialization
		},
	};
}
```

### 2. Config Store (`src/lib/stores/config.ts`)

```typescript
function createConfigStore() {
	const { subscribe, set, update } = writable<AppConfig | null>(null);

	// ✅ Guard to prevent multiple initializations
	let isInitialized = false;

	return {
		subscribe,
		async init() {
			// Prevent multiple initializations
			if (isInitialized) {
				console.log('[Config] Already initialized, skipping');
				return;
			}

			isInitialized = true;
			// ... rest of initialization
		},
	};
}
```

### 3. Theme Store (`src/lib/stores/theme.svelte.ts`)

```typescript
async initialize() {
    // ✅ Prevent multiple initializations
    if (state.initialized) {
        console.log('[ThemeStore] Already initialized, skipping');
        return;
    }

    console.log('[ThemeStore] Initializing...');
    // ... rest of initialization
}
```

## Results

### Before

```
[AuthStore] Initializing
[Auth Command] Checking authentication status
[AuthStore] Initializing                      // ❌ Duplicate
[Auth Command] Checking authentication status // ❌ Duplicate
```

### After

```
[AuthStore] Initializing
[Auth Command] Checking authentication status
[AuthStore] Already initialized, skipping     // ✅ Skipped
```

## Benefits

1. **No duplicate backend calls** - Saves API requests and improves performance
2. **Cleaner logs** - Easier to debug without spam
3. **Proper state management** - Prevents race conditions from multiple initializations
4. **HMR-safe** - Works correctly in development with hot reloading

## Notes

- Guards use closure variables (`isInitialized`) to track state
- Theme store already had `state.initialized` flag, just added explicit logging
- These guards work for both development (HMR) and production
- Cache system still works - first init uses cache if available
