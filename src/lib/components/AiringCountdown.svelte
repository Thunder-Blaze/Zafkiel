<script lang="ts">
	/**
	 * Live countdown that ticks every second.
	 * Displays: Xd Xh Xm Xs
	 * When timeUntilAiring ≤ 0 it shows "Airing now".
	 */
	import { onDestroy } from 'svelte';

	let { timeUntilAiring }: { timeUntilAiring: number } = $props();

	/** Seconds remaining, decremented by the interval */
	let remaining = $state(0);

	function format(secs: number): string {
		if (secs <= 0) return 'Airing now';
		const d = Math.floor(secs / 86400);
		const h = Math.floor((secs % 86400) / 3600);
		const m = Math.floor((secs % 3600) / 60);
		const s = secs % 60;
		if (d > 0) return `${d}d ${h}h ${m}m`;
		if (h > 0) return `${h}h ${m}m ${s}s`;
		return `${m}m ${s}s`;
	}

	const display = $derived(format(remaining));

	let interval: ReturnType<typeof setInterval> | null = null;

	$effect(() => {
		remaining = timeUntilAiring;
		if (interval) clearInterval(interval);
		if (timeUntilAiring > 0) {
			interval = setInterval(() => {
				remaining = Math.max(0, remaining - 1);
				if (remaining <= 0 && interval) {
					clearInterval(interval);
					interval = null;
				}
			}, 1000);
		}
		return () => {
			if (interval) clearInterval(interval);
		};
	});

	onDestroy(() => {
		if (interval) clearInterval(interval);
	});
</script>

<span class="tabular-nums">{display}</span>
