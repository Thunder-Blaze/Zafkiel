import { invoke } from '@tauri-apps/api/core';
import type { TorrentProvider, TorrentInfo, ExtensionManifest } from './ExtensionManager';
import type { AnimeLarge } from '$lib/types/anime';

export class NyaaProvider implements TorrentProvider {
	manifest: ExtensionManifest = {
		id: 'nyaa-si',
		name: 'Nyaa.si',
		version: '1.1.0',
		author: 'System',
		type: 'torrent',
		description: 'Nyaa.si RSS torrent search provider',
	};

	private rssUrl = 'https://nyaa.si';

	async search(query: string): Promise<TorrentInfo[]> {
		const url = `${this.rssUrl}/?page=rss&q=${encodeURIComponent(query)}&c=0_0&f=0`;
		try {
			const xml = await invoke<string>('fetch_url', {
				url,
				headers: { 'Referer': 'https://nyaa.si/' }
			});
			return this.parseRSS(xml);
		} catch (e) {
			console.error(`[Nyaa] Search failed for "${query}":`, e);
			return [];
		}
	}

	async searchAnime(anime: AnimeLarge): Promise<TorrentInfo[]> {
		const title = anime.title?.english || anime.title?.romaji || '';
		if (!title) return [];
		return this.search(title);
	}

	async searchEpisode(
		anime: AnimeLarge,
		episodeNumber: number,
		anidbId?: number,
		anidbEpisodeId?: number,
		quality: string = 'all',
		absoluteEpisodeNumber?: number
	): Promise<TorrentInfo[]> {
		const qualityStr = quality === 'all' ? '' : ` ${quality}`;
		const epStr = episodeNumber < 10 ? `0${episodeNumber}` : `${episodeNumber}`;
		const absEpStr = absoluteEpisodeNumber ? (absoluteEpisodeNumber < 10 ? `0${absoluteEpisodeNumber}` : `${absoluteEpisodeNumber}`) : null;

		const animeTitle = anime.title?.english || anime.title?.romaji || '';
		if (!animeTitle) return [];

		// ── Clean base title ──
		let baseTitle = animeTitle.replace(/【|】|［|］|\[|\]/g, ' ').replace(/[:’']/g, ' ').replace(/\s+/g, ' ').trim();

		const seasonMatch = baseTitle.match(/Season\s*(\d+)/i);
		const seasonNum = seasonMatch ? parseInt(seasonMatch[1], 10) : 1;
		const sStr = seasonNum < 10 ? `0${seasonNum}` : `${seasonNum}`;
		const titleNoSeason = baseTitle.replace(/\s*Season\s*\d+/i, '').trim();

		const queryVariants = [
			`${titleNoSeason} Season ${seasonNum} - ${epStr}${qualityStr}`,
			`${titleNoSeason} S${seasonNum} - ${epStr}${qualityStr}`,
			`${baseTitle} - ${epStr}${qualityStr}`,
			`${titleNoSeason} S${sStr}E${epStr}${qualityStr}`,
			`${baseTitle} ${epStr}${qualityStr}`
		];

		// Include absolute numbering variant if available (very common on Nyaa for sequels)
		if (absEpStr) {
			queryVariants.push(`${titleNoSeason} - ${absEpStr}${qualityStr}`);
			queryVariants.push(`${titleNoSeason} ${absEpStr}${qualityStr}`);
		}

		// Only include the bare "Title - Ep" query if we're on Season 1
		if (seasonNum === 1) {
			queryVariants.push(`${titleNoSeason} - ${epStr}${qualityStr}`);
		}

		// De-duplicate queries to avoid hitting Nyaa with identical requests (prevents 10054 Connection Reset)
		const queries = Array.from(new Set(queryVariants.map(q => q.trim()).filter(Boolean)));

		const allResults = new Map<string, TorrentInfo>();

		for (const q of queries) {
			const results = await this.search(q);
			const filtered = results.filter(r => {
				const t = r.title.toLowerCase();
				// 1. Exclude obvious batches
				if (t.includes('batch') || t.includes('complete') || t.includes('collection')) {
					return false;
				}
				
				// 2. Clear known noise that mimics episode numbers (e.g. 10-bit, 8-bit)
				const cleanTitle = t.replace(/\d+(?:-?bit|bits)/g, '');

				// 3. Strict episode number check
				// Accepts relative or absolute numbering
				const relativeRegex = new RegExp(`(?:[\\s\\[-]|^)${epStr}(?:[\\s\\](v-]|$)|(?:[\\s\\[-]|^)${episodeNumber}(?:[\\s\\](v-]|$)|[Ee]${epStr}`);
				const absoluteRegex = absEpStr ? new RegExp(`(?:[\\s\\[-]|^)${absEpStr}(?:[\\s\\](v-]|$)|(?:[\\s\\[-]|^)${absoluteEpisodeNumber}(?:[\\s\\](v-]|$)|[Ee]${absEpStr}`) : null;
				
				const matchesRelative = relativeRegex.test(cleanTitle);
				const matchesAbsolute = absoluteRegex ? absoluteRegex.test(cleanTitle) : false;
				
				if (!matchesRelative && !matchesAbsolute) return false;

				// 4. Season consistency check
				if (seasonNum > 1 && matchesRelative && !matchesAbsolute) {
					// If it strictly matches the relative episode (e.g. "01"), it MUST match the season if a season is mentioned
					for (let i = 1; i <= 10; i++) {
						if (i === seasonNum) continue;
						
						// Reject if it mentions Season X without mentioning current season
						if (t.includes(`season ${i}`) && !t.includes(`season ${seasonNum}`)) return false;
						
						const sMatch = t.match(/[Ss](\d+)/);
						if (sMatch && parseInt(sMatch[1], 10) !== seasonNum) {
							// Check if it's a multi-season tag [S01-S02]
							if (!t.includes(`s${seasonNum}`) && !t.includes(`s0${seasonNum}`)) return false;
						}
					}
				}

				return true;
			});
			
			for (const r of filtered) {
				if (!allResults.has(r.magnet)) {
					allResults.set(r.magnet, r);
				}
			}

			// Stagger requests to avoid Nyaa rate limiting (Connection Reset)
			await new Promise(resolve => setTimeout(resolve, 1500));
		}

		return Array.from(allResults.values());
	}

	private parseRSS(xml: string): TorrentInfo[] {
		const parser = new DOMParser();
		const doc = parser.parseFromString(xml, 'text/xml');
		const items = doc.querySelectorAll('item');
		const results: TorrentInfo[] = [];

		const getTagText = (el: Element, tagName: string) => {
			const tag = Array.from(el.children).find((c) => 
				c.nodeName.toLowerCase() === tagName.toLowerCase() || 
				c.nodeName.toLowerCase().endsWith(':' + tagName.toLowerCase())
			);
			return tag?.textContent ?? '';
		};

		items.forEach((item) => {
			const title = item.querySelector('title')?.textContent ?? '';
			const pubDate = item.querySelector('pubDate')?.textContent ?? '';
			const seeds = parseInt(getTagText(item, 'seeders') || '0', 10);
			const peers = parseInt(getTagText(item, 'leechers') || '0', 10);
			const size = getTagText(item, 'size') || '0 B';
			const infoHash = getTagText(item, 'infoHash');
			const magnet = infoHash ? `magnet:?xt=urn:btih:${infoHash}&dn=${encodeURIComponent(title)}` : '';

			// ── Subgroup (Fansub) Parsing ──
			let fansub: string | undefined = undefined;
			
			// 1. Try leading brackets: [Subgroup]
			const subgroupMatch = title.match(/^\[(.*?)\]/);
			if (subgroupMatch) {
				fansub = subgroupMatch[1];
			} else {
				// 2. Try trailing tags: ... -Subgroup-Raws (CR)
				const trailingMatch = title.match(/-(.*?)\s*\(CR\)/) || title.match(/-(.*?)\s*\(DSNP\)/) || title.match(/-(.*?)$/);
				if (trailingMatch) {
					const pot = trailingMatch[1].trim();
					if (pot.length > 2 && pot.length < 25 && !pot.includes(' ')) {
						fansub = pot;
					}
				}
			}

			results.push({
				title,
				magnet,
				size,
				seeds,
				peers,
				provider: 'Nyaa.si',
				fansub,
				uploadedAt: pubDate ? new Date(pubDate).toLocaleDateString() : '',
			});
		});
		return results;
	}
}
