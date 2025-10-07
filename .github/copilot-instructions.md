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
