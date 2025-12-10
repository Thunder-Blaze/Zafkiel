import type { TorrentProvider, ExtensionManifest, TorrentInfo } from '$lib/services/ExtensionManager';
import type { AnimeLarge } from '$lib/types/anime';
import { filterTitle } from '$lib/utils/data-filters';
import { invoke } from '@tauri-apps/api/core';

export const SubsPleaseExtension: TorrentProvider = {
    manifest: {
        id: 'nyaa',
        name: 'Nyaa',
        version: '1.0.0',
        author: 'Zafkiel',
        description: 'Fetches torrents from Nyaa.si (including SubsPlease releases)',
        type: 'torrent'
    },

    async search(query: string): Promise<TorrentInfo[]> {
        try {
            // Nyaa RSS URL
            const url = `https://nyaa.si/?page=rss&q=${encodeURIComponent(query)}&c=0_0&f=0`;

            // Use backend fetch to bypass CORS
            const xmlText = await invoke<string>('fetch_url', { url });

            // Parse XML
            const parser = new DOMParser();
            const xmlDoc = parser.parseFromString(xmlText, "text/xml");
            const items = xmlDoc.querySelectorAll("item");

            const results: TorrentInfo[] = [];

            items.forEach(item => {
                const title = item.querySelector("title")?.textContent || "Unknown Title";
                const link = item.querySelector("link")?.textContent || "";
                const pubDate = item.querySelector("pubDate")?.textContent || "";

                const seeds = parseInt(item.getElementsByTagName("nyaa:seeders")[0]?.textContent || "0");
                const peers = parseInt(item.getElementsByTagName("nyaa:leechers")[0]?.textContent || "0");
                const size = item.getElementsByTagName("nyaa:size")[0]?.textContent || "Unknown";

                const infoHash = item.getElementsByTagName("nyaa:infoHash")[0]?.textContent;
                let magnet = "";

                if (infoHash) {
                    magnet = `magnet:?xt=urn:btih:${infoHash}&dn=${encodeURIComponent(title)}`;
                } else {
                    magnet = link;
                }

                results.push({
                    title,
                    size,
                    seeds,
                    peers,
                    magnet,
                    provider: 'Nyaa',
                    uploadedAt: new Date(pubDate).toLocaleDateString()
                });
            });

            return results;
        } catch (error) {
            console.error('Nyaa search error:', error);
            return [];
        }
    },

    async searchAnime(anime: AnimeLarge): Promise<TorrentInfo[]> {
        const title = filterTitle(anime.title || {});
        // Try searching with English title first, then Romaji
        let results = await this.search(title);

        // If no results, try Romaji
        if (results.length === 0 && anime.title?.romaji && anime.title.romaji !== title) {
            results = await this.search(anime.title.romaji);
        }

        return results;
    }
};
