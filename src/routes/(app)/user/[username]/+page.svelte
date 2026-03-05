<script lang="ts">
	import { page } from '$app/state';
	import { useUserByName, useUserReviews, useRecentActivity } from '$lib/hooks/useAnilist.svelte';
	import type { ActivityUnion, Review } from '$lib/types/anilist';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import { Separator } from '$lib/components/ui/separator';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import ActivityCard from '$lib/components/ActivityCard.svelte';
	import ReviewCard from '$lib/components/ReviewCard.svelte';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';
	import Icon from '@iconify/svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';
	import { goto } from '$app/navigation';

	const userName = $derived(page.params.username ?? '');
	const userQuery = $derived(useUserByName(userName));

	const user = $derived(userQuery.data?.data);
	const isLoading = $derived(userQuery.isLoading);
	const error = $derived(userQuery.error);

	const activityQuery = $derived(useRecentActivity(1, 10));
	const activities = $derived((activityQuery.data?.data?.data ?? []) as ActivityUnion[]);

	const reviewsQuery = $derived(useUserReviews(user?.id ?? 0));
	const reviews = $derived((reviewsQuery.data?.data?.data ?? []) as Review[]);

	function formatDate(timestamp?: number) {
		if (!timestamp) return 'Unknown';
		return new Date(timestamp * 1000).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
		});
	}

	function formatMinutes(minutes?: number) {
		if (!minutes) return '0h';
		const h = Math.floor(minutes / 60);
		const m = minutes % 60;
		return m === 0 ? `${h}h` : `${h}h ${m}m`;
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

{#if isLoading}
	<div class="container mx-auto max-w-6xl px-4 py-8">
		<PageLoader type="user" />
	</div>
{:else if error || (userQuery.data && !userQuery.data.success)}
	<div class="container mx-auto max-w-6xl px-4 py-8">
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
	</div>
{:else if user}
	<!-- Full-width banner -->
	<div class="relative h-40 w-full overflow-hidden md:h-56 xl:h-64">
		{#if user.bannerImage}
			<CachedImage
				src={user.bannerImage}
				alt="{user.name} banner"
				class="h-full w-full object-cover"
			/>
			<div class="absolute inset-0 bg-linear-to-t from-background/80 to-transparent"></div>
		{:else}
			<div class="h-full w-full bg-linear-to-br from-primary/20 via-muted to-muted"></div>
		{/if}
	</div>

	<!-- Profile header (overlaps banner via negative margin) -->
	<div class="container mx-auto max-w-6xl px-4">
		<div class="-mt-16 flex flex-col gap-4 md:-mt-12 md:flex-row md:items-end md:gap-6">
			<!-- Avatar -->
			<div class="shrink-0">
				{#if user.avatar?.large}
					<CachedImage
						src={user.avatar.large}
						alt={user.name}
						class="h-28 w-28 rounded-xl border-4 border-background object-cover shadow-xl md:h-32 md:w-32"
					/>
				{:else}
					<div
						class="flex h-28 w-28 items-center justify-center rounded-xl border-4 border-background bg-muted shadow-xl md:h-32 md:w-32"
					>
						<Icon icon="solar:user-bold" class="h-14 w-14 text-muted-foreground" />
					</div>
				{/if}
			</div>

			<!-- Name + meta -->
			<div class="flex flex-1 flex-col justify-end gap-2 pb-1">
				<div class="flex flex-wrap items-center gap-2">
					<h1 class="text-2xl font-bold md:text-3xl">{user.name}</h1>
					{#if user.donatorTier && user.donatorTier > 0}
						<Badge class="bg-linear-to-r from-pink-500 to-purple-600 text-white">
							<Icon icon="solar:cup-star-bold" class="mr-1 h-3 w-3" />
							Donator
						</Badge>
					{/if}
					{#if user.moderatorRoles && user.moderatorRoles.length > 0}
						<Badge variant="destructive">
							<Icon icon="solar:shield-bold" class="mr-1 h-3 w-3" />
							Moderator
						</Badge>
					{/if}
				</div>

				<div class="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
					{#if user.createdAt}
						<span class="flex items-center gap-1">
							<Icon icon="solar:calendar-date-bold" class="h-3.5 w-3.5" />
							Joined {formatDate(user.createdAt)}
						</span>
					{/if}
					{#if user.updatedAt}
						<span class="flex items-center gap-1">
							<Icon icon="solar:clock-circle-bold" class="h-3.5 w-3.5" />
							Active {formatDate(user.updatedAt)}
						</span>
					{/if}
					{#if user.siteUrl}
						<a
							href={user.siteUrl}
							target="_blank"
							rel="noopener noreferrer"
							class="flex items-center gap-1 text-primary hover:underline"
						>
							<Icon icon="solar:link-bold" class="h-3.5 w-3.5" />
							AniList
						</a>
					{/if}
				</div>
			</div>

			<!-- Quick stats -->
			{#if user.statistics}
				<div class="flex shrink-0 gap-4 pb-1">
					{#if user.statistics.anime}
						<div class="text-center">
							<p class="text-xl font-bold">{user.statistics.anime.count ?? 0}</p>
							<p class="text-xs text-muted-foreground">Anime</p>
						</div>
					{/if}
					{#if user.statistics.manga}
						<div class="text-center">
							<p class="text-xl font-bold">{user.statistics.manga.count ?? 0}</p>
							<p class="text-xs text-muted-foreground">Manga</p>
						</div>
					{/if}
					{#if user.statistics.anime?.episodesWatched}
						<div class="text-center">
							<p class="text-xl font-bold">{user.statistics.anime.episodesWatched}</p>
							<p class="text-xs text-muted-foreground">Episodes</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<Separator class="mt-6" />

		<!-- Tabs -->
		<Tabs value="overview" class="mt-0 w-full">
			<TabsList class="w-full justify-start overflow-x-auto rounded-none border-b bg-transparent p-0">
				{#each [['overview', 'Overview'], ['stats', 'Statistics'], ['activity', 'Activity'], ['reviews', 'Reviews']] as [val, label] (val)}
					<TabsTrigger
						value={val}
						class="rounded-none border-b-2 border-transparent px-5 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						{label}
					</TabsTrigger>
				{/each}
			</TabsList>

			<!-- Overview -->
			<TabsContent value="overview" class="mt-6">
				<div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
					<div class="space-y-6 lg:col-span-2">
						{#if user.about}
							<div>
								<h3 class="mb-3 text-base font-semibold">About</h3>
								<MarkdownRenderer body={user.about} class="text-sm" />
							</div>
						{:else}
							<div
								class="flex min-h-[120px] items-center justify-center rounded-lg border border-dashed text-center"
							>
								<p class="text-sm text-muted-foreground">No bio yet.</p>
							</div>
						{/if}
					</div>

					<div class="space-y-4">
						<Card>
							<CardHeader class="pb-2">
								<CardTitle class="text-sm font-semibold">Lists</CardTitle>
							</CardHeader>
							<CardContent class="flex flex-col gap-2">
								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start gap-2"
									onclick={() => goto(`/user/${user.name}/animelist`)}
								>
									<Icon icon="solar:play-circle-bold-duotone" class="size-4 text-primary" />
									Anime List
								</Button>
								<Button
									variant="outline"
									size="sm"
									class="w-full justify-start gap-2"
									onclick={() => goto(`/user/${user.name}/mangalist`)}
								>
									<Icon icon="solar:book-2-bold-duotone" class="size-4 text-primary" />
									Manga List
								</Button>
							</CardContent>
						</Card>
					</div>
				</div>
			</TabsContent>

			<!-- Statistics -->
			<TabsContent value="stats" class="mt-6">
				{#if user.statistics}
					<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
						{#if user.statistics.anime}
							<Card>
								<CardHeader class="pb-2">
									<CardTitle class="flex items-center gap-2 text-base">
										<Icon icon="solar:play-circle-bold" class="h-4 w-4 text-primary" />
										Anime
									</CardTitle>
								</CardHeader>
								<CardContent class="space-y-0">
									{#each [
										['Total', user.statistics.anime.count],
										['Episodes Watched', user.statistics.anime.episodesWatched],
										['Time Watched', formatMinutes(user.statistics.anime.minutesWatched)],
										['Mean Score', user.statistics.anime.meanScore ? `${user.statistics.anime.meanScore}/100` : null],
									] as [label, val] (label)}
										{#if val !== null && val !== undefined}
											<div class="flex justify-between border-b py-2 last:border-0">
												<span class="text-sm text-muted-foreground">{label}</span>
												<span class="text-sm font-medium">{val}</span>
											</div>
										{/if}
									{/each}
								</CardContent>
							</Card>
						{/if}

						{#if user.statistics.manga}
							<Card>
								<CardHeader class="pb-2">
									<CardTitle class="flex items-center gap-2 text-base">
										<Icon icon="solar:book-2-bold" class="h-4 w-4 text-primary" />
										Manga
									</CardTitle>
								</CardHeader>
								<CardContent class="space-y-0">
									{#each [
										['Total', user.statistics.manga.count],
										['Chapters Read', user.statistics.manga.chaptersRead],
										['Volumes Read', user.statistics.manga.volumesRead],
										['Mean Score', user.statistics.manga.meanScore ? `${user.statistics.manga.meanScore}/100` : null],
									] as [label, val] (label)}
										{#if val !== null && val !== undefined}
											<div class="flex justify-between border-b py-2 last:border-0">
												<span class="text-sm text-muted-foreground">{label}</span>
												<span class="text-sm font-medium">{val}</span>
											</div>
										{/if}
									{/each}
								</CardContent>
							</Card>
						{/if}
					</div>
				{:else}
					<div
						class="flex min-h-[200px] flex-col items-center justify-center gap-2 rounded-lg border border-dashed text-center"
					>
						<Icon icon="solar:chart-square-bold-duotone" class="h-10 w-10 text-muted-foreground" />
						<p class="text-sm text-muted-foreground">No statistics available.</p>
					</div>
				{/if}
			</TabsContent>

			<!-- Activity -->
			<TabsContent value="activity" class="mt-6">
				{#if activityQuery.isLoading}
					<div class="flex min-h-[200px] items-center justify-center">
						<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
					</div>
				{:else if activities.length === 0}
					<div
						class="flex min-h-[200px] flex-col items-center justify-center gap-2 rounded-lg border border-dashed text-center"
					>
						<Icon icon="solar:widget-2-bold-duotone" class="h-10 w-10 text-muted-foreground" />
						<p class="text-sm text-muted-foreground">No recent activity.</p>
					</div>
				{:else}
					<div class="flex flex-col gap-3">
						{#each activities as activity, i (i)}
							<ActivityCard {activity} />
						{/each}
					</div>
				{/if}
			</TabsContent>

			<!-- Reviews -->
			<TabsContent value="reviews" class="mt-6">
				{#if reviewsQuery.isLoading}
					<div class="flex min-h-[200px] items-center justify-center">
						<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
					</div>
				{:else if reviews.length === 0}
					<div
						class="flex min-h-[200px] flex-col items-center justify-center gap-2 rounded-lg border border-dashed text-center"
					>
						<Icon icon="solar:document-text-bold-duotone" class="h-10 w-10 text-muted-foreground" />
						<p class="text-sm text-muted-foreground">No reviews written yet.</p>
					</div>
				{:else}
					<div class="flex flex-col gap-4 pb-8">
						{#each reviews as review (review.id)}
							<ReviewCard {review} />
						{/each}
					</div>
				{/if}
			</TabsContent>
		</Tabs>
	</div>
{:else}
	<div class="container mx-auto max-w-6xl px-4 py-8">
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">User Not Found</h2>
					<p class="text-muted-foreground">The requested user profile could not be found.</p>
					<Button variant="outline" onclick={() => history.back()}>Go Back</Button>
				</div>
			</CardContent>
		</Card>
	</div>
{/if}
