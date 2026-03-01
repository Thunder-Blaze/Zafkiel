/**
 * Shared formatting utilities for AniList data display
 */

/**
 * Returns a human-readable relative time string for a Unix timestamp (seconds).
 */
export function timeAgo(ts: number): string {
	const diff = Math.floor(Date.now() / 1000) - ts;
	if (diff < 60) return `${diff}s ago`;
	if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
	if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
	if (diff < 604800) return `${Math.floor(diff / 86400)}d ago`;
	return `${Math.floor(diff / 604800)}w ago`;
}

/**
 * Formats an episode/chapter progress string, e.g. "12 / 24" or "12 / ?"
 */
export function formatProgress(progress?: number, total?: number): string {
	if (progress == null) return '—';
	if (total == null) return `${progress} / ?`;
	return `${progress} / ${total}`;
}

/**
 * Returns a Tailwind color class for a score (0–100 scale).
 */
export function scoreToColorClass(score?: number | null): string {
	if (score == null) return 'text-muted-foreground';
	if (score >= 75) return 'text-green-500';
	if (score >= 50) return 'text-yellow-500';
	return 'text-red-500';
}

/**
 * Formats an AniList score on a 0–10 scale to 1 decimal place,
 * or returns "N/A" if the score is null / 0.
 */
export function formatScore(score?: number | null): string {
	if (score == null || score === 0) return 'N/A';
	return score.toFixed(1);
}

/**
 * Formats minutes watched into a human-readable duration.
 */
export function formatMinutesWatched(minutes?: number): string {
	if (!minutes) return '0h';
	const days = Math.floor(minutes / 1440);
	const hours = Math.floor((minutes % 1440) / 60);
	if (days > 0) return `${days}d ${hours}h`;
	return `${hours}h ${minutes % 60}m`;
}

/**
 * Formats a countdown (seconds until airing) to a compact string.
 */
export function formatCountdown(seconds: number): string {
	if (seconds <= 0) return 'Now';
	const days = Math.floor(seconds / 86400);
	const hours = Math.floor((seconds % 86400) / 3600);
	const mins = Math.floor((seconds % 3600) / 60);
	if (days > 0) return `${days}d ${hours}h`;
	if (hours > 0) return `${hours}h ${mins}m`;
	return `${mins}m`;
}
