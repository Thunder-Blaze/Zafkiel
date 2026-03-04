import { invoke } from '@tauri-apps/api/core';

export interface Torrent {
	id: number;
	name?: string;
	progress: number;
	speed: number;
	upload_speed: number;
	peers: number;
	seeds: number;
	state: string;
	total_size: number;
	downloaded: number;
}

export interface TorrentFile {
	id: number;
	name: string;
	size: number;
	progress: number;
}

export class TorrentService {
	static async getTorrents(): Promise<Torrent[]> {
		try {
			return await invoke<Torrent[]>('get_torrents');
		} catch (e) {
			console.error('Failed to get torrents:', e);
			return [];
		}
	}

	static async getTorrentFiles(magnet: string): Promise<TorrentFile[]> {
		try {
			return await invoke<TorrentFile[]>('get_torrent_files', { magnet });
		} catch (e) {
			console.error('Failed to get torrent files:', e);
			return [];
		}
	}

	static async getTorrentFilesById(id: number): Promise<TorrentFile[]> {
		try {
			return await invoke<TorrentFile[]>('get_torrent_files_by_id', { id });
		} catch (e) {
			console.error('Failed to get torrent files by ID:', e);
			return [];
		}
	}

	static async addTorrent(magnet: string): Promise<void> {
		// Just triggering stream implicitly adds it, but we can also have a dedicated add if needed.
		// For now, streaming handles it.
		console.log('addTorrent called, but currently handled by streamTorrent implicitly.');
	}

	static async streamTorrent(magnet: string, fileId?: number): Promise<string> {
		try {
			const url = await invoke<string>('stream_torrent', { magnet, fileId });
			return url;
		} catch (e) {
			console.error('Failed to start stream via Rust backend:', e);
			throw e;
		}
	}

	static async streamTorrentById(id: number, fileId?: number): Promise<string> {
		try {
			const url = await invoke<string>('stream_torrent_by_id', { id, fileId });
			return url;
		} catch (e) {
			console.error('Failed to start stream by ID:', e);
			throw e;
		}
	}

	static async pauseTorrent(id: number): Promise<void> {
		await invoke('pause_torrent', { id });
	}

	static async resumeTorrent(id: number): Promise<void> {
		await invoke('resume_torrent', { id });
	}

	static async deleteTorrent(id: number, deleteFiles: boolean = false): Promise<void> {
		await invoke('delete_torrent', { id, deleteFiles });
	}

	static async openInExternalPlayer(url: string): Promise<void> {
		await invoke('open_in_external_player', { url });
	}

	static async getStreamBaseUrl(): Promise<string> {
		try {
			return await invoke<string>('get_stream_base_url');
		} catch (e) {
			console.error('Failed to get stream base URL:', e);
			return '';
		}
	}
}
