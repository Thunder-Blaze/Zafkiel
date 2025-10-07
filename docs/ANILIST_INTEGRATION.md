# AniList API Integration

This document describes the AniList API integration in Zafkiel using the `anilist_moe` Rust crate and TanStack Query for caching.

## Overview

The integration provides:

- ✅ Type-safe backend API using anilist_moe crate
- ✅ Tauri commands for frontend communication
- ✅ TanStack Query hooks with intelligent caching
- ✅ Automatic token management from encrypted config
- ✅ Rate limiting awareness (90 requests/minute)

## Architecture

```
Frontend (TypeScript/Svelte)
  ↓ (TanStack Query Hooks)
src/lib/hooks/useAnilist.svelte.ts
  ↓ (Tauri invoke)
src/lib/services/anilist.ts
  ↓ (IPC Bridge)
Backend (Rust/Tauri)
  ↓ (Commands)
src-tauri/src/anilist_commands.rs
  ↓ (Service Layer)
src-tauri/src/anilist.rs
  ↓ (anilist_moe crate)
AniList GraphQL API
```

## Backend Implementation

### Files Created

1. **`src-tauri/src/anilist.rs`** - Service layer
   - Wraps `AniListClient` with token management
   - Provides clean async API for anime/manga/user operations
   - Handles caching at Rust level with Arc<Mutex<>>

2. **`src-tauri/src/anilist_commands.rs`** - Tauri commands
   - Exposes backend functions to frontend via IPC
   - Auto-loads encrypted token from config
   - Returns typed responses with error handling

### Available Backend Commands

#### Anime

- `search_anime(query, page?, perPage?)`
- `get_anime_by_id(id)`
- `get_trending_anime(page?, perPage?)`
- `get_popular_anime(page?, perPage?)`
- `get_seasonal_anime(params)` (fallback to popular)

#### Manga

- `search_manga(query, page?, perPage?)`
- `get_manga_by_id(id)`
- `get_trending_manga(page?, perPage?)`
- `get_popular_manga(page?, perPage?)`

#### Users

- `get_current_user()` (requires auth token)
- `get_user_by_id(id)`
- `get_user_by_name(name)`
- `search_users(query, page?, perPage?)`

## Frontend Implementation

### Files Created

1. **`src/lib/types/anilist.ts`** - TypeScript types
   - Mirrors Rust types from anilist_moe crate
   - Full type safety for Media, User, and all related types

2. **`src/lib/services/anilist.ts`** - API client
   - Wraps Tauri invoke calls
   - Provides clean async API matching backend

3. **`src/lib/hooks/useAnilist.svelte.ts`** - TanStack Query hooks
   - Smart caching with different staleTime per endpoint
   - Query key factories for cache invalidation
   - Automatic error handling and retry logic

4. **`src/routes/anime/+page.svelte`** - Demo page
   - Shows trending anime by default
   - Debounced search with reactive state
   - Cache status display

### Caching Strategy

| Endpoint Type       | Stale Time | Reasoning                       |
| ------------------- | ---------- | ------------------------------- |
| Anime/Manga Details | 30 minutes | Rarely changes                  |
| Trending Content    | 5 minutes  | Updates frequently              |
| Popular Content     | 15 minutes | Moderate change rate            |
| Seasonal Content    | 60 minutes | Very stable                     |
| User Profiles       | 30-60 min  | Infrequent changes              |
| Search Results      | 10 minutes | Balance freshness/cache benefit |

### Usage Example

```typescript
import { useTrendingAnime, useAnimeById } from '$lib/hooks/useAnilist.svelte';

// In Svelte component
const trendingQuery = useTrendingAnime({ page: 1, perPage: 20 });

// Access query state
if (trendingQuery.isLoading) {
	// Show loading state
}

if (trendingQuery.error) {
	// Handle error
}

if (trendingQuery.data) {
	// Display anime data
}

// Conditional queries
const animeQuery = useAnimeById(animeId, shouldFetch);
```

## Configuration

### Token Management

AniList access token is:

1. Stored encrypted in `~/.config/zafkiel/config.ron`
2. Automatically decrypted and loaded by backend
3. Applied to all authenticated requests

Get your token at: https://anilist.co/settings/developer

### Rate Limiting

AniList enforces 90 requests/minute:

- Backend handles rate limit errors gracefully
- Frontend caching significantly reduces API calls
- Consider debouncing search inputs (500ms recommended)
- TanStack Query deduplicates simultaneous requests

## Testing

Visit `/anime` to test the integration:

- View trending anime
- Search anime with debounced input
- Monitor cache status and data freshness
- See loading and error states

## Best Practices

### Do's ✅

- Always use hooks instead of direct API calls
- Trust the configured staleTime values
- Use `enabled` parameter for conditional queries
- Handle loading and error states
- Debounce user input for search

### Don'ts ❌

- Don't manually invalidate queries unless necessary
- Don't fetch same data multiple times
- Don't ignore rate limiting
- Don't use direct invoke() calls
- Don't hardcode API responses

## Notes

1. **Seasonal Anime**: The anilist_moe crate doesn't have direct seasonal anime support. The `get_seasonal_anime` command falls back to popular anime. In production, you'd need to implement filtering logic.

2. **Authentication**: Most endpoints work without authentication, but `get_current_user` requires a valid AniList token.

3. **Type Safety**: The TypeScript types match the Rust types from anilist_moe, ensuring end-to-end type safety.

4. **Error Handling**: All hooks throw descriptive errors that TanStack Query can catch and expose via the `.error` property.

## Future Enhancements

- [ ] Implement proper seasonal anime filtering
- [ ] Add mutation hooks for updating user lists
- [ ] Implement infinite scroll with useInfiniteQuery
- [ ] Add optimistic updates for mutations
- [ ] Cache persistence with localStorage
- [ ] Background refetch on window focus
- [ ] WebSocket for real-time notifications

## References

- anilist_moe crate: https://docs.rs/anilist_moe/0.2.0/anilist_moe/
- AniList API: https://anilist.gitbook.io/anilist-apiv2-docs/
- TanStack Query: https://tanstack.com/query/latest/docs/svelte/overview
- Tauri IPC: https://tauri.app/develop/calling-rust/
