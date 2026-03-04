<script lang="ts">
	import { useAiringAnime, useUpcomingAnime } from '$lib/hooks/useAnilist.svelte';
	import type { Media } from '$lib/types/anilist';
	import MediaCard from '$lib/components/MediaCard.svelte';
	import AiringCountdown from '$lib/components/AiringCountdown.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/components/ui/tabs';
	import { Badge } from '$lib/components/ui/badge';
	import Icon from '@iconify/svelte';
	import { gsapReveal, gsapStagger } from '$lib/utils/gsap-animations';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
	const today = new Date().getDay();

	// ── Airing ──────────────────────────────────────────────────────────────────
	let airingPage = $state(1);
	const airingQuery = $derived(useAiringAnime(airingPage, 50));
	const airingItems = $derived((airingQuery.data?.data?.data ?? []) as Media[]);
	const airingPageInfo = $derived(airingQuery.data?.data?.pageInfo);

	/** Group airing anime by day-of-week using nextAiringEpisode.airingAt */
	const airingByDay = $derived(
		DAYS.map((day, idx) => ({
			day,
			index: idx,
			isToday: idx === today,
			items: airingItems.filter((m) => {
				if (!m.nextAiringEpisode?.airingAt) return false;
				return new Date(m.nextAiringEpisode.airingAt * 1000).getDay() === idx;
			}),
		})).filter((g) => g.items.length > 0)
	);

	/** Anime with no nextAiringEpisode data */
	const airingUnknownDay = $derived(airingItems.filter((m) => !m.nextAiringEpisode?.airingAt));

	// ── Upcoming ─────────────────────────────────────────────────────────────────
	let upcomingPage = $state(1);
	const upcomingQuery = $derived(useUpcomingAnime(upcomingPage, 30));
	const upcomingItems = $derived((upcomingQuery.data?.data?.data ?? []) as Media[]);
	const upcomingPageInfo = $derived(upcomingQuery.data?.data?.pageInfo);

	function formatAiringTime(airingAt: number): string {
		const date = new Date(airingAt * 1000);
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
	}

	let showUnknown = $state(false);
</script>

<svelte:head>
	<title>Schedule · Zafkiel</title>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-8">
	<!-- Header -->
	<div use:gsapReveal class="mb-8 flex items-center gap-3">
		<div class="flex size-10 items-center justify-center rounded-xl bg-primary/10">
			<Icon icon="solar:calendar-bold-duotone" class="size-6 text-primary" />
		</div>
		<div>
			<h1 class="text-2xl font-bold">Airing Schedule</h1>
			<p class="text-sm text-muted-foreground">Currently airing and upcoming anime</p>
		</div>
	</div>

	<Tabs value="airing" class="w-full">
		<TabsList class="mb-6">
			<TabsTrigger value="airing" class="flex items-center gap-2">
				<Icon icon="solar:play-bold-duotone" class="size-4" />
				Currently Airing
			</TabsTrigger>
			<TabsTrigger value="upcoming" class="flex items-center gap-2">
				<Icon icon="solar:clock-circle-bold-duotone" class="size-4" />
				Upcoming
			</TabsTrigger>
		</TabsList>

		<!-- Airing Tab -->
		<TabsContent value="airing">
			{#if airingQuery.isLoading}
				<PageLoader type="default" />
			{:else if airingQuery.error}
				<div
					class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-12 text-center"
				>
					<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
					<h3 class="text-lg font-semibold">Failed to load schedule</h3>
					<Button variant="outline" onclick={() => airingQuery.refetch()}>Retry</Button>
				</div>
			{:else}
				<!-- Day-grouped sections -->
				{#each airingByDay as group (group.day)}
					<section use:gsapReveal class="mb-8">
						<div class="mb-4 flex items-center gap-3">
							<h2 class="text-lg font-semibold">{group.day}</h2>
							{#if group.isToday}
								<Badge variant="default" class="bg-primary text-primary-foreground">Today</Badge>
							{/if}
							<span class="text-sm text-muted-foreground">({group.items.length} anime)</span>
						</div>
						<div
							class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
						>
							{#each group.items as anime (anime.id)}
								<div class="relative">
									<!-- Airing time badge -->
									{#if anime.nextAiringEpisode}
										<div class="absolute top-2 left-2 z-10 flex flex-col gap-1">
											<div
												class="rounded-md bg-black/75 px-1.5 py-0.5 text-xs text-white backdrop-blur-sm"
											>
												Ep {anime.nextAiringEpisode.episode}
											</div>
											<div
												class="rounded-md bg-primary/85 px-1.5 py-0.5 text-xs text-primary-foreground backdrop-blur-sm"
											>
												{formatAiringTime(anime.nextAiringEpisode.airingAt)}
											</div>
										</div>
									{/if}
									<MediaCard media={anime} />
								</div>
							{/each}
						</div>
					</section>
				{/each}

				<!-- Anime without known airing day -->
				{#if airingUnknownDay.length > 0}
					<section class="mb-8">
						<button
							class="mb-4 flex w-full items-center gap-3 text-left"
							onclick={() => (showUnknown = !showUnknown)}
						>
							<h2 class="text-lg font-semibold">Unknown Schedule</h2>
							<span class="text-sm text-muted-foreground">({airingUnknownDay.length} anime)</span>
							<Icon
								icon="solar:alt-arrow-down-bold"
								class="ml-auto size-4 text-muted-foreground transition-transform {showUnknown
									? 'rotate-180'
									: ''}"
							/>
						</button>
						{#if showUnknown}
							<div
								class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
							>
								{#each airingUnknownDay as anime (anime.id)}
									<MediaCard media={anime} />
								{/each}
							</div>
						{/if}
					</section>
				{/if}

				{#if airingByDay.length === 0 && airingUnknownDay.length === 0}
					<div
						class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-12 text-center"
					>
						<Icon
							icon="solar:calendar-minimalistic-bold-duotone"
							class="h-12 w-12 text-muted-foreground"
						/>
						<h3 class="text-lg font-semibold">No Airing Anime</h3>
						<p class="text-sm text-muted-foreground">
							No airing schedule data available right now.
						</p>
					</div>
				{/if}

				<!-- Pagination -->
				{#if airingPageInfo && (airingPage > 1 || airingPageInfo.hasNextPage)}
					<div class="mt-6 flex items-center justify-center gap-3">
						<Button variant="outline" disabled={airingPage <= 1} onclick={() => (airingPage -= 1)}>
							<Icon icon="solar:arrow-left-linear" class="size-4" />
							Previous
						</Button>
						<span class="text-sm text-muted-foreground">Page {airingPage}</span>
						<Button
							variant="outline"
							disabled={!airingPageInfo.hasNextPage}
							onclick={() => (airingPage += 1)}
						>
							Next
							<Icon icon="solar:arrow-right-linear" class="size-4" />
						</Button>
					</div>
				{/if}
			{/if}
		</TabsContent>

		<!-- Upcoming Tab -->
		<TabsContent value="upcoming">
			{#if upcomingQuery.isLoading}
				<PageLoader type="default" />
			{:else if upcomingQuery.error}
				<div
					class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-12 text-center"
				>
					<Icon icon="solar:danger-triangle-bold-duotone" class="h-12 w-12 text-destructive" />
					<h3 class="text-lg font-semibold">Failed to load upcoming anime</h3>
					<Button variant="outline" onclick={() => upcomingQuery.refetch()}>Retry</Button>
				</div>
			{:else if upcomingItems.length === 0}
				<div
					class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed p-12 text-center"
				>
					<Icon icon="solar:clock-circle-bold-duotone" class="h-12 w-12 text-muted-foreground" />
					<h3 class="text-lg font-semibold">No Upcoming Anime</h3>
					<p class="text-sm text-muted-foreground">Check back later for upcoming titles.</p>
				</div>
			{:else}
				<div
					use:gsapStagger
					class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
				>
					{#each upcomingItems as anime (anime.id)}
						<div class="relative">
							{#if anime.nextAiringEpisode}
								<div class="absolute top-2 left-2 z-10 flex flex-col gap-1">
									<div
										class="rounded-md bg-primary/85 px-1.5 py-0.5 text-xs text-primary-foreground backdrop-blur-sm"
									>
										in <AiringCountdown timeUntilAiring={anime.nextAiringEpisode.timeUntilAiring} />
									</div>
								</div>
							{/if}
							<MediaCard media={anime} />
						</div>
					{/each}
				</div>

				{#if upcomingPageInfo && (upcomingPage > 1 || upcomingPageInfo.hasNextPage)}
					<div class="mt-6 flex items-center justify-center gap-3">
						<Button
							variant="outline"
							disabled={upcomingPage <= 1}
							onclick={() => (upcomingPage -= 1)}
						>
							<Icon icon="solar:arrow-left-linear" class="size-4" />
							Previous
						</Button>
						<span class="text-sm text-muted-foreground">Page {upcomingPage}</span>
						<Button
							variant="outline"
							disabled={!upcomingPageInfo.hasNextPage}
							onclick={() => (upcomingPage += 1)}
						>
							Next
							<Icon icon="solar:arrow-right-linear" class="size-4" />
						</Button>
					</div>
				{/if}
			{/if}
		</TabsContent>
	</Tabs>
</div>
