CREATE TABLE `cached_images` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`original_url` text NOT NULL,
	`local_path` text NOT NULL,
	`last_accessed` integer NOT NULL
);
--> statement-breakpoint
CREATE UNIQUE INDEX `cached_images_original_url_unique` ON `cached_images` (`original_url`);--> statement-breakpoint
CREATE UNIQUE INDEX `cached_images_local_path_unique` ON `cached_images` (`local_path`);--> statement-breakpoint
CREATE TABLE `downloads` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`media_id` integer NOT NULL,
	`title` text NOT NULL,
	`cover_image_id` integer,
	`file_path` text NOT NULL,
	`file_size` integer,
	`unit_number` integer NOT NULL,
	`extension_source` text NOT NULL,
	`download_url` text,
	`created_at` integer NOT NULL,
	`last_accessed` integer,
	`is_watched` integer DEFAULT false,
	FOREIGN KEY (`media_id`) REFERENCES `media`(`id`) ON UPDATE no action ON DELETE cascade,
	FOREIGN KEY (`cover_image_id`) REFERENCES `cached_images`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `media` (
	`id` integer PRIMARY KEY NOT NULL,
	`type` text NOT NULL,
	`format` text,
	`status` text,
	`title` text NOT NULL,
	`description` text,
	`genres` text,
	`total_units` integer,
	`duration` integer,
	`season` text,
	`season_year` integer,
	`start_date_year` integer,
	`cover_image_id` integer,
	`banner_image_id` integer,
	`cover_color` text,
	`average_score` integer,
	`popularity` integer,
	`favourites` integer,
	`is_adult` integer DEFAULT false,
	`is_favourite` integer DEFAULT false,
	`user_status` text,
	`user_score` real,
	`user_progress` integer,
	`local_progress` integer,
	`last_timestamp_watched` real,
	`extension_source` text,
	`data_source` text DEFAULT 'anilist' NOT NULL,
	`cached_at` integer NOT NULL,
	`updated_at` integer,
	FOREIGN KEY (`cover_image_id`) REFERENCES `cached_images`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`banner_image_id`) REFERENCES `cached_images`(`id`) ON UPDATE no action ON DELETE no action
);
--> statement-breakpoint
CREATE TABLE `recently_viewed` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`media_id` integer NOT NULL,
	`viewed_at` integer NOT NULL,
	FOREIGN KEY (`media_id`) REFERENCES `media`(`id`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE TABLE `search_cache` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`query` text NOT NULL,
	`media_type` text,
	`results` text NOT NULL,
	`total_count` integer,
	`cached_at` integer NOT NULL,
	`expires_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `users` (
	`id` integer PRIMARY KEY NOT NULL,
	`name` text NOT NULL,
	`avatar_large` text,
	`banner_image` text,
	`avatar_image_id` integer,
	`banner_image_id` integer,
	`anime_count` integer DEFAULT 0,
	`manga_count` integer DEFAULT 0,
	`episodes_watched` integer DEFAULT 0,
	`chapters_read` integer DEFAULT 0,
	`cached_at` integer NOT NULL,
	`updated_at` integer,
	FOREIGN KEY (`avatar_image_id`) REFERENCES `cached_images`(`id`) ON UPDATE no action ON DELETE no action,
	FOREIGN KEY (`banner_image_id`) REFERENCES `cached_images`(`id`) ON UPDATE no action ON DELETE no action
);
