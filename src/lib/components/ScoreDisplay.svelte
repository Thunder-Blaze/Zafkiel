<script lang="ts">
	/**
	 * Reusable score display component.
	 * Supports: badge (number), ring (circle), and stars variants.
	 * Score is on AniList's 0–100 scale internally; displayed as 0–10 (1 dp).
	 */
	import { scoreToColorClass } from '$lib/utils/format';

	type Variant = 'badge' | 'ring' | 'stars';

	let {
		score,
		variant = 'badge',
		size = 'md',
		class: className = '',
	}: {
		score?: number | null;
		variant?: Variant;
		size?: 'sm' | 'md' | 'lg';
		class?: string;
	} = $props();

	const colorClass = $derived(scoreToColorClass(score));

	// Convert 0–100 → 0–10 with 1dp
	const displayScore = $derived(
		score == null || score === 0 ? null : (score / 10).toFixed(1),
	);

	const sizeClasses: Record<string, Record<string, string>> = {
		badge: {
			sm: 'text-[10px] px-1 py-0 rounded',
			md: 'text-xs px-1.5 py-0.5 rounded-md',
			lg: 'text-sm px-2 py-1 rounded-lg',
		},
		ring: {
			sm: 'size-8 text-[10px]',
			md: 'size-11 text-xs',
			lg: 'size-14 text-sm',
		},
		stars: {
			sm: 'text-xs gap-0.5',
			md: 'text-sm gap-1',
			lg: 'text-base gap-1',
		},
	};

	// Stars: convert 0–100 scale → 0–5 stars (in 0.5 steps)
	const starCount = $derived(
		score == null || score === 0 ? 0 : Math.round((score / 100) * 5 * 2) / 2,
	);
</script>

{#if variant === 'badge'}
	<span
		class="inline-flex items-center font-semibold tabular-nums {colorClass} {sizeClasses.badge[size]} {className}"
	>
		{displayScore ?? 'N/A'}
	</span>
{:else if variant === 'ring'}
	<div
		class="relative inline-flex shrink-0 items-center justify-center rounded-full border-2 font-bold tabular-nums
			{colorClass}
			{colorClass.includes('green') ? 'border-green-500/30 bg-green-500/10' : ''}
			{colorClass.includes('yellow') ? 'border-yellow-500/30 bg-yellow-500/10' : ''}
			{colorClass.includes('red') ? 'border-red-500/30 bg-red-500/10' : ''}
			{colorClass.includes('muted') ? 'border-muted bg-muted/20' : ''}
			{sizeClasses.ring[size]} {className}"
	>
		{displayScore ?? '—'}
	</div>
{:else if variant === 'stars'}
	<div class="inline-flex items-center {sizeClasses.stars[size]} {className}">
		{#each Array(5) as _, i}
			{@const filled = i + 1 <= Math.floor(starCount)}
			{@const half = !filled && i + 0.5 <= starCount}
			<span
				class={filled
					? 'text-yellow-400'
					: half
						? 'text-yellow-400/60'
						: 'text-muted-foreground/30'}
			>
				★
			</span>
		{/each}
		{#if displayScore}
			<span class="ml-1 text-muted-foreground">{displayScore}</span>
		{/if}
	</div>
{/if}
