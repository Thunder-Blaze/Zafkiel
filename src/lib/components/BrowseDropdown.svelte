<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { useConfigState } from '$lib/stores/config.svelte';

	let open = $state(false);
	let triggerEl = $state<HTMLButtonElement | null>(null);
	let menuEl = $state<HTMLDivElement | null>(null);

	const blurEffectsEnabled = $derived(useConfigState().blurEffects);

	// Position of the dropdown panel
	let pos = $state({ top: 0, left: 0 });

	// Portal action — moves element to document.body so backdrop-filter works
	// outside of TitleBar's compositing stacking context
	function portal(node: HTMLElement) {
		document.body.appendChild(node);
		return {
			destroy() {
				node.parentNode?.removeChild(node);
			},
		};
	}

	function updatePosition() {
		if (!triggerEl) return;
		const rect = triggerEl.getBoundingClientRect();
		pos = { top: rect.bottom + 8, left: rect.left };
	}

	function toggle() {
		if (!open) updatePosition();
		open = !open;
	}

	function close() {
		open = false;
	}

	function navigate(path: string) {
		close();
		goto(path);
	}

	// Close on outside click or Escape
	$effect(() => {
		if (!open || !browser) return;

		const handleClick = (e: MouseEvent) => {
			if (
				menuEl &&
				!menuEl.contains(e.target as Node) &&
				triggerEl &&
				!triggerEl.contains(e.target as Node)
			) {
				close();
			}
		};
		const handleKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') close();
		};

		document.addEventListener('mousedown', handleClick);
		document.addEventListener('keydown', handleKey);
		return () => {
			document.removeEventListener('mousedown', handleClick);
			document.removeEventListener('keydown', handleKey);
		};
	});
</script>

<!-- Trigger -->
<button
	bind:this={triggerEl}
	type="button"
	onclick={toggle}
	class="flex h-8 items-center gap-1.5 rounded-md px-2.5 text-xs font-medium transition-colors
		{open
		? 'bg-primary/10 text-primary'
		: 'text-foreground/70 hover:bg-foreground/5 hover:text-foreground'}"
>
	<Icon icon="solar:compass-bold-duotone" class="h-4 w-4" />
	Browse
	<Icon
		icon="solar:alt-arrow-down-linear"
		class="h-3 w-3 transition-transform {open ? 'rotate-180' : ''}"
	/>
</button>

<!-- Dropdown panel — portaled to document.body so backdrop-filter works outside TitleBar stacking context -->
{#if open && browser}
	<div
		use:portal
		bind:this={menuEl}
		class="fixed z-[999999]"
		style="top: {pos.top}px; left: {pos.left}px;"
		transition:scale={{ duration: 150, easing: quintOut, start: 0.95, opacity: 0 }}
	>
		<div
			class="w-72 rounded-xl border border-border/40 p-3.5 shadow-xl
				{blurEffectsEnabled ? 'bg-popover/85 backdrop-blur-xl' : 'bg-popover'}"
		>
			<!-- Anime -->
			<div class="mb-2.5 flex items-center gap-3 rounded-lg bg-foreground/5 px-3 py-2.5">
				<Icon icon="solar:play-circle-bold-duotone" class="h-5 w-5 shrink-0 text-primary" />
				<div class="flex flex-col">
					<button
						onclick={() => navigate('/search/anime')}
						class="mb-1 text-left text-sm font-semibold text-foreground transition-colors hover:text-primary"
					>
						Anime
					</button>
					<div class="flex items-center text-xs whitespace-nowrap text-muted-foreground">
						<button
							onclick={() => navigate('/browse/anime?sort=POPULARITY_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Top 100</button
						>
						<span class="mx-2 opacity-30">·</span>
						<button
							onclick={() => navigate('/browse/anime?sort=TRENDING_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Trending</button
						>
						<span class="mx-2 opacity-30">·</span>
						<button
							onclick={() => navigate('/browse/anime?format=MOVIE&sort=SCORE_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Top Movies</button
						>
					</div>
				</div>
			</div>

			<!-- Manga -->
			<div class="mb-1.5 flex items-center gap-3 rounded-lg bg-foreground/5 px-3 py-2.5">
				<Icon icon="solar:book-2-bold-duotone" class="h-5 w-5 shrink-0 text-primary" />
				<div class="flex flex-col">
					<button
						onclick={() => navigate('/search/manga')}
						class="mb-1 text-left text-sm font-semibold text-foreground transition-colors hover:text-primary"
					>
						Manga
					</button>
					<div class="flex items-center text-xs whitespace-nowrap text-muted-foreground">
						<button
							onclick={() => navigate('/browse/manga?sort=POPULARITY_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Top 100</button
						>
						<span class="mx-2 opacity-30">·</span>
						<button
							onclick={() => navigate('/browse/manga?sort=TRENDING_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Trending</button
						>
						<span class="mx-2 opacity-30">·</span>
						<button
							onclick={() => navigate('/browse/manga?country=KR&sort=POPULARITY_DESC')}
							class="py-0.5 transition-colors hover:text-foreground">Top Manhwa</button
						>
					</div>
				</div>
			</div>

			<!-- Other -->
			<div class="grid grid-cols-[auto_auto] justify-between gap-x-2">
				<button
					onclick={() => navigate('/search/staff')}
					class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					<Icon icon="solar:users-group-two-rounded-bold-duotone" class="h-3.5 w-3.5" />
					Staff
				</button>
				<button
					onclick={() => navigate('/search/characters')}
					class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					<Icon icon="solar:user-circle-bold-duotone" class="h-3.5 w-3.5" />
					Characters
				</button>
				<button
					onclick={() => navigate('/search/reviews')}
					class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					<Icon icon="solar:star-bold-duotone" class="h-3.5 w-3.5" />
					Reviews
				</button>
				<button
					onclick={() => navigate('/search/recommendations')}
					class="flex items-center gap-2 px-1 py-1.5 text-xs text-muted-foreground transition-colors hover:text-foreground"
				>
					<Icon icon="solar:like-bold-duotone" class="h-3.5 w-3.5" />
					Recommendations
				</button>
			</div>
		</div>
	</div>
{/if}
