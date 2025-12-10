<script lang="ts">
	import { page } from '$app/state';
	import { useUserById } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

	const userId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const userQuery = $derived(useUserById(userId));

	const user = $derived(userQuery.data?.data);
	const isLoading = $derived(userQuery.isLoading);
	const error = $derived(userQuery.error);

	$effect(() => {
		console.log('Profile Page Debug:', {
			userId,
			isLoading,
			error,
			hasUser: !!user,
			data: userQuery.data,
		});
	});

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
	<meta
		name="description"
		content={user?.about ? stripHtml(user.about).slice(0, 160) : 'User profile on Zafkiel'}
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
				<p class="text-muted-foreground">Loading user profile...</p>
			</div>
		</div>
	{:else if error || (userQuery.data && !userQuery.data.success)}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Profile</h2>
					<p class="text-muted-foreground">
						{error?.message || userQuery.data?.error || 'Failed to load user profile'}
					</p>
					<Button variant="outline" onclick={() => userQuery.refetch()}>Try Again</Button>
				</div>
			</CardContent>
		</Card>
	{:else if user}
		<!-- Hero Section -->
		<div class="relative mb-8">
			{#if user.bannerImage}
				<div class="relative h-48 overflow-hidden rounded-xl md:h-80">
					<CachedImage
						src={user.bannerImage}
						alt="{user.name} banner"
						class="h-full w-full object-cover"
					/>
					<div
						class="absolute inset-0 bg-gradient-to-t from-background via-background/60 to-transparent"
					></div>
				</div>
			{/if}

			<div class="relative z-10 -mt-24 px-4 md:px-8">
				<div class="flex flex-col gap-6 md:flex-row">
					<!-- Avatar -->
					<div class="shrink-0">
						{#if user.avatar?.large}
							<CachedImage
								src={user.avatar.large}
								alt={user.name}
								class="mx-auto h-40 w-40 rounded-full border-4 border-background object-cover shadow-2xl md:mx-0"
							/>
						{:else}
							<div
								class="mx-auto flex h-40 w-40 items-center justify-center rounded-full border-4 border-background bg-muted md:mx-0"
							>
								<Icon icon="solar:user-bold" class="h-20 w-20 text-muted-foreground" />
							</div>
						{/if}
					</div>

					<!-- Main Info -->
					<div class="flex flex-1 flex-col justify-end space-y-4 pb-4">
						<div>
							<h1 class="mb-2 text-3xl font-bold md:text-4xl">{user.name}</h1>

							<!-- User Info -->
							<div class="flex flex-wrap items-center gap-3">
								{#if user.createdAt}
									<div class="flex items-center gap-1 text-muted-foreground">
										<Icon icon="solar:calendar-date-bold" class="h-4 w-4" />
										<span>Joined {formatDate(user.createdAt)}</span>
									</div>
								{/if}
								{#if user.donatorTier && user.donatorTier > 0}
									<Badge variant="default" class="bg-gradient-to-r from-pink-500 to-purple-600">
										<Icon icon="solar:cup-star-bold" class="mr-1 h-3 w-3" />
										Donator
									</Badge>
								{/if}
								{#if user.moderatorRoles && user.moderatorRoles.length > 0}
									<Badge variant="destructive">Moderator</Badge>
								{/if}
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
						value="stats"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Statistics
					</TabsTrigger>
				</TabsList>

				<TabsContent value="overview" class="mt-6">
					<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
						<!-- Main Content -->
						<div class="space-y-8 lg:col-span-2">
							<!-- About -->
							{#if user.about}
								<div class="space-y-4">
									<h3 class="text-lg font-semibold">About</h3>
									<div class="prose prose-sm dark:prose-invert max-w-none text-muted-foreground">
										{@html user.about}
									</div>
								</div>
							{/if}
						</div>

						<!-- Sidebar -->
						<div class="space-y-8">
							<!-- User Details -->
							<Card>
								<CardHeader>
									<CardTitle>Profile Information</CardTitle>
								</CardHeader>
								<CardContent class="space-y-4">
									{#if user.updatedAt}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Last Active</span>
											<span class="font-medium">{formatDate(user.updatedAt)}</span>
										</div>
									{/if}

									{#if user.siteUrl}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">AniList Profile</span>
											<a
												href={user.siteUrl}
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
						</div>
					</div>
				</TabsContent>

				<TabsContent value="stats" class="mt-6">
					{#if user.statistics}
						<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
							<!-- Anime Stats -->
							{#if user.statistics.anime}
								<Card>
									<CardHeader>
										<CardTitle class="flex items-center gap-2">
											<Icon icon="solar:play-circle-bold" class="h-5 w-5 text-primary" />
											Anime Statistics
										</CardTitle>
									</CardHeader>
									<CardContent class="space-y-4">
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Total Anime</span>
											<span class="font-medium">{user.statistics.anime.count}</span>
										</div>
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Episodes Watched</span>
											<span class="font-medium">{user.statistics.anime.episodesWatched}</span>
										</div>
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Time Watched</span>
											<span class="font-medium"
												>{formatMinutes(user.statistics.anime.minutesWatched)}</span
											>
										</div>
										{#if user.statistics.anime.meanScore}
											<div class="flex justify-between border-b py-2">
												<span class="text-muted-foreground">Mean Score</span>
												<span class="font-medium">{user.statistics.anime.meanScore}/100</span>
											</div>
										{/if}
									</CardContent>
								</Card>
							{/if}

							<!-- Manga Stats -->
							{#if user.statistics.manga}
								<Card>
									<CardHeader>
										<CardTitle class="flex items-center gap-2">
											<Icon icon="solar:book-2-bold" class="h-5 w-5 text-primary" />
											Manga Statistics
										</CardTitle>
									</CardHeader>
									<CardContent class="space-y-4">
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Total Manga</span>
											<span class="font-medium">{user.statistics.manga.count}</span>
										</div>
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Chapters Read</span>
											<span class="font-medium">{user.statistics.manga.chaptersRead}</span>
										</div>
										<div class="flex justify-between border-b py-2">
											<span class="text-muted-foreground">Volumes Read</span>
											<span class="font-medium">{user.statistics.manga.volumesRead}</span>
										</div>
										{#if user.statistics.manga.meanScore}
											<div class="flex justify-between border-b py-2">
												<span class="text-muted-foreground">Mean Score</span>
												<span class="font-medium">{user.statistics.manga.meanScore}/100</span>
											</div>
										{/if}
									</CardContent>
								</Card>
							{/if}
						</div>
					{:else}
						<div
							class="flex min-h-[200px] flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center"
						>
							<Icon
								icon="solar:chart-square-bold-duotone"
								class="mb-4 h-12 w-12 text-muted-foreground"
							/>
							<h3 class="text-lg font-semibold">No Statistics Available</h3>
							<p class="text-muted-foreground">User statistics are not available.</p>
						</div>
					{/if}
				</TabsContent>
			</Tabs>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">User Not Found</h2>
					<p class="text-muted-foreground">The requested user profile could not be found.</p>
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
