<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { 
		ADVANCED_TAGS, 
		ANIME_STREAMING_ON,
		MANGA_READABLE_ON,
	} from '$lib/constants/search';
	import { createVirtualizer } from '@tanstack/svelte-virtual';

	let { 
		open = $bindable(false),
		type = 'ANIME',
		filtersState = $bindable({})
	} = $props<{
		open: boolean;
		type: 'ANIME' | 'MANGA';
		filtersState: Record<string, string[]>;
	}>();

	const isAnime = $derived(type === 'ANIME');

	function toggleTag(category: string, item: string) {
		if (!filtersState[category]) filtersState[category] = [];
		if (filtersState[category]!.includes(item)) {
			filtersState[category] = filtersState[category]!.filter((i: string) => i !== item);
		} else {
			filtersState[category] = [...filtersState[category]!, item];
		}
	}

	function isSelected(category: string, item: string) {
		return filtersState[category]?.includes(item);
	}

	function reset() {
		filtersState = {};
	}

	// Structural extraction of the massive maps into a linearized array of 'Groups'
	type FilterGroup = { id: string; title: string; category: string; tags: string[] };

	const filterGroups: FilterGroup[] = $derived((() => {
		const groups: FilterGroup[] = [];
		
		// 1. Add Format/Platform specific filters
		if (isAnime) {
			groups.push({ id: 'streaming', title: 'Streaming On', category: 'Streaming On', tags: ANIME_STREAMING_ON });
		} else {
			for (const [lang, platforms] of Object.entries(MANGA_READABLE_ON)) {
				groups.push({ id: `readable-${lang}`, title: `Readable On (${lang})`, category: `Readable On (${lang})`, tags: platforms });
			}
		}

		// 2. Add all Advanced Tags
		for (const [category, tags] of Object.entries(ADVANCED_TAGS)) {
			groups.push({ id: `adv-${category}`, title: category, category, tags });
		}

		return groups;
	})());

	let scrollEl = $state<HTMLDivElement>();

	const virtualizer = createVirtualizer({
		count: 0,
		getScrollElement: () => scrollEl ?? null,
		estimateSize: () => 150, // rough height of a single tag category container
		overscan: 2,
	});

	$effect(() => {
		$virtualizer.setOptions({
			count: filterGroups.length,
			getScrollElement: () => scrollEl ?? null,
			estimateSize: () => 150,
			overscan: 2,
		});
	});

	function measure(node: HTMLElement) {
		$virtualizer.measureElement(node);
		return {
			destroy() {
				$virtualizer.measureElement(node);
			},
		};
	}
</script>

<Dialog.Root bind:open={open}>
	<Dialog.Content class="max-w-4xl max-h-[90vh] overflow-hidden w-[90vw] p-0 flex flex-col gap-0 border-border/40 bg-background/95 backdrop-blur-md shadow-2xl">
		<!-- Header -->
		<div class="z-20 flex shrink-0 items-center justify-between border-b border-border/40 bg-background/95 px-6 py-4 shadow-sm backdrop-blur-sm">
			<Dialog.Title class="text-xl font-bold tracking-tight text-foreground">Advanced Filters</Dialog.Title>
			<div class="flex items-center gap-3">
				<Button variant="ghost" size="sm" onclick={reset} class="h-8 text-xs font-semibold hover:bg-muted/80">Reset All</Button>
				<Dialog.Close class="rounded-full p-1.5 text-muted-foreground outline-none ring-primary transition-colors hover:bg-muted hover:text-foreground focus-visible:ring-2">
					<Icon icon="solar:close-circle-bold" class="h-6 w-6" />
				</Dialog.Close>
			</div>
		</div>

		<!-- Virtualized Body -->
		<div 
			class="flex-1 overflow-y-auto px-6 py-4 bg-muted/10 min-h-[400px]" 
			bind:this={scrollEl}
		>
			<div style="height: {$virtualizer.getTotalSize()}px; width: 100%; position: relative;">
				{#each $virtualizer.getVirtualItems() as virtualRow (virtualRow.index)}
					{@const group = filterGroups[virtualRow.index]}
					{#if group}
						<div
							use:measure
							data-index={virtualRow.index}
							style="position: absolute; top: 0; left: 0; width: 100%; transform: translateY({virtualRow.start}px);"
						>
							<div class="mb-8 flex flex-col gap-3">
								<h3 class="flex items-center gap-2 text-sm font-bold uppercase tracking-wider text-muted-foreground">
									<Icon icon="solar:tag-bold" class="size-4 opacity-70" />
									{group.title}
								</h3>
								<div class="flex flex-wrap gap-2 pr-4">
									{#each group.tags as tag}
										<button 
											class="rounded-lg border px-3 py-1.5 text-xs font-semibold transition-all duration-200 outline-none ring-primary focus-visible:ring-2
												{isSelected(group.category, tag) 
													? 'border-primary bg-primary/10 text-primary shadow-sm' 
													: 'border-border/50 bg-card/40 text-muted-foreground hover:border-primary/50 hover:bg-card hover:text-foreground hover:shadow-sm'}"
											onclick={() => toggleTag(group.category, tag)}
										>
											{tag}
										</button>
									{/each}
								</div>
							</div>
						</div>
					{/if}
				{/each}
			</div>
		</div>
		
		<!-- Footer -->
		<div class="z-20 flex shrink-0 items-center justify-end border-t border-border/40 bg-background/95 px-6 py-4 shadow-sm backdrop-blur-sm">
			<Dialog.Close>
				<Button class="font-semibold shadow-md transition-transform hover:scale-[1.02] active:scale-95">
					<Icon icon="solar:check-read-bold" class="mr-2 size-4" />
					Apply Filters
				</Button>
			</Dialog.Close>
		</div>
	</Dialog.Content>
</Dialog.Root>
