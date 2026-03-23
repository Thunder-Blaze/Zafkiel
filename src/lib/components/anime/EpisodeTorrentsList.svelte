<script lang="ts">
	import { useEpisodeTorrents } from '$lib/hooks/useEpisodeMetadata.svelte';
	import type { EpisodeMeta } from '$lib/services/EpisodeMetadataService';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';

	let { episode, onDownload } = $props<{
		episode: EpisodeMeta;
		onDownload: (magnetUri: string) => void;
	}>();

	const torrentsQuery = $derived(useEpisodeTorrents(episode, 'all'));
	const torrents = $derived(torrentsQuery.data ?? []);
	const isLoading = $derived(torrentsQuery.isLoading);
</script>

<div class="flex w-full flex-col">
	{#if isLoading}
		<div
			class="flex w-full items-center justify-center rounded-md border border-border/50 bg-muted/10 p-6 text-muted-foreground"
		>
			<Icon icon="solar:spinner-bold" class="mr-2 size-5 animate-spin" />
			<span class="text-sm font-medium">Fetching parsed releases...</span>
		</div>
	{:else if torrents.length === 0}
		<div
			class="flex flex-col items-center justify-center rounded-md border border-dashed border-border/80 bg-muted/20 p-6 text-center"
		>
			<Icon icon="solar:file-remove-bold-duotone" class="mb-2 size-8 text-muted-foreground/50" />
			<p class="text-sm font-medium text-foreground">No Parsed Releases Found</p>
			<p class="mt-1 text-xs text-muted-foreground">
				We couldn't find any matching `[Source]` formats for this episode.
			</p>
		</div>
	{:else}
		<!-- A list of matching Torrents -->
		<div
			class="custom-scrollbar grid max-h-[350px] grid-cols-1 gap-2.5 overflow-y-auto pr-1.5"
			data-lenis-prevent="true"
		>
			{#each torrents as torrent}
				<div
					class="group flex flex-col justify-between rounded-md border border-border/50 bg-background p-3.5 shadow-[0_1px_2px_rgba(0,0,0,0.03)] transition-all hover:border-primary/40 hover:bg-muted/10 sm:flex-row sm:items-center dark:shadow-none"
				>
					<!-- Data -->
					<div class="mb-3 flex min-w-0 flex-1 flex-col sm:mr-4 sm:mb-0">
						<!-- Title -->
						<span
							class="mr-2 truncate text-xs leading-tight font-semibold text-foreground/90"
							title={torrent.title}
						>
							{torrent.title}
						</span>

						<!-- Informational Tags -->
						<div class="mt-2 flex flex-wrap items-center gap-2 text-[11px]">
							{#if torrent.fansub}
								<span
									class="rounded-sm bg-primary/10 px-1.5 py-0.5 font-bold tracking-wide text-primary"
								>
									{torrent.fansub}
								</span>
							{/if}

							{#if torrent.resolution}
								<span
									class="rounded-sm bg-secondary/80 px-1.5 py-0.5 font-semibold text-secondary-foreground"
								>
									{torrent.resolution}
								</span>
							{/if}

							{#if torrent.languages && torrent.languages.length > 0}
								{#each torrent.languages as lang}
									<span
										class="box-border rounded-sm bg-amber-500/10 px-1.5 py-0.5 font-bold text-amber-600 dark:text-amber-400"
									>
										{lang}
									</span>
								{/each}
							{/if}

							<div class="relative mx-0.5 h-3 w-[1.5px] rounded-full bg-border"></div>

							<!-- Storage -->
							<span
								class="flex items-center gap-1 font-medium text-muted-foreground"
								title="File Size"
							>
								<Icon icon="solar:folder-bold" class="size-3" />
								{torrent.size}
							</span>

							<div class="relative mx-0.5 h-3 w-[1.5px] rounded-full bg-border"></div>

							<!-- Analytics -->
							<span
								class="flex items-center gap-1 font-bold text-emerald-600 dark:text-emerald-500"
								title="Seeders"
							>
								<Icon icon="solar:arrow-up-bold" class="size-3" />
								{torrent.seeds}
							</span>

							<span
								class="flex items-center gap-1 font-semibold text-rose-600 dark:text-rose-500"
								title="Leechers"
							>
								<Icon icon="solar:arrow-down-bold" class="size-3" />
								{torrent.peers}
							</span>

							{#if torrent.downloads > 0}
								<span
									class="flex items-center gap-1 font-semibold text-sky-600 dark:text-sky-500"
									title="Downloads"
								>
									<Icon icon="solar:download-bold" class="size-3" />
									{torrent.downloads}
								</span>
							{/if}
						</div>
					</div>

					<!-- Direct Downloads & Copy -->
					<div class="mt-3 flex w-full shrink-0 items-center gap-2 sm:mt-0 sm:w-auto">
						<Button
							size="icon"
							variant="outline"
							class="size-8 shrink-0 text-muted-foreground hover:text-foreground"
							title="Copy Magnet Link"
							aria-label="Copy Magnet Link"
							onclick={(e) => {
								e.stopPropagation();
								navigator.clipboard.writeText(torrent.magnetUri);
								toast.success('Magnet link copied!');
							}}
						>
							<Icon icon="solar:copy-outline" class="size-4" />
						</Button>

						<Button
							size="sm"
							variant="secondary"
							class="w-full text-xs font-bold shadow-sm transition-transform active:scale-95 sm:w-auto"
							onclick={(e) => {
								e.stopPropagation();
								onDownload(torrent.magnetUri);
							}}
						>
							<Icon icon="solar:download-square-linear" class="mr-1.5 size-3.5" />
							Download
						</Button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background-color: hsl(var(--muted));
		border-radius: 10px;
	}
	.custom-scrollbar:hover::-webkit-scrollbar-thumb {
		background-color: hsl(var(--primary) / 0.5);
	}
</style>
