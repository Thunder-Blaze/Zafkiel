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
		<h2 class="mb-4 text-xl font-semibold">Staff</h2>
		<div class="space-y-4">
			{#if animeData.staff?.edges && animeData.staff.edges.length > 0}
				<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
					{#each animeData.staff.edges as staffEdge}
						{#if staffEdge.node}
							<div
								class="rounded-lg border border-border bg-card p-4 transition-shadow hover:shadow-md"
							>
								<div class="flex gap-3">
									<img
										src={staffEdge.node.image?.large || '/api/placeholder/80/120'}
										alt={staffEdge.node.name?.full || 'Staff'}
										class="h-16 w-12 rounded object-cover"
										loading="lazy"
									/>
									<div class="min-w-0 flex-1">
										<h4 class="truncate text-sm font-semibold">
											{staffEdge.node.name?.full || 'Unknown Staff'}
										</h4>
										{#if staffEdge.node.name?.native}
											<p class="truncate text-xs text-muted-foreground">
												{staffEdge.node.name.native}
											</p>
										{/if}
										<p class="mt-1 text-xs font-medium text-primary">
											{staffEdge.role || 'Unknown Role'}
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
					<Icon icon="lucide:users" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
					<p class="text-muted-foreground">No staff information available.</p>
				</div>
			{/if}
		</div>
	</div>
{/if}
