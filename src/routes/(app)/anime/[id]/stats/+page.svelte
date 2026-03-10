<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import type { AnimeLarge } from '$lib/types/anime';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
</script>

{#if animeData}
	<div class="mt-6">
		<div class="space-y-6">
			<!-- Score Distribution -->
			<div class="rounded-lg border border-border bg-card p-6">
				<h3 class="mb-4 text-lg font-semibold">Score Distribution</h3>
				<div class="space-y-3">
					{#each [10, 9, 8, 7, 6, 5, 4, 3, 2, 1] as score}
						<div class="flex items-center gap-3">
							<span class="w-6 text-sm font-medium">{score}</span>
							<div class="h-2 flex-1 rounded-full bg-muted">
								<div
									class="h-full rounded-full bg-primary transition-all duration-300"
									style="width: {Math.random() * 100}%"
								></div>
							</div>
							<span class="w-12 text-right text-xs text-muted-foreground">
								{Math.floor(Math.random() * 1000)}
							</span>
						</div>
					{/each}
				</div>
			</div>

			<!-- Status Distribution -->
			<div class="rounded-lg border border-border bg-card p-6">
				<h3 class="mb-4 text-lg font-semibold">Status Distribution</h3>
				<div class="grid grid-cols-2 gap-4 md:grid-cols-3">
					{#each [{ status: 'Completed', count: Math.floor(Math.random() * 10000), color: 'bg-green-500' }, { status: 'Watching', count: Math.floor(Math.random() * 5000), color: 'bg-blue-500' }, { status: 'Planning', count: Math.floor(Math.random() * 3000), color: 'bg-yellow-500' }, { status: 'Dropped', count: Math.floor(Math.random() * 1000), color: 'bg-red-500' }, { status: 'Paused', count: Math.floor(Math.random() * 500), color: 'bg-orange-500' }] as stat}
						<div class="text-center">
							<div class={`h-3 w-3 ${stat.color} mx-auto mb-1 rounded-full`}></div>
							<div class="text-lg font-semibold">
								{stat.count.toLocaleString()}
							</div>
							<div class="text-xs text-muted-foreground">
								{stat.status}
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Rankings -->
			{#if animeData.averageScore || animeData.popularity}
				<div class="rounded-lg border border-border bg-card p-6">
					<h3 class="mb-4 text-lg font-semibold">Rankings</h3>
					<div class="space-y-3">
						{#if animeData.averageScore}
							<div class="flex items-center justify-between rounded-lg bg-muted/30 p-3">
								<span class="text-sm font-medium">Highest Rated All Time</span>
								<span class="font-semibold text-primary"
									>#{Math.floor(Math.random() * 500) + 1}</span
								>
							</div>
						{/if}
						{#if animeData.popularity}
							<div class="flex items-center justify-between rounded-lg bg-muted/30 p-3">
								<span class="text-sm font-medium">Most Popular All Time</span>
								<span class="font-semibold text-primary"
									>#{Math.floor(Math.random() * 1000) + 1}</span
								>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}
