<script lang="ts">
	let {
		genres = [],
	 }: {
		genres?: string[],
	} = $props();
</script>

<div
	class="flex flex-wrap gap-1.5"
>
	{#each genres.slice(0, 5) as genre, i (genre)}
		<span
			class="bg-primary/20 shrink-0 rounded-sm px-2 py-0.5 text-[10px] font-medium whitespace-nowrap"
		>
			{genre}
		</span>
	{/each}
</div>
