import { sqliteTable, AnySQLiteColumn, uniqueIndex, integer, text, foreignKey, real } from "drizzle-orm/sqlite-core"
  import { sql } from "drizzle-orm"

export const cachedImages = sqliteTable("cached_images", {
	id: integer().primaryKey({ autoIncrement: true }).notNull(),
	originalUrl: text("original_url").notNull(),
	localPath: text("local_path").notNull(),
	lastAccessed: integer("last_accessed").notNull(),
},
(table) => [
	uniqueIndex("cached_images_local_path_unique").on(table.localPath),
	uniqueIndex("cached_images_original_url_unique").on(table.originalUrl),
]);

export const downloads = sqliteTable("downloads", {
	id: integer().primaryKey({ autoIncrement: true }).notNull(),
	mediaId: integer("media_id").notNull().references(() => media.id, { onDelete: "cascade" } ),
	title: text().notNull(),
	coverImageId: integer("cover_image_id").references(() => cachedImages.id),
	filePath: text("file_path").notNull(),
	fileSize: integer("file_size"),
	unitNumber: integer("unit_number").notNull(),
	extensionSource: text("extension_source").notNull(),
	downloadUrl: text("download_url"),
	createdAt: integer("created_at").notNull(),
	lastAccessed: integer("last_accessed"),
	isWatched: integer("is_watched").default(false),
});

export const media = sqliteTable("media", {
	id: integer().primaryKey().notNull(),
	type: text().notNull(),
	format: text(),
	status: text(),
	title: text().notNull(),
	description: text(),
	genres: text(),
	totalUnits: integer("total_units"),
	duration: integer(),
	season: text(),
	seasonYear: integer("season_year"),
	startDateYear: integer("start_date_year"),
	coverImageId: integer("cover_image_id").references(() => cachedImages.id),
	bannerImageId: integer("banner_image_id").references(() => cachedImages.id),
	coverColor: text("cover_color"),
	averageScore: integer("average_score"),
	popularity: integer(),
	favourites: integer(),
	isAdult: integer("is_adult").default(false),
	isFavourite: integer("is_favourite").default(false),
	userStatus: text("user_status"),
	userScore: real("user_score"),
	userProgress: integer("user_progress"),
	localProgress: integer("local_progress"),
	lastTimestampWatched: real("last_timestamp_watched"),
	extensionSource: text("extension_source"),
	dataSource: text("data_source").default("anilist").notNull(),
	cachedAt: integer("cached_at").notNull(),
	updatedAt: integer("updated_at"),
});

export const recentlyViewed = sqliteTable("recently_viewed", {
	id: integer().primaryKey({ autoIncrement: true }).notNull(),
	mediaId: integer("media_id").notNull().references(() => media.id, { onDelete: "cascade" } ),
	viewedAt: integer("viewed_at").notNull(),
});

export const searchCache = sqliteTable("search_cache", {
	id: integer().primaryKey({ autoIncrement: true }).notNull(),
	query: text().notNull(),
	mediaType: text("media_type"),
	results: text().notNull(),
	totalCount: integer("total_count"),
	cachedAt: integer("cached_at").notNull(),
	expiresAt: integer("expires_at").notNull(),
});

export const users = sqliteTable("users", {
	id: integer().primaryKey().notNull(),
	name: text().notNull(),
	avatarLarge: text("avatar_large"),
	bannerImage: text("banner_image"),
	avatarImageId: integer("avatar_image_id").references(() => cachedImages.id),
	bannerImageId: integer("banner_image_id").references(() => cachedImages.id),
	animeCount: integer("anime_count").default(0),
	mangaCount: integer("manga_count").default(0),
	episodesWatched: integer("episodes_watched").default(0),
	chaptersRead: integer("chapters_read").default(0),
	cachedAt: integer("cached_at").notNull(),
	updatedAt: integer("updated_at"),
});

