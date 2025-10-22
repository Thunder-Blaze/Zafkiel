<script lang="ts">
	import { page } from '$app/state';
	import { useUserById, useUserByName } from '$lib/hooks/useAnilist.svelte';
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
	import { Calendar, Clock, Star, Users, Play, BookOpen, Trophy, MapPin } from 'lucide-svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';

	const userId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const userQuery = $derived(useUserById(userId));

	const user = $derived(userQuery.data?.data);
	const isLoading = $derived(userQuery.isLoading);
	const error = $derived(userQuery.error);

	function formatDate(timestamp?: number) {
		if (!timestamp) return 'Unknown';
		return new Date(timestamp * 1000).toLocaleDateString();
	}

	function formatMinutes(minutes?: number) {
		if (!minutes) return '0 hours';
		const hours = Math.floor(minutes / 60);
		const remainingMinutes = minutes % 60;

		if (hours === 0) return `${remainingMinutes} minutes`;
		if (remainingMinutes === 0) return `${hours} hours`;
		return `${hours}h ${remainingMinutes}m`;
	}

	function stripHtml(html?: string) {
		if (!html) return '';
		return html.replace(/<[^>]*>/g, '');
	}
</script>

<svelte:head>
	<title>{user?.name || 'Loading...'} - Zafkiel</title>
	<meta name="description" content={user?.about ? stripHtml(user.about).slice(0, 160) : 'User profile on Zafkiel'} />
</svelte:head>

<div class="container mx-auto px-4 py-8 max-w-7xl">
	{#if isLoading}
		<div class="flex items-center justify-center min-h-[400px]">
			<div class="flex flex-col items-center space-y-4">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
				<p class="text-muted-foreground">Loading user profile...</p>
			</div>
		</div>
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="text-center space-y-4">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Profile</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load user profile'}</p>
					<Button variant="outline" onclick={() => userQuery.refetch()}>
						Try Again
					</Button>
				</div>
			</CardContent>
		</Card>
	{:else if user}
		<!-- Hero Section -->
		<div class="relative mb-8">
			{#if user.bannerImage}
				<div class="relative h-48 md:h-64 rounded-lg overflow-hidden">
					<CachedImage
						src={user.bannerImage}
						alt="{user.name} banner"
						class="w-full h-full object-cover"
					/>
					<div class="absolute inset-0 bg-gradient-to-t from-background via-background/50 to-transparent"></div>
				</div>
			{/if}

			<div class="flex flex-col md:flex-row gap-6 mt-6">
				<!-- Avatar -->
				<div class="flex-shrink-0">
					{#if user.avatar?.large}
						<CachedImage
							src={user.avatar.large}
							alt={user.name}
							class="w-32 h-32 object-cover rounded-full shadow-lg mx-auto md:mx-0 border-4 border-background"
						/>
					{:else}
						<div class="w-32 h-32 bg-muted rounded-full flex items-center justify-center mx-auto md:mx-0">
							<Users class="w-16 h-16 text-muted-foreground" />
						</div>
					{/if}
				</div>

				<!-- Main Info -->
				<div class="flex-1 space-y-4">
					<div>
						<h1 class="text-3xl md:text-4xl font-bold mb-2">{user.name}</h1>

						<!-- User Info -->
						<div class="flex flex-wrap items-center gap-3 mb-4">
							{#if user.createdAt}
								<div class="flex items-center gap-1 text-muted-foreground">
									<Calendar class="w-4 h-4" />
									<span>Joined {formatDate(user.createdAt)}</span>
								</div>
							{/if}
							{#if user.donatorTier && user.donatorTier > 0}
								<Badge variant="default" class="bg-gradient-to-r from-pink-500 to-purple-600">
									<Trophy class="w-3 h-3 mr-1" />
									Donator
								</Badge>
							{/if}
							{#if user.moderatorRoles && user.moderatorRoles.length > 0}
								<Badge variant="destructive">
									Moderator
								</Badge>
							{/if}
						</div>

						{#if user.options?.profileColor}
							<div class="w-4 h-4 rounded-full border-2 border-background shadow-sm"
								 style="background-color: {user.options.profileColor}"></div>
						{/if}
					</div>
				</div>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Main Content -->
			<div class="lg:col-span-2 space-y-6">
				<!-- About -->
				{#if user.about}
					<Card>
						<CardHeader>
							<CardTitle>About</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="prose prose-sm max-w-none dark:prose-invert">
								{@html user.about}
							</div>
						</CardContent>
					</Card>
				{/if}

				<!-- Statistics -->
				{#if user.statistics}
					<Card>
						<CardHeader>
							<CardTitle>Statistics</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
								<!-- Anime Stats -->
								{#if user.statistics.anime}
									<div>
										<h4 class="font-semibold mb-3 flex items-center gap-2">
											<Play class="w-4 h-4" />
											Anime
										</h4>
										<div class="space-y-2">
											<div class="flex justify-between">
												<span class="text-muted-foreground">Total Anime:</span>
												<span class="font-medium">{user.statistics.anime.count}</span>
											</div>
											<div class="flex justify-between">
												<span class="text-muted-foreground">Episodes:</span>
												<span class="font-medium">{user.statistics.anime.episodesWatched}</span>
											</div>
											<div class="flex justify-between">
												<span class="text-muted-foreground">Time Watched:</span>
												<span class="font-medium">{formatMinutes(user.statistics.anime.minutesWatched)}</span>
											</div>
											{#if user.statistics.anime.meanScore}
												<div class="flex justify-between">
													<span class="text-muted-foreground">Mean Score:</span>
													<span class="font-medium">{user.statistics.anime.meanScore}/100</span>
												</div>
											{/if}
										</div>
									</div>
								{/if}

								<!-- Manga Stats -->
								{#if user.statistics.manga}
									<div>
										<h4 class="font-semibold mb-3 flex items-center gap-2">
											<BookOpen class="w-4 h-4" />
											Manga
										</h4>
										<div class="space-y-2">
											<div class="flex justify-between">
												<span class="text-muted-foreground">Total Manga:</span>
												<span class="font-medium">{user.statistics.manga.count}</span>
											</div>
											<div class="flex justify-between">
												<span class="text-muted-foreground">Chapters:</span>
												<span class="font-medium">{user.statistics.manga.chaptersRead}</span>
											</div>
											<div class="flex justify-between">
												<span class="text-muted-foreground">Volumes:</span>
												<span class="font-medium">{user.statistics.manga.volumesRead}</span>
											</div>
											{#if user.statistics.manga.meanScore}
												<div class="flex justify-between">
													<span class="text-muted-foreground">Mean Score:</span>
													<span class="font-medium">{user.statistics.manga.meanScore}/100</span>
												</div>
											{/if}
										</div>
									</div>
								{/if}
							</div>
						</CardContent>
					</Card>
				{/if}

				<!-- Favorites (commented out - not available in basic User type) -->
				<!-- {#if user.favourites}
					<Card>
						<CardHeader>
							<CardTitle>Favorites</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="grid grid-cols-1 md:grid-cols-2 gap-6"> -->
								<!-- Favorite Anime -->
								<!-- {#if false && user.favourites?.anime?.nodes && user.favourites.anime.nodes.length > 0}
									<div>
										<h4 class="font-semibold mb-3">Anime</h4>
										<div class="space-y-2">
											{#each user.favourites.anime.nodes.slice(0, 5) as anime}
												<a
													href="/anime/{anime.id}"
													class="flex items-center gap-3 p-2 rounded-lg hover:bg-muted transition-colors"
												>
													{#if anime.coverImage?.medium}
														<CachedImage
															src={anime.coverImage.medium}
															alt={anime.title}
															class="w-12 h-16 object-cover rounded"
														/>
													{/if}
													<div class="flex-1 min-w-0">
														<p class="font-medium truncate">{anime.title}</p>
														{#if anime.format}
															<p class="text-sm text-muted-foreground">{anime.format}</p>
														{/if}
													</div>
												</a>
											{/each}
										</div>
									</div>
								{/if} -->

								<!-- Favorite Manga -->
								<!-- {#if false && user.favourites?.manga?.nodes && user.favourites.manga.nodes.length > 0}
									<div>
										<h4 class="font-semibold mb-3">Manga</h4>
										<div class="space-y-2">
											{#each user.favourites.manga.nodes.slice(0, 5) as manga}
												<a
													href="/manga/{manga.id}"
													class="flex items-center gap-3 p-2 rounded-lg hover:bg-muted transition-colors"
												>
													{#if manga.coverImage?.medium}
														<CachedImage
															src={manga.coverImage.medium}
															alt={manga.title}
															class="w-12 h-16 object-cover rounded"
														/>
													{/if}
													<div class="flex-1 min-w-0">
														<p class="font-medium truncate">{manga.title}</p>
														{#if manga.format}
															<p class="text-sm text-muted-foreground">{manga.format}</p>
														{/if}
													</div>
												</a>
											{/each}
										</div>
									</div>
								{/if}
							</div>
						</CardContent>
					</Card>
				{/if} -->
			</div>

			<!-- Sidebar -->
			<div class="space-y-6">
				<!-- User Details -->
				<Card>
					<CardHeader>
						<CardTitle>Profile Information</CardTitle>
					</CardHeader>
					<CardContent class="space-y-3">
						<!-- {#if user.previousNames && user.previousNames.length > 0}
							<div>
								<p class="text-sm text-muted-foreground">Previous Names</p>
								<div class="space-y-1">
									{#each user.previousNames.slice(0, 3) as prevName}
										<p class="font-medium text-sm">{prevName.name}</p>
									{/each}
								</div>
							</div>
						{/if} -->

						{#if user.updatedAt}
							<div>
								<p class="text-sm text-muted-foreground">Last Active</p>
								<p class="font-medium">{formatDate(user.updatedAt)}</p>
							</div>
						{/if}

						{#if user.siteUrl}
							<div>
								<p class="text-sm text-muted-foreground">AniList Profile</p>
								<a
									href={user.siteUrl}
									target="_blank"
									rel="noopener noreferrer"
									class="text-primary hover:underline font-medium"
								>
									View on AniList
								</a>
							</div>
						{/if}
					</CardContent>
				</Card>

				<!-- Genres (if available in stats) -->
				<!-- {#if false && user.statistics?.anime?.genres && user.statistics.anime.genres.length > 0}
					<Card>
						<CardHeader>
							<CardTitle>Top Anime Genres</CardTitle>
						</CardHeader>
						<CardContent>
							<div class="space-y-2">
								{#each user.statistics.anime.genres.slice(0, 8) as genre}
									<div class="flex items-center justify-between">
										<span class="text-sm">{genre.genre}</span>
										<div class="flex items-center gap-2">
											<span class="text-xs text-muted-foreground">{genre.count}</span>
											<div class="w-16 h-2 bg-muted rounded-full overflow-hidden">
												<div
													class="h-full bg-primary rounded-full"
													style="width: {Math.min(100, (genre.count / user.statistics.anime.count) * 100)}%"
												></div>
											</div>
										</div>
									</div>
								{/each}
							</div>
						</CardContent>
					</Card>
				{/if} -->
			</div>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="text-center space-y-4">
					<h2 class="text-2xl font-bold">User Not Found</h2>
					<p class="text-muted-foreground">The requested user profile could not be found.</p>
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
