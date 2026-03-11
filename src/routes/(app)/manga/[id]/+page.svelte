<script lang="ts">
	import { page } from '$app/state';
	import { useMangaById } from '$lib/hooks/useAnilist.svelte';
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
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const mangaId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const mangaQuery = $derived(useMangaById(mangaId));

	const manga = $derived(mangaQuery.data?.data);
	const isLoading = $derived(mangaQuery.isLoading);
	const error = $derived(mangaQuery.error);

	function getStatusColor(status: string) {
		switch (status) {
			case 'FINISHED':
				return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
			case 'RELEASING':
				return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200';
			case 'NOT_YET_RELEASED':
				return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
			case 'CANCELLED':
				return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
			case 'HIATUS':
				return 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200';
			default:
				return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200';
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
	}
	function formatScore(score?: number) {
		return score ? `${score / 10}/10` : 'N/A';
	}

	function stripHtml(html?: string) {
		if (!html) return '';
		return html.replace(/<[^>]*>/g, '');
	}
</script>

<svelte:head>
	<title>{manga?.title || 'Loading...'} - Zafkiel</title>
	<meta
		name="description"
		content={manga?.description
			? stripHtml(manga.description).slice(0, 160)
			: 'Manga details on Zafkiel'}
	/>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-8">
	{#if isLoading}
		<PageLoader type="manga" />
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Manga</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load manga details'}</p>
					<Button variant="outline" onclick={() => mangaQuery.refetch()}>Try Again</Button>
				</div>
			</CardContent>
		</Card>
	{:else if manga}
		<!-- Hero Section -->
		<div class="relative mb-8">
			{#if manga.bannerImage}
				<div class="relative h-64 overflow-hidden rounded-xl md:h-96">
					<CachedImage
						src={manga.bannerImage}
						alt={getTitle(manga.title)}
						class="h-full w-full object-cover"
					/>
					<div
						class="absolute inset-0 bg-gradient-to-t from-background via-background/60 to-transparent"
					></div>
				</div>
			{/if}

			<div class="relative z-10 -mt-32 px-4 md:px-8">
				<div class="flex flex-col gap-8 md:flex-row">
					<!-- Cover Image -->
					<div class="shrink-0">
						{#if manga.coverImage?.large}
							<CachedImage
								src={manga.coverImage.large}
								alt={getTitle(manga.title)}
								class="mx-auto h-80 w-56 rounded-xl object-cover shadow-2xl md:mx-0"
							/>
						{:else}
							<div
								class="mx-auto flex h-80 w-56 items-center justify-center rounded-xl bg-muted md:mx-0"
							>
								<Icon icon="solar:book-2-bold" class="h-16 w-16 text-muted-foreground" />
							</div>
						{/if}
					</div>

					<!-- Main Info -->
					<div class="flex flex-1 flex-col justify-end space-y-4 pb-4">
						<div>
							<h1 class="mb-2 text-4xl font-bold text-white md:text-5xl lg:text-6xl">
								{getTitle(manga.title)}
							</h1>

							<!-- Status and Score -->
							<div class="mb-6 flex flex-wrap items-center gap-3">
								<Badge class={getStatusColor(manga.status || '')}>
									{manga.status?.replace('_', ' ')}
								</Badge>
								{#if manga.averageScore}
									<div
										class="flex items-center gap-1 rounded-full bg-background/50 px-3 py-1 backdrop-blur-sm"
									>
										<Icon icon="solar:star-bold" class="h-4 w-4 text-yellow-400" />
										<span class="font-semibold">{formatScore(manga.averageScore)}</span>
									</div>
								{/if}
								{#if manga.popularity}
									<div
										class="flex items-center gap-1 rounded-full bg-background/50 px-3 py-1 text-muted-foreground backdrop-blur-sm"
									>
										<Icon icon="solar:users-group-rounded-bold" class="h-4 w-4" />
										<span>{manga.popularity.toLocaleString()} users</span>
									</div>
								{/if}
							</div>

							<!-- Action Buttons -->
							<div class="flex flex-wrap gap-3">
								<Button size="lg" class="gap-2 rounded-full px-8">
									<Icon icon="solar:book-2-bold" class="h-5 w-5" />
									Read Now
								</Button>
								<Button variant="secondary" size="lg" class="gap-2 rounded-full">
									<Icon icon="solar:bookmark-bold" class="h-5 w-5" />
									Add to List
								</Button>
							</div>
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
						value="characters"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Characters
					</TabsTrigger>
					<TabsTrigger
						value="staff"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Staff
					</TabsTrigger>
				</TabsList>

				<TabsContent value="overview" class="mt-6">
					<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
						<!-- Main Content -->
						<div class="space-y-8 lg:col-span-2">
							<!-- Description -->
							{#if manga.description}
								<div class="space-y-4">
									<h3 class="text-lg font-semibold">Synopsis</h3>
									<div class="prose prose-sm max-w-none text-muted-foreground dark:prose-invert">
										{@html manga.description}
									</div>
								</div>
							{/if}

							<!-- Genres -->
							{#if manga.genres && manga.genres.length > 0}
								<div class="space-y-4">
									<h3 class="text-lg font-semibold">Genres</h3>
									<div class="flex flex-wrap gap-2">
										{#each manga.genres as genre}
											<Badge variant="secondary" class="px-3 py-1 text-sm">{genre}</Badge>
										{/each}
									</div>
								</div>
							{/if}
						</div>

						<!-- Sidebar -->
						<div class="space-y-8">
							<!-- Detailed Info -->
							<Card>
								<CardHeader>
									<CardTitle>Information</CardTitle>
								</CardHeader>
								<CardContent class="space-y-4">
									{#if manga.format}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Format</span>
											<span class="font-medium">{manga.format}</span>
										</div>
									{/if}
									{#if manga.chapters}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Chapters</span>
											<span class="font-medium">{manga.chapters}</span>
										</div>
									{/if}
									{#if manga.volumes}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Volumes</span>
											<span class="font-medium">{manga.volumes}</span>
										</div>
									{/if}
									{#if manga.status}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Status</span>
											<span class="font-medium capitalize"
												>{manga.status.replace(/_/g, ' ').toLowerCase()}</span
											>
										</div>
									{/if}
									{#if manga.startDate}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Start Date</span>
											<span class="font-medium">{formatDate(manga.startDate)}</span>
										</div>
									{/if}
									{#if manga.source}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Source</span>
											<span class="font-medium capitalize"
												>{manga.source.replace(/_/g, ' ').toLowerCase()}</span
											>
										</div>
									{/if}
								</CardContent>
							</Card>

							<!-- Tags -->
							{#if manga.tags && manga.tags.length > 0}
								<Card>
									<CardHeader>
										<CardTitle>Tags</CardTitle>
									</CardHeader>
									<CardContent>
										<div class="flex flex-wrap gap-2">
											{#each manga.tags.slice(0, 15) as tag}
												<Badge variant="outline" class="cursor-help" title={tag.description}>
													{tag.name}
													{#if tag.rank}
														<span class="ml-1 text-[10px] text-muted-foreground">{tag.rank}%</span>
													{/if}
												</Badge>
											{/each}
										</div>
									</CardContent>
								</Card>
							{/if}
						</div>
					</div>
				</TabsContent>

				<TabsContent value="characters" class="mt-6">
					<div
						class="flex min-h-[200px] flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center"
					>
						<Icon
							icon="solar:users-group-rounded-bold-duotone"
							class="mb-4 h-12 w-12 text-muted-foreground"
						/>
						<h3 class="text-lg font-semibold">Characters</h3>
						<p class="text-muted-foreground">Character list coming soon.</p>
					</div>
				</TabsContent>

				<TabsContent value="staff" class="mt-6">
					<div
						class="flex min-h-[200px] flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center"
					>
						<Icon icon="solar:user-id-bold-duotone" class="mb-4 h-12 w-12 text-muted-foreground" />
						<h3 class="text-lg font-semibold">Staff</h3>
						<p class="text-muted-foreground">Staff list coming soon.</p>
					</div>
				</TabsContent>
			</Tabs>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">Manga Not Found</h2>
					<p class="text-muted-foreground">The requested manga could not be found.</p>
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
