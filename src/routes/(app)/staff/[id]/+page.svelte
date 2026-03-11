<script lang="ts">
	import { page } from '$app/state';
	import { useStaffById } from '$lib/hooks/useAnilist.svelte';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Tabs, TabsContent, TabsList, TabsTrigger } from '$lib/components/ui/tabs';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';

	const staffId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const staffQuery = $derived(useStaffById(staffId));

	const staff = $derived(staffQuery.data?.data);
	const isLoading = $derived(staffQuery.isLoading);
	const error = $derived(staffQuery.error);

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
		<!-- Hero Section -->
		<div class="relative mb-8">
			<div class="flex flex-col gap-8 md:flex-row">
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
							{getTitle(staff.name)}
						</h1>
						{#if staff.name?.native}
							<h2 class="text-xl text-muted-foreground">{staff.name.native}</h2>
						{/if}

						<!-- Quick Info -->
						<div class="mt-6 flex flex-wrap items-center gap-3">
							{#if staff.favourites}
								<Badge variant="outline" class="gap-1">
									<Icon icon="solar:heart-bold" class="h-3 w-3 text-red-500" />
									{staff.favourites.toLocaleString()} Favorites
								</Badge>
							{/if}
							{#if staff.gender}
								<Badge variant="secondary">
									{staff.gender}
								</Badge>
							{/if}
							{#if staff.age}
								<Badge variant="secondary">
									Age: {staff.age}
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
						Production Roles
					</TabsTrigger>
					<TabsTrigger
						value="characters"
						class="rounded-none border-b-2 border-transparent px-6 py-3 data-[state=active]:border-primary data-[state=active]:bg-transparent data-[state=active]:shadow-none"
					>
						Voice Roles
					</TabsTrigger>
				</TabsList>

				<TabsContent value="overview" class="mt-6">
					<div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
						<!-- Main Content -->
						<div class="space-y-8 lg:col-span-2">
							<!-- Description -->
							{#if staff.description}
								<div class="space-y-4">
									<h3 class="text-lg font-semibold">Description</h3>
									<div class="prose prose-sm max-w-none text-muted-foreground dark:prose-invert">
										{@html staff.description}
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
				</TabsContent>

				<TabsContent value="media" class="mt-6">
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
				</TabsContent>

				<TabsContent value="characters" class="mt-6">
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
				</TabsContent>
			</Tabs>
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
	:global(.prose p) {
		margin-bottom: 1rem;
	}

	:global(.prose br) {
		margin-bottom: 0.5rem;
	}
</style>
