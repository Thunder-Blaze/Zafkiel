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

<div class="flex flex-col w-full">
	{#if isLoading}
		<div class="flex items-center justify-center p-6 text-muted-foreground w-full bg-muted/10 rounded-md border border-border/50">
			<Icon icon="solar:spinner-bold" class="size-5 animate-spin mr-2" />
			<span class="text-sm font-medium">Fetching parsed releases...</span>
		</div>
	{:else if torrents.length === 0}
		<div class="flex flex-col items-center justify-center text-center p-6 rounded-md border border-dashed border-border/80 bg-muted/20">
			<Icon icon="solar:file-remove-bold-duotone" class="size-8 text-muted-foreground/50 mb-2" />
			<p class="text-sm font-medium text-foreground">No Parsed Releases Found</p>
			<p class="text-xs text-muted-foreground mt-1">
				We couldn't find any matching `[Source]` formats for this episode.
			</p>
		</div>
	{:else}
		<!-- A list of matching Torrents -->
		<div class="grid grid-cols-1 gap-2.5 max-h-[350px] overflow-y-auto pr-1.5 custom-scrollbar" data-lenis-prevent="true">
			{#each torrents as torrent}
				<div class="group flex flex-col sm:flex-row sm:items-center justify-between p-3.5 rounded-md bg-background border border-border/50 hover:bg-muted/10 hover:border-primary/40 transition-all shadow-[0_1px_2px_rgba(0,0,0,0.03)] dark:shadow-none">
					<!-- Data -->
					<div class="flex flex-col min-w-0 mb-3 sm:mb-0 sm:mr-4 flex-1">
						<!-- Title -->
						<span class="font-semibold text-xs leading-tight text-foreground/90 truncate mr-2" title={torrent.title}>
							{torrent.title}
						</span>

						<!-- Informational Tags -->
						<div class="flex items-center flex-wrap gap-2 mt-2 text-[11px]">
							{#if torrent.fansub}
								<span class="font-bold text-primary px-1.5 py-0.5 rounded-sm bg-primary/10 tracking-wide">
									{torrent.fansub}
								</span>
							{/if}
							
							{#if torrent.resolution}
								<span class="font-semibold text-secondary-foreground bg-secondary/80 px-1.5 py-0.5 rounded-sm">
									{torrent.resolution}
								</span>
							{/if}

							{#if torrent.languages && torrent.languages.length > 0}
								{#each torrent.languages as lang}
									<span class="font-bold text-amber-600 dark:text-amber-400 bg-amber-500/10 px-1.5 py-0.5 rounded-sm box-border">
										{lang}
									</span>
								{/each}
							{/if}

							<div class="h-3 w-[1.5px] bg-border mx-0.5 relative rounded-full"></div>
							
							<!-- Storage -->
							<span class="font-medium text-muted-foreground flex items-center gap-1" title="File Size">
								<Icon icon="solar:folder-bold" class="size-3" />
								{torrent.size}
							</span>

							<div class="h-3 w-[1.5px] bg-border mx-0.5 relative rounded-full"></div>
							
							<!-- Analytics -->
							<span class="flex items-center gap-1 font-bold text-emerald-600 dark:text-emerald-500" title="Seeders">
								<Icon icon="solar:arrow-up-bold" class="size-3" />
								{torrent.seeds}
							</span>
							
							<span class="flex items-center gap-1 font-semibold text-rose-600 dark:text-rose-500" title="Leechers">
								<Icon icon="solar:arrow-down-bold" class="size-3" />
								{torrent.peers}
							</span>

							{#if torrent.downloads > 0}
								<span class="flex items-center gap-1 font-semibold text-sky-600 dark:text-sky-500" title="Downloads">
									<Icon icon="solar:download-bold" class="size-3" />
									{torrent.downloads}
								</span>
							{/if}
						</div>
					</div>

					<!-- Direct Downloads & Copy -->
					<div class="flex items-center gap-2 w-full sm:w-auto mt-3 sm:mt-0 shrink-0">
						<Button
							size="icon"
							variant="outline"
							class="size-8 text-muted-foreground hover:text-foreground shrink-0"
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
							class="w-full sm:w-auto text-xs font-bold shadow-sm transition-transform active:scale-95" 
							onclick={(e) => {
								e.stopPropagation();
								onDownload(torrent.magnetUri);
							}}
						>
							<Icon icon="solar:download-square-linear" class="size-3.5 mr-1.5" />
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
