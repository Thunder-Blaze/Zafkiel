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
		<div class="flex flex-col gap-6 md:flex-row md:items-start lg:gap-8">
			<!-- Left Column: Character Image Tracker -->
			<div class="mx-auto w-[215px] shrink-0 md:mx-0">
				{#if character.image?.large}
					<CachedImage
						src={character.image.large}
						alt={getTitle(character.name)}
						class="w-full rounded bg-muted object-cover shadow-sm"
					/>
				{:else}
					<div
						class="flex aspect-[2/3] w-full items-center justify-center rounded bg-muted shadow-sm"
					>
						<Icon icon="solar:user-bold" class="h-16 w-16 text-muted-foreground" />
					</div>
				{/if}
			</div>

			<!-- Right Column: Content -->
			<div class="flex min-w-0 flex-1 flex-col pt-1">
				<!-- Header -->
				<h1 class="mb-1 text-3xl font-bold leading-tight text-foreground md:text-4xl text-pretty">
					{getTitle(character.name)}
				</h1>
				{#if character.name?.native}
					<h2 class="mb-5 text-base text-muted-foreground">{character.name.native}</h2>
				{/if}

				<!-- Quick Stats -->
				<div class="mb-6 flex flex-wrap items-center gap-2.5">
					<button
						class="flex items-center gap-1.5 rounded-sm px-2.5 py-1 text-xs font-semibold transition-all hover:opacity-80 disabled:opacity-50 disabled:cursor-not-allowed {isFavourite ? 'bg-destructive/15 text-destructive dark:bg-[#E85D75]/10 dark:text-[#E85D75]' : 'bg-muted/60 text-muted-foreground hover:bg-muted/80 hover:text-foreground'}"
						onclick={toggleFavourite}
						disabled={isToggling}
						aria-label={isFavourite ? "Remove from favourites" : "Add to favourites"}
					>
						<!-- Loading spinner or heart icon based on interacting state -->
						{#if isToggling}
							<Icon icon="solar:spinner-bold" class="h-3.5 w-3.5 animate-spin" />
						{:else}
							<Icon icon={isFavourite ? "solar:heart-bold" : "solar:heart-linear"} class="h-3.5 w-3.5" />
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
						<div class="rounded-sm bg-muted/60 px-2.5 py-1 text-xs font-medium text-foreground">
							{character.gender}
						</div>
					{/if}
					{#if character.age}
						<div class="rounded-sm bg-muted/60 px-2.5 py-1 text-xs font-medium text-foreground">
							Age: {character.age}
						</div>
					{/if}
				</div>

				<!-- Tabs Navigation -->
				<div class="mb-8 w-full border-b border-border/40">
					<div class="inline-flex h-9 w-full sm:w-auto items-center justify-start text-muted-foreground">
						<a
							href={`/character/${characterId}`}
							class={"flex-1 sm:flex-none border-b-2 px-6 h-full font-medium transition-colors hover:text-foreground text-sm flex items-center justify-center " +
								(page.url.pathname === `/character/${characterId}` || page.url.pathname === `/character/${characterId}/`
									? 'border-primary text-foreground'
									: 'border-transparent text-muted-foreground')}
						>
							Overview
						</a>
						<a
							href={`/character/${characterId}/media`}
							class={"flex-1 sm:flex-none border-b-2 px-6 h-full font-medium transition-colors hover:text-foreground text-sm flex items-center justify-center " +
								(page.url.pathname.endsWith('/media')
									? 'border-primary text-foreground'
									: 'border-transparent text-muted-foreground')}
						>
							Media
						</a>
					</div>
				</div>

				<!-- Nested Pages rendered here -->
				<div class="w-full">
					{@render children()}
				</div>
			</div>
		</div>
	</div>
{/if}
