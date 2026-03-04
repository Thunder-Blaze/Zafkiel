import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
	timeAgo,
	formatProgress,
	scoreToColorClass,
	formatScore,
	formatMinutesWatched,
	formatCountdown,
} from './format';

describe('timeAgo', () => {
	beforeEach(() => {
		// Mock Date.now to a fixed time (seconds-based: 1000 * seconds)
		vi.spyOn(Date, 'now').mockReturnValue(1_700_000_000 * 1000);
	});
	afterEach(() => vi.restoreAllMocks());

	const NOW = 1_700_000_000;

	it('returns seconds ago for recent timestamps', () => {
		expect(timeAgo(NOW - 30)).toBe('30s ago');
	});

	it('returns minutes ago for timestamps within an hour', () => {
		expect(timeAgo(NOW - 90)).toBe('1m ago');
		expect(timeAgo(NOW - 3599)).toBe('59m ago');
	});

	it('returns hours ago for timestamps within a day', () => {
		expect(timeAgo(NOW - 7200)).toBe('2h ago');
	});

	it('returns days ago for timestamps within a week', () => {
		expect(timeAgo(NOW - 86400)).toBe('1d ago');
		expect(timeAgo(NOW - 172800)).toBe('2d ago');
	});

	it('returns weeks ago for older timestamps', () => {
		expect(timeAgo(NOW - 604800)).toBe('1w ago');
	});
});

describe('formatProgress', () => {
	it('returns em dash when progress is undefined', () => {
		expect(formatProgress()).toBe('—');
	});

	it('returns progress with unknown total', () => {
		expect(formatProgress(12)).toBe('12 / ?');
	});

	it('returns progress with total', () => {
		expect(formatProgress(12, 24)).toBe('12 / 24');
	});
});

describe('scoreToColorClass', () => {
	it('returns muted for null score', () => {
		expect(scoreToColorClass(null)).toBe('text-muted-foreground');
		expect(scoreToColorClass(undefined)).toBe('text-muted-foreground');
	});

	it('returns green for high scores', () => {
		expect(scoreToColorClass(75)).toBe('text-green-500');
		expect(scoreToColorClass(100)).toBe('text-green-500');
	});

	it('returns yellow for mid scores', () => {
		expect(scoreToColorClass(50)).toBe('text-yellow-500');
		expect(scoreToColorClass(74)).toBe('text-yellow-500');
	});

	it('returns red for low scores', () => {
		expect(scoreToColorClass(0)).toBe('text-red-500');
		expect(scoreToColorClass(49)).toBe('text-red-500');
	});
});

describe('formatScore', () => {
	it('returns N/A for null or zero scores', () => {
		expect(formatScore(null)).toBe('N/A');
		expect(formatScore(0)).toBe('N/A');
		expect(formatScore(undefined)).toBe('N/A');
	});

	it('formats score to one decimal place', () => {
		expect(formatScore(8.7)).toBe('8.7');
		expect(formatScore(10)).toBe('10.0');
		expect(formatScore(6)).toBe('6.0');
	});
});

describe('formatMinutesWatched', () => {
	it('returns 0h for no data', () => {
		expect(formatMinutesWatched()).toBe('0h');
		expect(formatMinutesWatched(0)).toBe('0h');
	});

	it('formats shorter durations as hours + minutes', () => {
		expect(formatMinutesWatched(90)).toBe('1h 30m');
	});

	it('formats long durations with days', () => {
		expect(formatMinutesWatched(1440)).toBe('1d 0h');
		expect(formatMinutesWatched(2880)).toBe('2d 0h');
	});
});

describe('formatCountdown', () => {
	it('returns Now when time is up', () => {
		expect(formatCountdown(0)).toBe('Now');
		expect(formatCountdown(-1)).toBe('Now');
	});

	it('formats minutes for short countdowns', () => {
		expect(formatCountdown(300)).toBe('5m');
	});

	it('formats hours and minutes', () => {
		expect(formatCountdown(5400)).toBe('1h 30m');
	});

	it('formats days and hours for long countdowns', () => {
		expect(formatCountdown(90000)).toBe('1d 1h');
	});
});
