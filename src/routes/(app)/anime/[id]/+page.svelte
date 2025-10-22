<script lang="ts">
	import { page } from '$app/state';
	import { useAnimeById } from '$lib/hooks/useAnilist.svelte';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Calendar, Clock, Star, Users, Play, Bookmark } from 'lucide-svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const animeQuery = $derived(useAnimeById(animeId));

	const anime = $derived(animeQuery.data?.data);
	const isLoading = $derived(animeQuery.isLoading);
	const error = $derived(animeQuery.error);

	function getStatusColor(status: string) {
		switch (status) {
			case 'FINISHED': return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
			case 'RELEASING': return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
			case 'NOT_YET_RELEASED': return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
			case 'CANCELLED': return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
			case 'HIATUS': return 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200';
			default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
		}
	}

	// Helper functions
	function getTitle(title: any): string {
		if (typeof title === 'string') return title;
		if (title?.romaji) return title.romaji;
		if (title?.english) return title.english;
		if (title?.native) return title.native;
		return 'Unknown Title';
	}

	function formatDate(date: any): string {
		if (!date) return 'Unknown';
		if (date.year && date.month && date.day) {
			return `${date.day}/${date.month}/${date.year}`;
		}
		if (date.year && date.month) {
			return `${date.month}/${date.year}`;
		}
		if (date.year) {
			return `${date.year}`;
		}
		return 'Unknown';
	}	function formatScore(score?: number) {
		return score ? `${score / 10}/10` : 'N/A';
	}

	function stripHtml(html?: string) {
		if (!html) return '';
		return html.replace(/<[^>]*>/g, '');
	}
</script>

<svelte:head>
	<title>{anime?.title || 'Loading...'} - Zafkiel</title>
	<meta name="description" content={anime?.description ? stripHtml(anime.description).slice(0, 160) : 'Anime details on Zafkiel'} />
</svelte:head>

<div class="container mx-auto px-4 py-8 max-w-7xl">
	{#if isLoading}
		<div class="flex items-center justify-center min-h-[400px]">
			<div class="flex flex-col items-center space-y-4">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
				<p class="text-muted-foreground">Loading anime details...</p>
			</div>
		</div>
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="text-center space-y-4">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Anime</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load anime details'}</p>
					<Button variant="outline" onclick={() => animeQuery.refetch()}>
						Try Again
					</Button>
				</div>
			</CardContent>
		</Card>
	{:else if anime}
		<!-- Hero Section -->
		<div class="relative mb-8">
			{#if anime.bannerImage}
				<div class="relative h-64 md:h-80 rounded-lg overflow-hidden">
					<CachedImage
						src={anime.bannerImage}
						alt={getTitle(anime.title)}
						class="w-full h-full object-cover"
					/>
					<div class="absolute inset-0 bg-gradient-to-t from-background via-background/50 to-transparent"></div>
				</div>
			{/if}

			<div class="flex flex-col md:flex-row gap-6 mt-6">
				<!-- Cover Image -->
				<div class="flex-shrink-0">
					{#if anime.coverImage?.large}
						<CachedImage
							src={anime.coverImage.large}
							alt={getTitle(anime.title)}
							class="w-48 h-72 object-cover rounded-lg shadow-lg mx-auto md:mx-0"
						/>
					{:else}
						<div class="w-48 h-72 bg-muted rounded-lg flex items-center justify-center mx-auto md:mx-0">
							<Play class="w-16 h-16 text-muted-foreground" />
						</div>
					{/if}
				</div>

				<!-- Main Info -->
				<div class="flex-1 space-y-4">
					<div>
											<h1 class="text-4xl md:text-5xl font-bold text-white mb-2">
						{getTitle(anime.title)}
					</h1>

						<!-- Status and Score -->
						<div class="flex flex-wrap items-center gap-3 mb-4">
							<Badge class={getStatusColor(anime.status || '')}>
								{anime.status?.replace('_', ' ')}
							</Badge>
							{#if anime.averageScore}
								<div class="flex items-center gap-1">
									<Star class="w-4 h-4 fill-yellow-400 text-yellow-400" />
									<span class="font-semibold">{formatScore(anime.averageScore)}</span>
								</div>
							{/if}
							{#if anime.popularity}
								<div class="flex items-center gap-1 text-muted-foreground">
									<Users class="w-4 h-4" />
									<span>{anime.popularity.toLocaleString()} users</span>
								</div>
							{/if}
						</div>

						<!-- Action Buttons -->
						<div class="flex flex-wrap gap-2 mb-4">
							<Button class="gap-2">
								<Play class="w-4 h-4" />
								Watch Now
							</Button>
							<Button variant="outline" class="gap-2">
								<Bookmark class="w-4 h-4" />
								Add to List
							</Button>
						</div>
					</div>

					<!-- Quick Info Grid -->
					<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
						{#if anime.format}
							<div>
								<p class="text-sm text-muted-foreground">Format</p>
								<p class="font-medium">{anime.format}</p>
							</div>
						{/if}
						{#if anime.episodes}
							<div>
								<p class="text-sm text-muted-foreground">Episodes</p>
								<p class="font-medium">{anime.episodes}</p>
							</div>
						{/if}
						{#if anime.duration}
							<div>
								<p class="text-sm text-muted-foreground">Duration</p>
								<p class="font-medium">{anime.duration} min</p>
							</div>
						{/if}
						{#if anime.season && anime.seasonYear}
							<div>
								<p class="text-sm text-muted-foreground">Season</p>
								<p class="font-medium">{anime.season} {anime.seasonYear}</p>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Main Content -->
			<div class="lg:col-span-2 space-y-6">
				<!-- Description -->
				{#if anime.description}
					<Card>
						<CardHeader>
							<CardTitle>Synopsis</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="prose prose-sm max-w-none dark:prose-invert">
								{@html anime.description}
							</div>
						</CardContent>
					</Card>
				{/if}

				<!-- Genres -->
				{#if anime.genres && anime.genres.length > 0}
					<Card>
						<CardHeader>
							<CardTitle>Genres</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="flex flex-wrap gap-2">
								{#each anime.genres as genre}
									<Badge variant="secondary">{genre}</Badge>
								{/each}
							</div>
						</CardContent>
					</Card>
				{/if}

				<!-- Studios -->
				{#if anime.studios?.nodes && anime.studios.nodes.length > 0}
					<Card>
						<CardHeader>
							<CardTitle>Studios</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-2">
								{#each anime.studios.nodes as studio}
									<a
										href="/studio/{studio.id}"
										class="block p-3 rounded-lg hover:bg-muted transition-colors"
									>
										<h4 class="font-medium">{studio.name}</h4>
										{#if studio.isAnimationStudio}
											<Badge variant="outline" class="mt-1">Animation Studio</Badge>
										{/if}
									</a>
								{/each}
							</div>
						</CardContent>
					</Card>
				{/if}
			</div>

			<!-- Sidebar -->
			<div class="space-y-6">
				<!-- Detailed Info -->
				<Card>
					<CardHeader>
						<CardTitle>Information</CardTitle>
					</CardHeader>
					<CardContent class="space-y-3">
						{#if anime.startDate}
							<div class="flex items-center gap-2">
								<Calendar class="w-4 h-4 text-muted-foreground" />
								<div>
									<p class="text-sm text-muted-foreground">Start Date</p>
									<p class="font-medium">{formatDate(anime.startDate)}</p>
								</div>
							</div>
						{/if}

						{#if anime.endDate}
							<div class="flex items-center gap-2">
								<Calendar class="w-4 h-4 text-muted-foreground" />
								<div>
									<p class="text-sm text-muted-foreground">End Date</p>
									<p class="font-medium">{formatDate(anime.endDate)}</p>
								</div>
							</div>
						{/if}

						{#if anime.source}
							<div>
								<p class="text-sm text-muted-foreground">Source</p>
								<p class="font-medium">{anime.source}</p>
							</div>
						{/if}

						{#if anime.countryOfOrigin}
							<div>
								<p class="text-sm text-muted-foreground">Country</p>
								<p class="font-medium">{anime.countryOfOrigin}</p>
							</div>
						{/if}

						{#if anime.favourites}
							<div>
								<p class="text-sm text-muted-foreground">Favorites</p>
								<p class="font-medium">{anime.favourites.toLocaleString()}</p>
							</div>
						{/if}
					</CardContent>
				</Card>

				<!-- Tags -->
				{#if anime.tags && anime.tags.length > 0}
					<Card>
						<CardHeader>
							<CardTitle>Tags</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-2">
								{#each anime.tags.slice(0, 10) as tag}
									<div class="flex items-center justify-between">
										<span class="text-sm">{tag.name}</span>
										{#if tag.rank}
											<Badge variant="outline" class="text-xs">
												{tag.rank}%
											</Badge>
										{/if}
									</div>
								{/each}
							</div>
						</CardContent>
					</Card>
				{/if}
			</div>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="text-center space-y-4">
					<h2 class="text-2xl font-bold">Anime Not Found</h2>
					<p class="text-muted-foreground">The requested anime could not be found.</p>
					<Button variant="outline" onclick={() => history.back()}>
						Go Back
					</Button>
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
