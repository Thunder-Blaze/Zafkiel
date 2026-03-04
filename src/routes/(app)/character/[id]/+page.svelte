<script lang="ts">
	import { page } from '$app/state';
	import { useCharacterById } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

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
		return 'Unknown Name';
	}

	function stripHtml(html?: string) {
		if (!html) return '';
		return html.replace(/<[^>]*>/g, '');
	}
</script>

<svelte:head>
	<title>{character ? getTitle(character.name) : 'Loading...'} - Zafkiel</title>
	<meta
		name="description"
		content={character?.description
			? stripHtml(character.description).slice(0, 160)
			: 'Character details on Zafkiel'}
	/>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-8">
	{#if isLoading}
		<div class="flex min-h-[400px] items-center justify-center">
			<div class="flex flex-col items-center space-y-4">
				<Icon
					icon="solar:refresh-circle-line-duotone"
					class="h-12 w-12 animate-spin text-primary"
				/>
				<p class="text-muted-foreground">Loading character details...</p>
			</div>
		</div>
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Character</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load character details'}</p>
					<Button variant="outline" onclick={() => characterQuery.refetch()}>Try Again</Button>
				</div>
			</CardContent>
		</Card>
	{:else if character}
		<!-- Hero Section -->
		<div class="relative mb-8">
			<div class="flex flex-col gap-8 md:flex-row">
				<!-- Image -->
				<div class="shrink-0">
					{#if character.image?.large}
						<CachedImage
							src={character.image.large}
							alt={getTitle(character.name)}
							class="mx-auto h-80 w-56 rounded-xl object-cover shadow-2xl md:mx-0"
						/>
					{:else}
						<div
							class="mx-auto flex h-80 w-56 items-center justify-center rounded-xl bg-muted md:mx-0"
						>
							<Icon icon="solar:user-bold" class="h-20 w-20 text-muted-foreground" />
						</div>
					{/if}
				</div>

				<!-- Main Info -->
				<div class="flex flex-1 flex-col justify-end space-y-4 pb-4">
					<div>
						<h1 class="mb-2 text-4xl font-bold md:text-5xl lg:text-6xl">
							{getTitle(character.name)}
						</h1>
						{#if character.name?.native}
							<h2 class="text-xl text-muted-foreground">{character.name.native}</h2>
						{/if}

						<!-- Quick Info -->
						<div class="mt-6 flex flex-wrap items-center gap-3">
							{#if character.favourites}
								<Badge variant="outline" class="gap-1">
									<Icon icon="solar:heart-bold" class="h-3 w-3 text-red-500" />
									{character.favourites.toLocaleString()} Favorites
								</Badge>
							{/if}
							{#if character.gender}
								<Badge variant="secondary">
									{character.gender}
								</Badge>
							{/if}
							{#if character.age}
								<Badge variant="secondary">
									Age: {character.age}
								</Badge>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Content Tabs -->
		<div class="mt-8">
			<Tabs value="overview" class="w-full">
				<TabsList
					class="w-full justify-start overflow-x-auto rounded-none border-b bg-transparent p-0"
				>
					<TabsTrigger
						value="overview"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Overview
					</TabsTrigger>
					<TabsTrigger
						value="media"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Media
					</TabsTrigger>
				</TabsList>

				<TabsContent value="overview" class="mt-6">
					<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
						<!-- Main Content -->
						<div class="space-y-8 lg:col-span-2">
							<!-- Description -->
							{#if character.description}
								<div class="space-y-4">
									<h3 class="text-lg font-semibold">Description</h3>
									<div class="prose prose-sm dark:prose-invert max-w-none text-muted-foreground">
										{@html character.description}
									</div>
								</div>
							{/if}
						</div>

						<!-- Sidebar -->
						<div class="space-y-8">
							<!-- Details -->
							<Card>
								<CardHeader>
									<CardTitle>Information</CardTitle>
								</CardHeader>
								<CardContent class="space-y-4">
									{#if character.dateOfBirth?.year || character.dateOfBirth?.month || character.dateOfBirth?.day}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Birthday</span>
											<span class="font-medium">
												{character.dateOfBirth.day}/{character.dateOfBirth.month}
												{#if character.dateOfBirth.year}/{character.dateOfBirth.year}{/if}
											</span>
										</div>
									{/if}
									{#if character.bloodType}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Blood Type</span>
											<span class="font-medium">{character.bloodType}</span>
										</div>
									{/if}
									{#if character.siteUrl}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">AniList Profile</span>
											<a
												href={character.siteUrl}
												target="_blank"
												rel="noopener noreferrer"
												class="font-medium text-primary hover:underline"
											>
												View on AniList
											</a>
										</div>
									{/if}
								</CardContent>
							</Card>

							<!-- Alternative Names -->
							{#if character.name?.alternative && character.name.alternative.length > 0}
								<Card>
									<CardHeader>
										<CardTitle>Alternative Names</CardTitle>
									</CardHeader>
									<CardContent>
										<div class="flex flex-wrap gap-2">
											{#each character.name.alternative as name}
												<Badge variant="outline">{name}</Badge>
											{/each}
										</div>
									</CardContent>
								</Card>
							{/if}
						</div>
					</div>
				</TabsContent>

				<TabsContent value="media" class="mt-6">
					{#if character.media?.edges && character.media.edges.length > 0}
						<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
							{#each character.media.edges as edge}
								{#if edge.node}
									<a
										href="/{edge.node.type?.toLowerCase()}/{edge.node.id}"
										class="group relative block overflow-hidden rounded-lg bg-card transition-all hover:scale-105 hover:shadow-lg"
									>
										<div class="aspect-[2/3] w-full overflow-hidden">
											{#if edge.node.coverImage?.large}
												<CachedImage
													src={edge.node.coverImage.large}
													alt={edge.node.title?.userPreferred ||
														edge.node.title?.english ||
														'Media Cover'}
													class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-110"
												/>
											{:else}
												<div class="flex h-full w-full items-center justify-center bg-muted">
													<Icon
														icon="solar:gallery-wide-bold-duotone"
														class="h-12 w-12 text-muted-foreground"
													/>
												</div>
											{/if}
											<div
												class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
											>
												<div class="absolute right-0 bottom-0 left-0 p-4">
													<p class="line-clamp-2 text-sm font-semibold text-white">
														{edge.node.title?.userPreferred || edge.node.title?.english}
													</p>
													{#if edge.characterRole}
														<Badge variant="secondary" class="mt-1 text-[10px]">
															{edge.characterRole}
														</Badge>
													{/if}
												</div>
											</div>
										</div>
									</a>
								{/if}
							{/each}
						</div>
					{:else}
						<div
							class="flex min-h-[200px] flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center"
						>
							<Icon
								icon="solar:clapperboard-text-bold-duotone"
								class="mb-4 h-12 w-12 text-muted-foreground"
							/>
							<h3 class="text-lg font-semibold">No Media Found</h3>
							<p class="text-muted-foreground">
								This character doesn't appear in any media or data is missing.
							</p>
						</div>
					{/if}
				</TabsContent>
			</Tabs>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">Character Not Found</h2>
					<p class="text-muted-foreground">The requested character could not be found.</p>
					<Button variant="outline" onclick={() => history.back()}>Go Back</Button>
				</div>
			</CardContent>
		</Card>
	{/if}
</div>

<style>
	:global(.prose p) {
		margin-bottom: 1rem;
	}

	:global(.prose br) {
		margin-bottom: 0.5rem;
	}
</style>
