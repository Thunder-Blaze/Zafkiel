<script lang="ts">
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import CharacterCard, { type Character } from './CharacterCard.svelte';
	import { createVirtualizer } from '@tanstack/svelte-virtual';

	let {
		characters = [],
		isLoading = false,
		isFetchingNextPage = false,
		hasNextPage = false,
		onFetchNextPage = () => {},
	} = $props<{
		characters: Character[];
		isLoading: boolean;
		isFetchingNextPage?: boolean;
		hasNextPage?: boolean;
		onFetchNextPage?: () => void;
	}>();

	// Group characters into rows of 2
	const rows = $derived.by(() => {
		const res = [];
		for (let i = 0; i < characters.length; i += 2) {
			res.push(characters.slice(i, i + 2));
		}
		return res;
	});

	let scrollContainer = $state<HTMLDivElement>();

	const virtualizer = createVirtualizer({
		count: 0,
		getScrollElement: () => scrollContainer ?? null,
		estimateSize: () => 100, // height of a row
		overscan: 5,
	});

	$effect(() => {
		$virtualizer.setOptions({
			count: rows.length,
			getScrollElement: () => scrollContainer ?? null,
			estimateSize: () => 100,
			overscan: 5,
		});
	});

	// Trigger fetch next page
	$effect(() => {
		const virtualItems = $virtualizer.getVirtualItems();
		if (virtualItems.length > 0) {
			const lastItem = virtualItems[virtualItems.length - 1];
			if (lastItem.index >= rows.length - 1 && hasNextPage && !isFetchingNextPage && !isLoading) {
				onFetchNextPage();
			}
		}
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

{#if isLoading && characters.length === 0}
	<div class="flex items-center justify-center p-8">
		<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
	</div>
{:else if characters.length > 0}
	<div
		bind:this={scrollContainer}
		class="custom-scrollbar h-[70vh] w-full overflow-y-auto pr-2"
		data-lenis-prevent="true"
	>
		<div style="height: {$virtualizer.getTotalSize()}px; width: 100%; position: relative;">
			{#each $virtualizer.getVirtualItems() as virtualRow (virtualRow.index)}
				<div
					use:measure
					data-index={virtualRow.index}
					style="position: absolute; top: 0; left: 0; width: 100%; transform: translateY({virtualRow.start}px);"
				>
					<div class="grid grid-cols-1 gap-4 pb-4 md:grid-cols-2 lg:grid-cols-2">
						{#each rows[virtualRow.index] as character (character.id)}
							<CharacterCard {character} />
						{/each}
					</div>
				</div>
			{/each}
		</div>

		{#if isFetchingNextPage}
			<div class="flex items-center justify-center py-6">
				<Icon icon="solar:refresh-circle-line-duotone" class="h-6 w-6 animate-spin text-primary" />
			</div>
		{/if}
	</div>
{:else}
	<div
		class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
	>
		<Icon icon="lucide:users" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
		<p class="text-muted-foreground">No characters found.</p>
	</div>
{/if}

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: var(--border);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: var(--primary);
	}
</style>
