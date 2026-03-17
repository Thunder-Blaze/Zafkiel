<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import Icon from '@iconify/svelte';
	import type { AnimeLarge } from '$lib/types/anime';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
</script>

{#if animeData}
	<div class="mt-2">
		<h2 class="mb-4 text-xl font-semibold">Related</h2>
		<div class="space-y-6">
			{#if animeData.relations?.edges && animeData.relations.edges.length > 0}
				<div class="space-y-4">
					{#each animeData.relations.edges as relation}
						{#if relation.node}
							<div
								class="rounded-lg border border-border bg-card p-4 transition-shadow hover:shadow-md"
							>
								<div class="flex gap-4">
									<img
										src={relation.node.coverImage?.medium || '/api/placeholder/80/120'}
										alt={relation.node.title?.userPreferred || 'Related Media'}
										class="h-20 w-16 rounded object-cover"
										loading="lazy"
									/>
									<div class="min-w-0 flex-1">
										<div class="mb-1 flex items-center gap-2">
											<span
												class="rounded-full bg-primary/20 px-2 py-1 text-xs font-medium text-primary"
											>
												{relation.relationType?.replace('_', ' ') || 'Related'}
											</span>
											<span class="text-xs text-muted-foreground">
												{relation.node.type} • {relation.node.format}
											</span>
										</div>
										<h4 class="mb-1 line-clamp-2 text-sm font-semibold">
											{relation.node.title?.userPreferred || 'Unknown Title'}
										</h4>
										<p class="text-xs text-muted-foreground">
											{relation.node.status} •
											{#if relation.node.episodes}
												{relation.node.episodes} episodes
											{:else if relation.node.chapters}
												{relation.node.chapters} chapters
											{:else}
												Unknown length
											{/if}
										</p>
									</div>
								</div>
							</div>
						{/if}
					{/each}
				</div>
			{:else}
				<div
					class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
				>
					<Icon icon="lucide:list-plus" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
					<p class="text-muted-foreground">No related anime found.</p>
				</div>
			{/if}
		</div>
	</div>
{/if}
