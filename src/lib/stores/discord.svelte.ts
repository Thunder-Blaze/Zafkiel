import { invoke } from '@tauri-apps/api/core';

export interface DiscordActivity {
	state: string;
	details: string;
	largeImage?: string | null;
	largeText?: string | null;
	smallImage?: string | null;
	smallText?: string | null;
	startTimestamp?: number | null;
	endTimestamp?: number | null;
}

class DiscordStore {
	private currentActivity: DiscordActivity | null = null;
	private enabled = true; // In the future, this could be tied to a user setting

	/**
	 * Sets the Discord activity.
	 * NOTE: The Rust commands expect snake_case arguments from Tauri if not using #[serde(rename_all = "camelCase")].
	 * By default, Svelte invokes pass exactly the JSON we provide. We must map keys correctly to match Rust fn arguments.
	 */
	private lastUpdate = 0;
	private updateTimeout: ReturnType<typeof setTimeout> | null = null;
    private pendingActivity: DiscordActivity | null = null;

	async setActivity(activity: DiscordActivity) {
		if (!this.enabled) return;
        
        const now = Date.now();
        // Discord rate limit is roughly 1 update per 15 seconds.
        // We'll allow updates every 5 seconds to be safe but responsive to seekers.
        const THROTTLE_MS = 5000; 

        if (now - this.lastUpdate < THROTTLE_MS) {
            this.pendingActivity = activity;
            if (!this.updateTimeout) {
                this.updateTimeout = setTimeout(() => {
                    this.updateTimeout = null;
                    if (this.pendingActivity) {
                        this.setActivity(this.pendingActivity);
                        this.pendingActivity = null;
                    }
                }, THROTTLE_MS - (now - this.lastUpdate));
            }
            return;
        }

		this.currentActivity = activity;
        this.lastUpdate = now;
        if (this.updateTimeout) {
            clearTimeout(this.updateTimeout);
            this.updateTimeout = null;
        }

		try {
			await invoke('set_discord_activity', { payload: activity });
		} catch (e) {
			console.warn('[Discord RPC] failed to set activity:', e);
		}
	}

	async clearActivity() {
		if (!this.enabled) return;
		this.currentActivity = null;
		try {
			await invoke('clear_discord_activity');
		} catch (e) {
			console.warn('[Discord RPC] failed to clear activity:', e);
		}
	}
}

export const discordStore = new DiscordStore();
