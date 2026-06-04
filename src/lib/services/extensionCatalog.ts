/**
 * Extension catalog – hardcoded list of available extensions.
 *
 * When we have a real backend API this file will be replaced by a fetch()
 * call.  For now the download URLs point to GitHub releases.
 */

import type { CatalogExtension } from '$lib/types/extensions';

export const EXTENSION_CATALOG: readonly CatalogExtension[] = [
	{
		id: 'animepahe',
		name: 'AnimePahe',
		latestVersion: '0.1.0',
		author: 'Zafkiel',
		description:
			'Stream anime from AnimePahe with HLS playback. ' +
			'Requires a one-time Cloudflare challenge to be solved in a browser window.',
		type: 'source',
		// Placeholder URL — replace with real GitHub release asset URL when built
		downloadUrl:
			'https://github.com/zafkiel-app/extensions/releases/download/animepahe-v0.1.0/animepahe.zext',
		iconUrl: 'https://animepahe.pw/favicon.ico',
		tags: ['streaming', 'anime', 'HLS'],
		requiresAuth: true,
		language: 'Japanese / English subtitles',
		// Dev-only: use local .zext instead of remote download
		localPath: '/home/ThunderBlaze/Documents/Projects/AiGen/Zafkiel-Uu/extensions/animepahe/animepahe.zext',
	},
] as const;
