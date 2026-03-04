# Auth State Caching Implementation

## Overview

Implemented sessionStorage caching for authentication state to prevent unnecessary API calls on page refresh. This reduces server load and improves app responsiveness.

## Implementation Details

### Cache Configuration

- **Storage**: `sessionStorage` (clears on tab close, persists on refresh)
- **Cache Key**: `'zafkiel_auth_state'`
- **TTL (Time To Live)**: 5 minutes (300,000 milliseconds)
- **Browser Check**: Only runs in browser environment (safe for SSR)

### Data Structure

```typescript
interface CachedAuthState {
	isAuthenticated: boolean;
	user: User | null;
	timestamp: number; // Unix timestamp in milliseconds
}
```

### Core Functions

#### 1. `loadCachedState()`

- **Purpose**: Load and validate cached authentication state
- **Behavior**:
  - Returns `null` if not in browser environment
  - Returns `null` if no cached data exists
  - Returns `null` if cached data is older than 5 minutes
  - Returns cached state if valid
- **Error Handling**: Catches JSON parse errors and returns `null`

#### 2. `saveCachedState(isAuthenticated: boolean, user: User | null)`

- **Purpose**: Save current authentication state to sessionStorage
- **Behavior**:
  - Only runs in browser environment
  - Saves state with current timestamp
  - Serializes to JSON
- **Error Handling**: Silently catches and logs storage errors

#### 3. `clearCachedState()`

- **Purpose**: Remove cached authentication state
- **Behavior**:
  - Only runs in browser environment
  - Removes cache key from sessionStorage
- **Error Handling**: Silently catches and logs removal errors

### Integration Points

#### `init()` Method

**Before**: Always called `checkAuthStatus()` on every page load

**After**:

1. Checks `loadCachedState()` first
2. If cached state is valid (< 5 minutes old):
   - Uses cached data
   - Skips API call
   - Logs: `[AuthStore] Using cached auth state`
3. If no cache or expired:
   - Calls `checkAuthStatus()` API
   - Saves result with `saveCachedState()`
   - Logs: `[AuthStore] User authenticated: {name}` or `[AuthStore] User not authenticated`

#### `login()` Method

**Before**: Did not cache authentication result

**After**:

- After successful OAuth flow and user profile fetch
- Calls `saveCachedState(true, userResponse.data)`
- Ensures subsequent page loads use cached data

#### `logout()` Method

**Before**: Did not clear cached state

**After**:

- After successful logout
- Calls `clearCachedState()`
- Ensures user must re-authenticate on next login

## Benefits

### Performance

- **Reduced API Calls**: Up to 1 API call per 5 minutes instead of every page load
- **Faster Page Loads**: No waiting for authentication check on cached data
- **Server Load**: Significant reduction in authentication endpoint traffic

### User Experience

- **Instant Auth State**: Page loads show correct auth state immediately
- **Smooth Navigation**: No authentication loading state on page refresh
- **Session Persistence**: Auth state survives page refreshes for 5 minutes

### Developer Experience

- **Clear Logging**: Console logs show when cache is used vs API called
- **Easy Debugging**: `[AuthStore]` prefix on all auth-related logs
- **Type Safety**: Full TypeScript typing for cached state

## Cache Invalidation Strategy

### Automatic Invalidation

- **Time-based**: Cache expires after 5 minutes
- **Logout**: Cache cleared immediately on logout
- **Session End**: Cache cleared when browser tab closes (sessionStorage)

### Manual Invalidation

If needed, can be cleared programmatically:

```typescript
sessionStorage.removeItem('zafkiel_auth_state');
```

Or through browser DevTools:

1. Open DevTools (F12)
2. Go to Application tab
3. Navigate to Session Storage → localhost
4. Delete `zafkiel_auth_state` key

## Testing

### Verify Cache Usage

1. Open app and login
2. Open DevTools Console
3. Refresh page
4. Check logs:
   - **With cache**: `[AuthStore] Using cached auth state`
   - **Without cache**: `[Auth Command] Checking authentication status`

### Verify Cache Expiration

1. Login and wait 5+ minutes
2. Refresh page
3. Should see fresh API call: `[Auth Command] Checking authentication status`

### Verify Cache Clearing

1. Login
2. Verify cache exists in DevTools → Application → Session Storage
3. Logout
4. Check cache is removed from Session Storage

## Logging Examples

### First Load (No Cache)

```
[AuthStore] Initializing
[Auth Command] Checking authentication status
[Auth Command] User is authenticated
[AuthStore] User authenticated: YourUsername
```

### Subsequent Loads (With Cache)

```
[AuthStore] Initializing
[AuthStore] Using cached auth state
```

### Cache Expired

```
[AuthStore] Initializing
[Auth Command] Checking authentication status
[Auth Command] User is authenticated
[AuthStore] User authenticated: YourUsername
```

### Logout

```
[AuthStore] Logging out
[Auth Command] Logging out
[AuthStore] Logout successful
```

## Future Improvements

### Potential Enhancements

- **Configurable TTL**: Allow users to adjust cache duration in settings
- **Background Refresh**: Silently refresh cache before expiration
- **Offline Support**: Use cached state even when offline
- **User Profile Cache**: Separately cache user profile data with longer TTL
- **Cache Versioning**: Add version number to detect schema changes

### Monitoring

- **Cache Hit Rate**: Track how often cache is used vs API called
- **Performance Metrics**: Measure page load time improvement
- **Error Tracking**: Monitor cache-related errors

## Related Files

- `src/lib/stores/auth.ts` - Main implementation
- `src/lib/services/auth.ts` - Auth service called by store
- `src/lib/types/anilist.ts` - User type definition

## Related Issues

- Resolves: Images re-downloading issue (separate fix)
- Addresses: Repeated authentication checks on page refresh
- Improves: Overall app performance and user experience
