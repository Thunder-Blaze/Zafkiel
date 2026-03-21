<script lang="ts">
	import { page } from '$app/state';
	import { useStaffById } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { toast } from 'svelte-sonner';
	import { isAuthenticated } from '$lib/stores/auth';
	import { invoke } from '@tauri-apps/api/core';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';

	const staffId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const staffQuery = $derived(useStaffById(staffId));

	const staff = $derived(staffQuery.data?.data);
	const isLoading = $derived(staffQuery.isLoading);
	const error = $derived(staffQuery.error);

	let activeTab = $state('overview');

	let isFavourite = $state(false);
	let favouritesCount = $state(0);
	
	$effect(() => {
		if (staff) {
			isFavourite = staff?.isFavourite || false;
			favouritesCount = staff?.favourites || 0;
		}
	});

	let isToggling = $state(false);

	async function toggleFavourite() {
		if (!$isAuthenticated) {
			toast.error('You must be logged in to favourite a staff member.');
			return;
		}
		if (isToggling || !staff) return;
		
		isToggling = true;
		try {
			const res = await invoke<any>('favourite_staff', { id: staff.id });
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
	<title>{staff ? getTitle(staff.name) : 'Loading...'} - Zafkiel</title>
	<meta
		name="description"
		content={staff?.description
			? stripHtml(staff.description).slice(0, 160)
			: 'Staff details on Zafkiel'}
	/>
</svelte:head>

<div class="container mx-auto max-w-7xl px-4 py-8">
	{#if isLoading}
		<PageLoader type="staff" />
	{:else if error}
		<Card class="border-destructive">
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold text-destructive">Error Loading Staff</h2>
					<p class="text-muted-foreground">{error.message || 'Failed to load staff details'}</p>
					<Button variant="outline" onclick={() => staffQuery.refetch()}>Try Again</Button>
				</div>
			</CardContent>
		</Card>
	{:else if staff}
		<!-- Main Layout -->
		<div class="w-full">
			<div class="relative mb-8">
				<div class="flex flex-col gap-8 md:flex-row md:items-stretch">
					<!-- Image -->
					<div class="shrink-0">
						{#if staff.image?.large}
							<CachedImage
								src={staff.image.large}
								alt={getTitle(staff.name)}
								class="mx-auto h-80 w-56 rounded-xl object-cover shadow-2xl md:mx-0"
							/>
						{:else}
							<div
								class="mx-auto flex h-80 w-56 items-center justify-center rounded-xl bg-muted md:mx-0 shadow-2xl"
							>
								<Icon icon="solar:user-bold" class="h-20 w-20 text-muted-foreground" />
							</div>
						{/if}
					</div>

					<!-- Main Info -->
					<div class="flex flex-1 flex-col justify-end pb-0 space-y-4 md:space-y-0">
						<div>
							<h1 class="mb-2 text-4xl font-bold md:text-5xl lg:text-6xl text-pretty">
								{getTitle(staff.name)}
							</h1>
							{#if staff.name?.native}
								<h2 class="text-xl text-muted-foreground mb-5">{staff.name.native}</h2>
							{/if}

					<!-- Quick Info -->
							<div class="mt-6 mb-6 flex flex-wrap items-center gap-3">
								<button
									class="inline-flex cursor-pointer items-center gap-2 rounded-md px-3.5 py-1.5 text-sm font-semibold shadow-sm transition-all hover:-translate-y-0.5 active:scale-95 disabled:pointer-events-none disabled:opacity-50 {isFavourite ? 'border border-destructive/30 bg-destructive/15 text-destructive hover:bg-destructive/25 dark:border-[#E85D75]/30 dark:bg-[#E85D75]/15 dark:text-[#E85D75] dark:hover:bg-[#E85D75]/25' : 'border border-border/50 bg-card text-muted-foreground hover:bg-accent hover:text-accent-foreground'}"
									onclick={toggleFavourite}
									disabled={isToggling}
									aria-label={isFavourite ? "Remove from favourites" : "Add to favourites"}
								>
									{#if isToggling}
										<Icon icon="solar:spinner-bold" class="h-4 w-4 animate-spin" />
									{:else}
										<Icon icon={isFavourite ? "solar:heart-bold" : "solar:heart-linear"} class="h-4 w-4" />
									{/if}
									
									{#if favouritesCount > 0}
										{(favouritesCount >= 1000
											? (favouritesCount / 1000).toFixed(1) + 'k'
											: favouritesCount) + ' Favorites'}
									{:else}
										Favorites
									{/if}
								</button>
								{#if staff.gender}
									<div class="inline-flex items-center rounded-md bg-secondary px-2.5 py-1 text-xs font-semibold text-secondary-foreground shadow-sm">
										{staff.gender}
									</div>
								{/if}
								{#if staff.age}
									<div class="inline-flex items-center rounded-md bg-secondary px-2.5 py-1 text-xs font-semibold text-secondary-foreground shadow-sm">
										Age: {staff.age}
									</div>
								{/if}
							</div>
						</div>

						<!-- Content Tabs -->
						<div class="w-full border-b border-border/40">
							<div class="inline-flex h-9 w-full sm:w-auto items-center justify-start text-muted-foreground">
								<button
									onclick={() => activeTab = 'overview'}
									class={"flex-1 sm:flex-none border-b-2 px-6 h-full font-medium transition-colors hover:text-foreground text-sm flex items-center justify-center " +
										(activeTab === 'overview' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground')}
								>
									Overview
								</button>
								<button
									onclick={() => activeTab = 'media'}
									class={"flex-1 sm:flex-none border-b-2 px-6 h-full font-medium transition-colors hover:text-foreground text-sm flex items-center justify-center " +
										(activeTab === 'media' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground')}
								>
									Production Roles
								</button>
								<button
									onclick={() => activeTab = 'characters'}
									class={"flex-1 sm:flex-none border-b-2 px-6 h-full font-medium transition-colors hover:text-foreground text-sm flex items-center justify-center " +
										(activeTab === 'characters' ? 'border-primary text-foreground' : 'border-transparent text-muted-foreground')}
								>
									Voice Roles
								</button>
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class="mt-8 w-full">
				{#if activeTab === 'overview'}
					<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
						<!-- Main Content -->
						<div class="space-y-8 lg:col-span-2">
							<!-- Description -->
							{#if staff.description}
								<div class="custom-scrollbar text-sm leading-relaxed text-foreground/90 markdown-wrapper">
									<MarkdownRenderer body={staff.description} />
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
									{#if staff.dateOfBirth?.year || staff.dateOfBirth?.month || staff.dateOfBirth?.day}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Birthday</span>
											<span class="font-medium">
												{staff.dateOfBirth.day}/{staff.dateOfBirth.month}
												{#if staff.dateOfBirth.year}/{staff.dateOfBirth.year}{/if}
											</span>
										</div>
									{/if}
									{#if staff.dateOfDeath?.year || staff.dateOfDeath?.month || staff.dateOfDeath?.day}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Death</span>
											<span class="font-medium">
												{staff.dateOfDeath.day}/{staff.dateOfDeath.month}
												{#if staff.dateOfDeath.year}/{staff.dateOfDeath.year}{/if}
											</span>
										</div>
									{/if}
									{#if staff.homeTown}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Hometown</span>
											<span class="font-medium">{staff.homeTown}</span>
										</div>
									{/if}
									{#if staff.bloodType}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">Blood Type</span>
											<span class="font-medium">{staff.bloodType}</span>
										</div>
									{/if}
									{#if staff.siteUrl}
										<div class="flex justify-between">
											<span class="text-sm text-muted-foreground">AniList Profile</span>
											<a
												href={staff.siteUrl}
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

							<!-- Years Active -->
							{#if staff.yearsActive && staff.yearsActive.length > 0}
								<Card>
									<CardHeader>
										<CardTitle>Years Active</CardTitle>
									</CardHeader>
									<CardContent>
										<p class="font-medium">
											{staff.yearsActive[0]} - {staff.yearsActive.length > 1
												? staff.yearsActive[1] || 'Present'
												: 'Present'}
										</p>
									</CardContent>
								</Card>
							{/if}
						</div>
					</div>
				{/if}

				{#if activeTab === 'media'}
					{#if staff.staffMedia?.edges && staff.staffMedia.edges.length > 0}
						<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
							{#each staff.staffMedia.edges as edge}
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
													{#if edge.staffRole}
														<Badge variant="secondary" class="mt-1 text-[10px]">
															{edge.staffRole}
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
							<h3 class="text-lg font-semibold">No Production Roles Found</h3>
							<p class="text-muted-foreground">
								This staff member doesn't have any production roles listed.
							</p>
						</div>
					{/if}
				{/if}

				{#if activeTab === 'characters'}
					{#if staff.characters?.edges && staff.characters.edges.length > 0}
						<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
							{#each staff.characters.edges as edge}
								{#if edge.node}
									<a
										href="/character/{edge.node.id}"
										class="group relative block overflow-hidden rounded-lg bg-card transition-all hover:scale-105 hover:shadow-lg"
									>
										<div class="aspect-[2/3] w-full overflow-hidden">
											{#if edge.node.image?.large}
												<CachedImage
													src={edge.node.image.large}
													alt={getTitle(edge.node.name)}
													class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-110"
												/>
											{:else}
												<div class="flex h-full w-full items-center justify-center bg-muted">
													<Icon icon="solar:user-bold" class="h-12 w-12 text-muted-foreground" />
												</div>
											{/if}
											<div
												class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
											>
												<div class="absolute right-0 bottom-0 left-0 p-4">
													<p class="line-clamp-2 text-sm font-semibold text-white">
														{getTitle(edge.node.name)}
													</p>
													{#if edge.role}
														<Badge variant="secondary" class="mt-1 text-[10px]">
															{edge.role}
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
								icon="solar:microphone-3-bold-duotone"
								class="mb-4 h-12 w-12 text-muted-foreground"
							/>
							<h3 class="text-lg font-semibold">No Voice Roles Found</h3>
							<p class="text-muted-foreground">
								This staff member doesn't have any voice acting roles listed.
							</p>
						</div>
					{/if}
				{/if}
			</div>
		</div>
	{:else}
		<Card>
			<CardContent class="pt-6">
				<div class="space-y-4 text-center">
					<h2 class="text-2xl font-bold">Staff Not Found</h2>
					<p class="text-muted-foreground">The requested staff member could not be found.</p>
					<Button variant="outline" onclick={() => history.back()}>Go Back</Button>
				</div>
			</CardContent>
		</Card>
	{/if}
</div>

<style>
</style>
