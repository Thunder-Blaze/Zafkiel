ALTER TABLE `cached_images` ADD `file_size` integer;--> statement-breakpoint
ALTER TABLE `cached_images` ADD `cached_at` integer NOT NULL;--> statement-breakpoint
ALTER TABLE `media` DROP COLUMN `season`;--> statement-breakpoint
ALTER TABLE `media` DROP COLUMN `season_year`;--> statement-breakpoint
ALTER TABLE `media` DROP COLUMN `start_date_year`;--> statement-breakpoint
ALTER TABLE `media` DROP COLUMN `cover_color`;--> statement-breakpoint
ALTER TABLE `media` DROP COLUMN `is_favourite`;