# Copilot Instructions - Zafkiel Project

This document provides guidelines for GitHub Copilot when working on the Zafkiel cross-platform anime watching application built with SvelteKit, Tauri, and TypeScript.

## 🎯 Core Principles

### Code Quality Standards

- **Simplicity First**: Write simple, concise code that is easy to understand and maintain
- **Performance Optimized**: Always prioritize performance and efficiency in implementation
- **Best Practices**: Follow industry best practices for Rust, Svelte, and TypeScript
- **Type Safety**: Leverage TypeScript's type system for robust, error-free code

### Technology Stack Guidelines

#### Svelte 5 (Runes Mode)

- **Always use Svelte 5's runes mode** - Reference: https://svelte.dev/
- Use `$state()`, `$derived()`, `$effect()` instead of legacy reactive declarations
- Prefer composition over inheritance with Svelte 5 patterns
- Utilize `$props()` for component properties with proper TypeScript types

```typescript
// ✅ Good - Svelte 5 runes
let count = $state(0);
let doubled = $derived(count * 2);

// ❌ Avoid - Legacy reactive syntax
let count = 0;
$: doubled = count * 2;
```

#### shadcn-svelte Components

- **Use latest shadcn-svelte documentation** - Reference: https://shadcn-svelte.com/docs
- Prefer shadcn-svelte components over custom UI components
- Follow the established design system and component patterns
- Maintain consistent styling with Tailwind CSS classes

#### TypeScript Best Practices

- Define proper interfaces and types for all data structures
- Use strict TypeScript configuration
- Leverage union types, generics, and utility types effectively
- Always provide return types for functions

```typescript
// ✅ Good - Proper typing
interface AnimeData {
	readonly id: number;
	readonly title: string;
	readonly score: number;
	readonly status: 'WATCHING' | 'COMPLETED' | 'PLAN_TO_WATCH' | 'DROPPED' | 'PAUSED';
}

function processAnime(anime: AnimeData): Promise<void> {
	// Implementation
}

// ❌ Avoid - Any types or missing types
function processAnime(anime: any) {
	// Implementation
}
```

#### Rust (Tauri Backend)

- Follow Rust best practices: ownership, borrowing, and lifetimes
- Use `Result<T, E>` for error handling
- Prefer `&str` over `String` when possible
- Implement proper error types and handling

```rust
// ✅ Good - Proper error handling
#[tauri::command]
async fn fetch_anime_data(id: u32) -> Result<AnimeData, String> {
    match api_client.get_anime(id).await {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to fetch anime: {}", e))
    }
}
```

## 🧪 Testing Requirements

### Storybook Stories

- **Always create Storybook stories** for UI components
- Include multiple variants and edge cases
- Use realistic mock data in stories
- Document component props and usage examples

```typescript
// ✅ Good - Comprehensive Storybook story
export const Default: Story = {
	args: {
		anime: mockAnimeData,
		isLoggedIn: true,
		onStatusChange: action('status-changed'),
	},
};

export const LoadingState: Story = {
	args: {
		...Default.args,
		isLoading: true,
	},
};
```

### Vitest Unit Tests

- Write unit tests for utility functions and business logic
- Test error conditions and edge cases
- Use descriptive test names and organize with `describe` blocks

```typescript
// ✅ Good - Comprehensive unit tests
describe('formatAnimeScore', () => {
	it('should format score with one decimal place', () => {
		expect(formatAnimeScore(8.7)).toBe('8.7');
	});

	it('should handle null scores gracefully', () => {
		expect(formatAnimeScore(null)).toBe('N/A');
	});
});
```

### Playwright E2E Tests

- Create end-to-end tests for critical user flows
- Test cross-platform compatibility features
- Include accessibility testing

## 📝 Code Documentation

### Comments Guidelines

- **Only comment complex logic** - Simple code should be self-documenting
- Write concise, meaningful comments that explain "why", not "what"
- Use JSDoc for public APIs and component interfaces

```typescript
// ✅ Good - Explains complex business logic
// Calculate weighted score based on user preferences and community rating
const weightedScore = (userWeight * userRating + communityWeight * communityRating) / totalWeight;

// ❌ Avoid - States the obvious
// Increment counter by 1
counter++;
```

### Function Documentation

```typescript
/**
 * Fetches anime data with caching and error handling
 * @param id - Unique anime identifier
 * @param useCache - Whether to use cached data if available
 * @returns Promise resolving to anime data or error
 */
async function fetchAnimeData(id: number, useCache = true): Promise<AnimeData> {
	// Implementation
}
```

## 🚫 Anti-Patterns to Avoid

### Hardcoding in Application Code

- **Never hardcode data in production code** - Use configuration files, environment variables, or APIs
- Only hardcode in Storybook stories and tests for demonstration purposes

```typescript
// ❌ Avoid - Hardcoded in app
const POPULAR_ANIME = ['One Piece', 'Naruto', 'Attack on Titan'];

// ✅ Good - Dynamic data
const popularAnime = await fetchPopularAnime();
```

### Legacy Patterns

- Avoid Svelte 4 reactive declarations (`$:`)
- Don't use deprecated shadcn-svelte components
- Avoid any types in TypeScript
- Don't use outdated Rust patterns

## 🔧 Project-Specific Guidelines

### File Structure

- Components in `src/lib/components/`
- Utilities in `src/lib/utils/`
- Types in `src/lib/types/`
- Stores in `src/lib/stores/`
- Tauri commands in `src-tauri/src/`

### Naming Conventions

- **Components**: PascalCase (`AnimeCard.svelte`)
- **Files**: kebab-case (`anime-service.ts`)
- **Variables**: camelCase (`animeData`)
- **Constants**: SCREAMING_SNAKE_CASE (`API_BASE_URL`)
- **Types**: PascalCase (`AnimeData`)

### State Management

- Use Svelte 5 runes for local component state
- Implement stores for global application state
- Prefer derived state over manual synchronization

```typescript
// ✅ Good - Svelte 5 state management
let animeList = $state<AnimeData[]>([]);
let filteredAnime = $derived(animeList.filter((anime) => anime.score > 8));

$effect(() => {
	// Side effects when animeList changes
	saveToLocalStorage(animeList);
});
```

## 🔗 Reference Documentation

When uncertain about implementation details, always refer to:

1. **Svelte 5**: https://svelte.dev/docs/svelte/overview
2. **shadcn-svelte**: https://shadcn-svelte.com/docs
3. **SvelteKit**: https://kit.svelte.dev/docs
4. **Tauri**: https://tauri.app/develop/
5. **TypeScript**: https://www.typescriptlang.org/docs/
6. **Vitest**: https://vitest.dev/guide/
7. **Playwright**: https://playwright.dev/docs/intro
8. **Storybook**: https://storybook.js.org/docs
9. **anilist_moe**: https://docs.rs/anilist_moe/0.2.0/anilist_moe/

## 📡 AniList API Integration

### Backend (Rust + Tauri)

The project uses the **`anilist_moe`** crate (v0.2.0) for AniList API interactions:

```rust
// Initialize AniList service in src-tauri/src/anilist.rs
let anilist_service = AniListService::new(None); // Or with token: Some("token")
app.manage(Arc::new(anilist_service));
```

**Key Features:**

- ✅ Type-safe responses - no `serde_json::Value`
- ✅ Complete API coverage (anime, manga, users, social features)
- ✅ Async/await with tokio
- ✅ Authentication support with Bearer tokens
- ✅ Comprehensive error handling
- ✅ Built-in pagination support
- ✅ Rate limiting: 90 requests/minute

**Backend Structure:**

- `src-tauri/src/anilist.rs`: Service layer wrapping AniListClient
- `src-tauri/src/anilist_commands.rs`: Tauri commands for frontend communication
- Token management: Automatically loads encrypted token from config

**Available Endpoints:**

_Anime:_

- `search_anime(query, page?, perPage?)` - Search anime
- `get_anime_by_id(id)` - Get anime details
- `get_trending_anime(page?, perPage?)` - Get trending anime
- `get_popular_anime(page?, perPage?)` - Get popular anime
- `get_seasonal_anime(season, year, page?, perPage?)` - Get seasonal anime

_Manga:_

- `search_manga(query, page?, perPage?)` - Search manga
- `get_manga_by_id(id)` - Get manga details
- `get_trending_manga(page?, perPage?)` - Get trending manga
- `get_popular_manga(page?, perPage?)` - Get popular manga

_Users:_

- `get_current_user()` - Get authenticated user (requires token)
- `get_user_by_id(id)` - Get user by ID
- `get_user_by_name(name)` - Get user by name
- `search_users(query, page?, perPage?)` - Search users

### Frontend (TypeScript + TanStack Query)

**Type Definitions:** `src/lib/types/anilist.ts`

- Mirrors Rust types from anilist_moe crate
- Full type safety for Media, User, and related types

**API Client:** `src/lib/services/anilist.ts`

```typescript
import { anilistApi } from '$lib/services/anilist';

// Usage
const response = await anilistApi.anime.search({ query: 'Steins Gate', page: 1, perPage: 10 });
```

**TanStack Query Hooks:** `src/lib/hooks/useAnilist.svelte.ts`

```typescript
import { useTrendingAnime, useAnimeById, useCurrentUser } from '$lib/hooks/useAnilist.svelte';

// In Svelte component
const trendingQuery = useTrendingAnime({ page: 1, perPage: 20 });
// Access: trendingQuery.data, trendingQuery.isLoading, trendingQuery.error
```

**Caching Strategy:**

- Anime/Manga details: 30 minutes (stable data)
- Trending content: 5 minutes (updates frequently)
- Popular content: 15 minutes
- Seasonal content: 60 minutes (very stable)
- User profiles: 30-60 minutes
- Search results: 10 minutes

**Query Keys Structure:**

```typescript
anilistKeys.anime.search({ query, page, perPage });
anilistKeys.anime.byId(id);
anilistKeys.anime.trending({ page, perPage });
anilistKeys.user.current();
// etc.
```

**Error Handling:**
All hooks automatically throw descriptive errors that TanStack Query can catch:

```typescript
const query = useAnimeById(123);
if (query.error) {
	// Handle error: query.error.message
}
```

**Best Practices:**

1. **Always use hooks instead of direct API calls** - Benefits from caching
2. **Enable/disable queries conditionally** - Pass `enabled` parameter
3. **Don't fetch same data multiple times** - TanStack Query deduplicates automatically
4. **Use query invalidation sparingly** - Trust the staleTime configuration
5. **Handle loading/error states** - Always check `isLoading` and `error`
6. **Extract properties when switching queries** - See reactivity pattern below

**TanStack Query + Svelte 5 Reactivity Pattern:**

```typescript
// ❌ WRONG - Don't switch between query result objects
const displayData = $derived(condition ? queryA : queryB);
// Problem: Svelte can't track nested properties when parent object changes

// ✅ CORRECT - Extract specific properties from the active query
const data = $derived(condition ? queryA.data : queryB.data);
const isLoading = $derived(condition ? queryA.isLoading : queryB.isLoading);
const error = $derived(condition ? queryA.error : queryB.error);
```

**Authentication:**

- Token stored encrypted in config at `~/.config/zafkiel/config.ron`
- Automatically loaded by backend for authenticated requests
- Use `get_current_user` to verify authentication status

**Rate Limiting:**

- AniList enforces 90 requests/minute
- Backend handles rate limit errors gracefully
- Frontend caching significantly reduces API calls
- Consider debouncing search inputs

## ✅ Quality Checklist

Before considering code complete, ensure:

- [ ] Code follows Svelte 5 runes patterns
- [ ] TypeScript types are properly defined
- [ ] Component has Storybook story
- [ ] Critical functionality has tests
- [ ] Complex logic is commented
- [ ] No hardcoded values in production code
- [ ] Performance considerations addressed
- [ ] Accessibility requirements met
- [ ] Cross-platform compatibility verified

## 🎨 UI/UX Guidelines

- Follow the established design system
- Ensure responsive design for desktop platforms
- Implement proper loading and error states
- Use consistent animations and transitions
- Maintain accessibility standards (ARIA labels, keyboard navigation)
- Support both light and dark themes

Remember: The goal is to create a maintainable, performant, and user-friendly cross-platform anime watching application that feels native on all supported platforms.
