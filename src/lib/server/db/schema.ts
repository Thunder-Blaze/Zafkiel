import { sqliteTable, integer, text, real } from 'drizzle-orm/sqlite-core';
import { relations } from 'drizzle-orm';

/**
 * USERS TABLE
 * Stores current logged-in user information from AniList
 */
export const users = sqliteTable('users', {
	id: integer('id').primaryKey(), // AniList user ID
	name: text('name').notNull(),
	avatar_large: text('avatar_large'),
	banner_image: text('banner_image'),
	// Local cache references
	avatar_image_id: integer('avatar_image_id').references(() => cached_images.id),
	banner_image_id: integer('banner_image_id').references(() => cached_images.id),
	// Statistics (simplified)
	anime_count: integer('anime_count').default(0),
	manga_count: integer('manga_count').default(0),
	episodes_watched: integer('episodes_watched').default(0),
	chapters_read: integer('chapters_read').default(0),
	// Cache metadata
	cached_at: integer('cached_at').notNull(),
	updated_at: integer('updated_at'),
});

/**
 * MEDIA TABLE
 * Stores essential anime/manga information for offline viewing
 */
export const media = sqliteTable('media', {
	id: integer('id').primaryKey(), // AniList media ID
	type: text('type', { enum: ['ANIME', 'MANGA'] }).notNull(),
	format: text('format'),
	status: text('status'),
	// Single title (user preferred)
	title: text('title').notNull(),
	// Essential content info
	description: text('description'),
	genres: text('genres'), // JSON array
	// Unit info (episodes or chapters)
	total_units: integer('total_units'), // episodes for anime, chapters for manga
	duration: integer('duration'), // Episode duration in minutes (for anime only)
	// Images - references to cached_images table
	cover_image_id: integer('cover_image_id').references(() => cached_images.id),
	banner_image_id: integer('banner_image_id').references(() => cached_images.id),
	// Statistics
	average_score: integer('average_score'),
	popularity: integer('popularity'),
	favourites: integer('favourites'),
	is_adult: integer('is_adult', { mode: 'boolean' }).default(false),
	user_status: text('user_status'), // WATCHING, COMPLETED, etc.
	user_score: real('user_score'),
	user_progress: integer('user_progress'), // AniList progress
	local_progress: integer('local_progress'), // Local progress tracking
	last_timestamp_watched: real('last_timestamp_watched'), // Last position in episode/chapter (0.0-1.0)
	// Extension support
	extension_source: text('extension_source'), // Which extension provides this
	data_source: text('data_source').notNull().default('anilist'), // anilist, mal, etc.
	// Cache metadata
	cached_at: integer('cached_at').notNull(),
	updated_at: integer('updated_at'),
});

/**
 * DOWNLOADS TABLE
 * Stores information about downloaded media files
 */
export const downloads = sqliteTable('downloads', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	media_id: integer('media_id').notNull().references(() => media.id, { onDelete: 'cascade' }),
	// Denormalized for quick access
	title: text('title').notNull(),
	cover_image_id: integer('cover_image_id').references(() => cached_images.id),
	// File info
	file_path: text('file_path').notNull(), // Path to downloaded file
	file_size: integer('file_size'), // Size in bytes
	unit_number: integer('unit_number').notNull(), // Episode number for anime, chapter for manga
	// Extension info
	extension_source: text('extension_source').notNull(), // Which extension downloaded this
	download_url: text('download_url'), // Original URL (for re-downloading)
	// Metadata
	created_at: integer('created_at').notNull(),
	last_accessed: integer('last_accessed'),
	is_watched: integer('is_watched', { mode: 'boolean' }).default(false),
});

/**
 * CACHED IMAGES TABLE
 * Tracks locally cached images with cleanup metadata
 */
export const cached_images = sqliteTable('cached_images', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	original_url: text('original_url').notNull().unique(),
	local_path: text('local_path').notNull().unique(),
	file_size: integer('file_size'),
	cached_at: integer('cached_at').notNull(),
	last_accessed: integer('last_accessed').notNull(),
});

/**
 * RECENTLY VIEWED TABLE
 * Stores recently accessed media for quick offline access
 */
export const recently_viewed = sqliteTable('recently_viewed', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	media_id: integer('media_id').notNull().references(() => media.id, { onDelete: 'cascade' }),
	viewed_at: integer('viewed_at').notNull(),
});

/**
 * SEARCH CACHE TABLE
 * Caches search results for offline browsing
 */
export const search_cache = sqliteTable('search_cache', {
	id: integer('id').primaryKey({ autoIncrement: true }),
	query: text('query').notNull(),
	media_type: text('media_type'), // 'ANIME', 'MANGA', or null for both
	results: text('results').notNull(), // JSON array of media IDs
	total_count: integer('total_count'),
	cached_at: integer('cached_at').notNull(),
	expires_at: integer('expires_at').notNull(), // TTL for search cache
});

// ============================================================================
// RELATIONS
// ============================================================================

export const usersRelations = relations(users, ({ one }) => ({
	avatarImage: one(cached_images, {
		fields: [users.avatar_image_id],
		references: [cached_images.id],
	}),
	bannerImage: one(cached_images, {
		fields: [users.banner_image_id],
		references: [cached_images.id],
	}),
}));

export const mediaRelations = relations(media, ({ many, one }) => ({
	downloads: many(downloads),
	recentViews: many(recently_viewed),
	coverImage: one(cached_images, {
		fields: [media.cover_image_id],
		references: [cached_images.id],
	}),
	bannerImage: one(cached_images, {
		fields: [media.banner_image_id],
		references: [cached_images.id],
	}),
}));

export const downloadsRelations = relations(downloads, ({ one }) => ({
	media: one(media, {
		fields: [downloads.media_id],
		references: [media.id],
	}),
	coverImage: one(cached_images, {
		fields: [downloads.cover_image_id],
		references: [cached_images.id],
	}),
}));

export const recentlyViewedRelations = relations(recently_viewed, ({ one }) => ({
	media: one(media, {
		fields: [recently_viewed.media_id],
		references: [media.id],
	}),
}));

export const cachedImagesRelations = relations(cached_images, ({ many }) => ({
	userAvatars: many(users, { relationName: 'avatar' }),
	userBanners: many(users, { relationName: 'banner' }),
	mediaCovers: many(media, { relationName: 'cover' }),
	mediaBanners: many(media, { relationName: 'banner' }),
	downloadCovers: many(downloads),
}));
