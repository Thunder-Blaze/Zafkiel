<script lang="ts">
	import { scale } from "svelte/transition";

	let {
		genres = [],
		animationsEnabled = false
	 }: {
		genres?: string[],
		animationsEnabled?: boolean
	} = $props();

	let animate = $state(false);

	$effect(() => {
		animate = true;
	});
</script>

<div
	class="flex flex-wrap gap-1.5"
>
	{#each genres.slice(0, 5) as genre, i (genre)}
		{#if animate}
		<span
			class="bg-primary/20 shrink-0 rounded-sm px-2 py-0.5 text-[10px] font-medium whitespace-nowrap"
			in:scale={{
				duration: animationsEnabled ? 250 : 0,
				delay: animationsEnabled ? 150 + i * 50 : 0,
				start: 0.7
			}}
		>
			{genre}
		</span>
		{/if}
	{/each}
</div>
