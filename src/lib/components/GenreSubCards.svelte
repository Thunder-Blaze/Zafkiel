<script lang="ts">
	let {
		genres = [],
		compact = true,
	}: {
		genres: string[];
		compact?: boolean;
	} = $props();
</script>

<div class="flex flex-wrap gap-1.5">
	{#if compact}
		{#each genres.slice(0, 3) as genre (genre)}
			<span
				class="shrink-0 rounded-sm bg-primary/20 px-2 py-0.5 text-[10px] font-medium whitespace-nowrap"
			>
				{genre}
			</span>
		{/each}
		{#if genres.length > 3}
			<span
				class="shrink-0 rounded-sm bg-primary/20 px-2 py-0.5 text-[10px] font-medium whitespace-nowrap"
			>
				+{genres.length - 3}
			</span>
		{/if}
	{:else}
		{#each genres as genre (genre)}
			<span
				class="shrink-0 rounded-sm bg-primary/20 px-2 py-0.5 text-[10px] font-medium whitespace-nowrap"
			>
				{genre}
			</span>
		{/each}
	{/if}
</div>
