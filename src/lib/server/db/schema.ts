import {
  pgTable,
  serial,
  varchar,
  text,
  integer,
  real,
  bigint,
  pgEnum,
} from 'drizzle-orm/pg-core';
import { relations } from 'drizzle-orm';

/**
 * ENUMS
 * Enums provide a typesafe way to manage predefined sets of values.
 */
export const dataSourceEnum = pgEnum('data_source', ['anilist', 'myanimelist']);

/**
 * TABLE: users
 * Stores user account information.
 */
export const users = pgTable('userdata', {
  id: serial('id').primaryKey(),
  username: varchar('username', { length: 256 }).notNull().unique(),
  profileImage: text('profile_image'), // URL to cached image
  episodesWatched: integer('episodes_watched').notNull().default(0),
  chaptersRead: integer('chapters_read').notNull().default(0),
});

/**
 * TABLE: media
 * Stores general and user-specific information about an anime or manga.
 */
export const media = pgTable('media', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 256 }).notNull(),
  synopsis: text('synopsis'),
  cover: text('cover'), // URL to cached cover image
  genres: text('genres').array(),
  totalUnits: integer('total_units'), // Total episodes or chapters
  extensionSource: varchar('extension_source', { length: 256 }).unique().notNull(),
  dataSource: dataSourceEnum('data_source').notNull(),
	progress: integer('progress'), // Episodes/chapters watched/read
	lastUnit: integer('last_unit'), // Last episode/chapter number watched
	lastTimestampPercentage: real('last_timestamp_percentage'), // e.g., 0.85 for 85%
});

/**
 * TABLE: downloads
 * Stores information about downloaded media files.
 */
export const downloads = pgTable('downloads', {
  id: serial('id').primaryKey(),
  mediaId: integer('media_id')
    .notNull()
    .references(() => media.id, { onDelete: 'cascade' }),
  title: varchar('title', { length: 256 }), // Denormalized for quick access
  cover: text('cover'), // Denormalized for quick access
  link: text('link').notNull(), // Path to the downloaded file
  unitNumber: integer('unit_number').notNull(), // Episode or chapter number
  sizeInBytes: bigint('size_in_bytes', { mode: 'number' }),
  extensionSource: varchar('extension_source', { length: 256 }).notNull(),
});

/**
 * RELATIONS
 * Defines how tables are related to each other for easier querying.
 */
export const mediaRelations = relations(media, ({ many }) => ({
  downloads: many(downloads),
}));

export const downloadsRelations = relations(downloads, ({ one }) => ({
  media: one(media, {
    fields: [downloads.mediaId],
    references: [media.id],
  }),
}));
