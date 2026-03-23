<script lang="ts">
	import { page } from '$app/state';
	import { useCharacterById } from '$lib/hooks/useAnilist.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';
	import { Button } from '$lib/components/ui/button';
	import { toast } from 'svelte-sonner';
	import { isAuthenticated } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';

	let { children } = $props();

	const characterId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const characterQuery = $derived(useCharacterById(characterId));

	const character = $derived(characterQuery.data?.data);
	const isLoading = $derived(characterQuery.isLoading);
	const error = $derived(characterQuery.error);

	function getTitle(name: any): string {
		if (typeof name === 'string') return name;
		if (name?.full) return name.full;
		if (name?.userPreferred) return name.userPreferred;
		if (name?.native) return name.native;
		if (name?.native) return name.native;
		return 'Unknown Name';
	}

	let isFavourite = $state(false);
	let favouritesCount = $state(0);

	$effect(() => {
		if (character) {
			isFavourite = character?.isFavourite || false;
			favouritesCount = character?.favourites || 0;
		}
	});

	let isToggling = $state(false);

	async function toggleFavourite() {
		if (!$isAuthenticated) {
			toast.error('You must be logged in to favourite a character.');
			return;
		}
		if (isToggling || !character) return;

		isToggling = true;
		try {
			const res = await invoke<any>('favourite_character', { id: character.id });
			if (res.success) {
				isFavourite = !isFavourite;
				favouritesCount += isFavourite ? 1 : -1;
				toast.success(isFavourite ? 'Added to favourites!' : 'Removed from favourites.');
			} else {
				throw new Error(res.error);
			}
		} catch (e: any) {
			toast.error(e.message || 'Failed to toggle favourite.');
		} finally {
			isToggling = false;
		}
	}

	const activeTabClass =
		'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-background text-foreground shadow-sm h-full';
	const inactiveTabClass =
		'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-background/50 text-muted-foreground hover:text-foreground h-full';
</script>

<svelte:head>
	<title>{character ? getTitle(character.name) : 'Loading...'} - Zafkiel</title>
</svelte:head>

{#if isLoading}
	<div class="w-full px-4 py-8 md:px-8 lg:px-10">
		<PageLoader type="character" />
	</div>
{:else if error || (characterQuery.data && !characterQuery.data.success)}
	<div class="w-full px-4 py-8 md:px-8 lg:px-10">
		<div class="rounded-lg border border-destructive bg-destructive/10 p-6 text-center">
			<h2 class="text-2xl font-bold text-destructive">Error Loading Character</h2>
			<p class="text-muted-foreground">
				{error?.message || characterQuery.data?.error || 'Failed to load details'}
			</p>
			<Button variant="outline" class="mt-4" onclick={() => characterQuery.refetch()}>
				Try Again
			</Button>
		</div>
	</div>
{:else if character}
	<div class="mx-auto w-full max-w-[1400px] px-4 py-8 md:px-8 lg:px-10">
		<div class="flex flex-col gap-8 md:flex-row md:items-stretch">
			<!-- Left Column: Character Image -->
			<div class="shrink-0">
				{#if character.image?.large}
					<CachedImage
						src={character.image.large}
						alt={getTitle(character.name)}
						class="mx-auto h-80 w-56 rounded-xl bg-muted object-cover shadow-2xl md:mx-0"
					/>
				{:else}
					<div
						class="mx-auto flex h-80 w-56 items-center justify-center rounded-xl bg-muted shadow-2xl md:mx-0"
					>
						<Icon icon="solar:user-bold" class="h-20 w-20 text-muted-foreground" />
					</div>
				{/if}
			</div>

			<!-- Right Column: Info & Tabs -->
			<div class="flex flex-1 flex-col justify-end pb-0">
				<div>
					<!-- Header -->
					<h1 class="mb-2 text-4xl font-bold text-pretty md:text-5xl lg:text-6xl">
						{getTitle(character.name)}
					</h1>
					{#if character.name?.native}
						<h2 class="mb-5 text-xl text-muted-foreground">{character.name.native}</h2>
					{/if}

					<!-- Quick Stats -->
					<div class="mb-6 flex flex-wrap items-center gap-3">
						<button
							class="inline-flex cursor-pointer items-center gap-2 rounded-md px-3.5 py-1.5 text-sm font-semibold shadow-sm transition-all hover:-translate-y-0.5 active:scale-95 disabled:pointer-events-none disabled:opacity-50 {isFavourite
								? 'border border-destructive/30 bg-destructive/15 text-destructive hover:bg-destructive/25 dark:border-[#E85D75]/30 dark:bg-[#E85D75]/15 dark:text-[#E85D75] dark:hover:bg-[#E85D75]/25'
								: 'border border-border/50 bg-card text-muted-foreground hover:bg-accent hover:text-accent-foreground'}"
							onclick={toggleFavourite}
							disabled={isToggling}
							aria-label={isFavourite ? 'Remove from favourites' : 'Add to favourites'}
						>
							{#if isToggling}
								<Icon icon="solar:spinner-bold" class="h-4 w-4 animate-spin" />
							{:else}
								<Icon
									icon={isFavourite ? 'solar:heart-bold' : 'solar:heart-linear'}
									class="h-4 w-4"
								/>
							{/if}

							{#if favouritesCount > 0}
								{(favouritesCount >= 1000
									? (favouritesCount / 1000).toFixed(1) + 'k'
									: favouritesCount) + ' Favorites'}
							{:else}
								Favorites
							{/if}
						</button>
						{#if character.gender}
							<div
								class="inline-flex items-center rounded-md bg-secondary px-2.5 py-1 text-xs font-semibold text-secondary-foreground shadow-sm"
							>
								{character.gender}
							</div>
						{/if}
						{#if character.age}
							<div
								class="inline-flex items-center rounded-md bg-secondary px-2.5 py-1 text-xs font-semibold text-secondary-foreground shadow-sm"
							>
								Age: {character.age}
							</div>
						{/if}
					</div>

					<!-- Tabs Navigation -->
					<div class="w-full border-b border-border/40">
						<div
							class="inline-flex h-9 w-full items-center justify-start text-muted-foreground sm:w-auto"
						>
							<a
								href={`/character/${characterId}`}
								class={'flex h-full flex-1 items-center justify-center border-b-2 px-6 text-sm font-medium transition-colors hover:text-foreground sm:flex-none ' +
									(page.url.pathname === `/character/${characterId}` ||
									page.url.pathname === `/character/${characterId}/`
										? 'border-primary text-foreground'
										: 'border-transparent text-muted-foreground')}
							>
								Overview
							</a>
							<a
								href={`/character/${characterId}/media`}
								class={'flex h-full flex-1 items-center justify-center border-b-2 px-6 text-sm font-medium transition-colors hover:text-foreground sm:flex-none ' +
									(page.url.pathname.endsWith('/media')
										? 'border-primary text-foreground'
										: 'border-transparent text-muted-foreground')}
							>
								Media
							</a>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Nested Pages rendered here, below the cover -->
		<div class="mt-8 w-full">
			{@render children()}
		</div>
	</div>
{/if}
