import { relations } from "drizzle-orm/relations";
import { cachedImages, downloads, media, recentlyViewed, users } from "./schema";

export const downloadsRelations = relations(downloads, ({one}) => ({
	cachedImage: one(cachedImages, {
		fields: [downloads.coverImageId],
		references: [cachedImages.id]
	}),
	media: one(media, {
		fields: [downloads.mediaId],
		references: [media.id]
	}),
}));

export const cachedImagesRelations = relations(cachedImages, ({many}) => ({
	downloads: many(downloads),
	media_bannerImageId: many(media, {
		relationName: "media_bannerImageId_cachedImages_id"
	}),
	media_coverImageId: many(media, {
		relationName: "media_coverImageId_cachedImages_id"
	}),
	users_bannerImageId: many(users, {
		relationName: "users_bannerImageId_cachedImages_id"
	}),
	users_avatarImageId: many(users, {
		relationName: "users_avatarImageId_cachedImages_id"
	}),
}));

export const mediaRelations = relations(media, ({one, many}) => ({
	downloads: many(downloads),
	cachedImage_bannerImageId: one(cachedImages, {
		fields: [media.bannerImageId],
		references: [cachedImages.id],
		relationName: "media_bannerImageId_cachedImages_id"
	}),
	cachedImage_coverImageId: one(cachedImages, {
		fields: [media.coverImageId],
		references: [cachedImages.id],
		relationName: "media_coverImageId_cachedImages_id"
	}),
	recentlyVieweds: many(recentlyViewed),
}));

export const recentlyViewedRelations = relations(recentlyViewed, ({one}) => ({
	media: one(media, {
		fields: [recentlyViewed.mediaId],
		references: [media.id]
	}),
}));

export const usersRelations = relations(users, ({one}) => ({
	cachedImage_bannerImageId: one(cachedImages, {
		fields: [users.bannerImageId],
		references: [cachedImages.id],
		relationName: "users_bannerImageId_cachedImages_id"
	}),
	cachedImage_avatarImageId: one(cachedImages, {
		fields: [users.avatarImageId],
		references: [cachedImages.id],
		relationName: "users_avatarImageId_cachedImages_id"
	}),
}));