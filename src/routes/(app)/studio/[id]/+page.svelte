<script lang="ts">
	import { page } from '$app/state';
	import { useStudioById } from '$lib/hooks/useAnilist.svelte';
	import {
		Card,
		CardContent,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

	const studioId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const studioQuery = $derived(useStudioById(studioId));

	const studio = $derived(studioQuery.data?.data);
	const isLoading = $derived(studioQuery.isLoading);
	const error = $derived(studioQuery.error);
</script>

<svelte:head>
	<title>{studio?.name || 'Loading...'} - Zafkiel</title>
	<meta name="description" content="Studio details on Zafkiel" />
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-8">
	{#if isLoading}
		<div class="flex min-h-[400px] items-center justify-center">
			<div class="flex flex-col items-center space-y-4">
				<Icon icon="solar:refresh-circle-line-duotone" class="h-12 w-12 animate-spin text-primary" />
				<p class="text-muted-foreground">Loading studio details...</p>
			</div>
		</div>
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Studio</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load studio details'}</p>
					<Button variant="outline" onclick={() => studioQuery.refetch()}>Try Again</Button>
				</div>
			</CardContent>
		</Card>
	{:else if studio}
		<!-- Hero Section -->
		<div class="mb-8 flex flex-col items-center justify-center space-y-4 text-center">
			<div class="flex h-24 w-24 items-center justify-center rounded-full bg-muted">
				<Icon icon="solar:clapperboard-edit-bold-duotone" class="h-12 w-12 text-primary" />
			</div>
			<h1 class="text-4xl font-bold">{studio.name}</h1>
			<div class="flex gap-2">
				{#if studio.isAnimationStudio}
					<Badge variant="secondary">Animation Studio</Badge>
				{/if}
				{#if studio.favourites}
					<Badge variant="outline" class="gap-1">
						<Icon icon="solar:heart-bold" class="h-3 w-3 text-red-500" />
						{studio.favourites.toLocaleString()} Favorites
					</Badge>
				{/if}
			</div>
		</div>

		<!-- Content Tabs -->
		<Tabs value="anime" class="w-full">
			<TabsList class="w-full justify-center bg-transparent p-0">
				<TabsTrigger
					value="anime"
					class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
				>
					Anime
				</TabsTrigger>
				<TabsTrigger
					value="info"
					class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
				>
					Information
				</TabsTrigger>
			</TabsList>

			<TabsContent value="anime" class="mt-6">
				{#if studio.media?.nodes && studio.media.nodes.length > 0}
					<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
						{#each studio.media.nodes as media}
							<a href="/anime/{media.id}" class="group relative block overflow-hidden rounded-lg bg-card transition-all hover:scale-105 hover:shadow-lg">
								<div class="aspect-[2/3] w-full overflow-hidden">
									{#if media.coverImage?.large}
										<CachedImage
											src={media.coverImage.large}
											alt={media.title?.userPreferred || media.title?.english || 'Anime Cover'}
											class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-110"
										/>
									{:else}
										<div class="flex h-full w-full items-center justify-center bg-muted">
											<Icon icon="solar:gallery-wide-bold-duotone" class="h-12 w-12 text-muted-foreground" />
										</div>
									{/if}
									<div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100">
										<div class="absolute bottom-0 left-0 right-0 p-4">
											<p class="line-clamp-2 text-sm font-semibold text-white">
												{media.title?.userPreferred || media.title?.english}
											</p>
											{#if media.averageScore}
												<div class="mt-1 flex items-center gap-1 text-xs text-yellow-400">
													<Icon icon="solar:star-bold" class="h-3 w-3" />
													<span>{media.averageScore}%</span>
												</div>
											{/if}
										</div>
									</div>
								</div>
							</a>
						{/each}
					</div>
				{:else}
					<div class="flex min-h-[200px] flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center">
						<Icon icon="solar:clapperboard-text-bold-duotone" class="mb-4 h-12 w-12 text-muted-foreground" />
						<h3 class="text-lg font-semibold">No Anime Found</h3>
						<p class="text-muted-foreground">This studio hasn't produced any anime yet or data is missing.</p>
					</div>
				{/if}
			</TabsContent>

			<TabsContent value="info" class="mt-6">
				<Card>
					<CardHeader>
						<CardTitle>Studio Information</CardTitle>
					</CardHeader>
					<CardContent class="space-y-4">
						<div class="flex justify-between border-b py-2">
							<span class="text-muted-foreground">Name</span>
							<span class="font-medium">{studio.name}</span>
						</div>
						{#if studio.siteUrl}
							<div class="flex justify-between border-b py-2">
								<span class="text-muted-foreground">AniList Page</span>
								<a href={studio.siteUrl} target="_blank" class="font-medium text-primary hover:underline">
									View on AniList
								</a>
							</div>
						{/if}
						<div class="flex justify-between border-b py-2">
							<span class="text-muted-foreground">Animation Studio</span>
							<span class="font-medium">{studio.isAnimationStudio ? 'Yes' : 'No'}</span>
						</div>
					</CardContent>
				</Card>
			</TabsContent>
		</Tabs>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">Studio Not Found</h2>
					<p class="text-muted-foreground">The requested studio could not be found.</p>
					<Button variant="outline" onclick={() => history.back()}>Go Back</Button>
				</div>
			</CardContent>
		</Card>
	{/if}
</div>
