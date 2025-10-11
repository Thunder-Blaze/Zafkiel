import { DatabaseService } from '$lib/services/database';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	try {
		const cachedImages = await DatabaseService.getAllCachedImages();
		return {
			cachedImages
		};
	} catch (error) {
		console.error('Failed to load cached images:', error);
		return {
			cachedImages: []
		};
	}
};
