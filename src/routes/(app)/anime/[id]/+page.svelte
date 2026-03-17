<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import type { AnimeLarge } from '$lib/types/anime';
	import { fade } from 'svelte/transition';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import Icon from '@iconify/svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));
	const animeData = $derived(animeQuery.data?.data as AnimeLarge | undefined);
</script>

{#if animeData}
	<div class="space-y-6" in:fade={{ duration: 300 }}>
		<!-- Synopsis Section -->
		{#if animeData.description}
			<div class="space-y-3">
				<h2 class="text-xl font-semibold">Synopsis</h2>
				<div class="prose prose-sm max-w-none leading-relaxed text-muted-foreground">
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html animeData.description}
				</div>
			</div>
			<Separator />
		{/if}

		<!-- Information Grid -->
		<div class="space-y-4">
			<h2 class="text-xl font-semibold">Information</h2>
			<div class="rounded-lg border bg-card p-6">
				<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
					<div class="space-y-4">
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Format</span>
							<span class="font-semibold">{animeData.format || 'Unknown'}</span>
						</div>
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Season</span>
							<span class="font-semibold">
								{animeData.season && animeData.seasonYear
									? `${animeData.season.charAt(0) + animeData.season.slice(1).toLowerCase()} ${animeData.seasonYear}`
									: 'Unknown'}
							</span>
						</div>
						{#if animeData.duration}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Episode Duration</span>
								<span class="font-semibold">{animeData.duration} minutes</span>
							</div>
						{/if}
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Status</span>
							<span class="font-semibold capitalize">
								{animeData.status
									? animeData.status.toLowerCase().replace('_', ' ')
									: 'Unknown'}
							</span>
						</div>
					</div>
					<div class="space-y-4">
						<div class="flex items-center justify-between border-b border-border/50 py-2">
							<span class="font-medium text-muted-foreground">Type</span>
							<span class="font-semibold">{animeData.type || 'TV'}</span>
						</div>
						{#if animeData.studios?.nodes && animeData.studios.nodes.length > 0}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Studio</span>
								<span class="font-semibold">{animeData.studios.nodes[0].name}</span>
							</div>
						{/if}
						{#if animeData.source}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">Source</span>
								<span class="font-semibold capitalize">
									{animeData.source.toLowerCase().replace('_', ' ')}
								</span>
							</div>
						{/if}
						{#if animeData.idMal}
							<div class="flex items-center justify-between border-b border-border/50 py-2">
								<span class="font-medium text-muted-foreground">MAL ID</span>
								<a
									href="https://myanimelist.net/anime/{animeData.idMal}"
									target="_blank"
									class="flex items-center gap-1 font-semibold text-primary hover:underline"
								>
									{animeData.idMal}
									<Icon icon="lucide:external-link" class="size-3" />
								</a>
							</div>
						{/if}
						<div class="flex items-center justify-between py-2">
							<span class="font-medium text-muted-foreground">AniList ID</span>
							<a
								href="https://anilist.co/anime/{animeData.id}"
								target="_blank"
								class="flex items-center gap-1 font-semibold text-primary hover:underline"
							>
								{animeData.id}
								<Icon icon="lucide:external-link" class="size-3" />
							</a>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex flex-col items-center justify-center gap-4 py-20 text-center">
		<Icon icon="solar:widget-5-bold-duotone" class="size-16 text-muted-foreground/40" />
		<div>
			<h3 class="text-lg font-semibold text-muted-foreground">Loading...</h3>
		</div>
	</div>
{/if}
