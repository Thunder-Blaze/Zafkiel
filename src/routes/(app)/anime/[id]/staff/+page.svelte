<script lang="ts">
	import { page } from '$app/state';
	import { useInfiniteAnimeStaffById } from '$lib/hooks/useAnilist.svelte';
	import StaffList from '../anime/StaffList.svelte';
	import PageLoader from '$lib/components/PageLoader.svelte';
	import type { AnimeLarge } from '$lib/types/anime';

	const animeId = $derived(page.params.id ? parseInt(page.params.id) : 0);
	const staffPerPage = 25;

	const animeQuery = $derived(useInfiniteAnimeStaffById(animeId, staffPerPage));

	const staff = $derived(
		(animeQuery.data as any)?.pages.flatMap((page: any) => {
			const data = page.data as AnimeLarge | undefined;
			return (
				data?.staff?.edges
					?.map((edge) =>
						edge.node
							? {
									id: edge.node.id,
									name: edge.node.name || {
										first: '',
										last: '',
										full: 'Unknown',
										native: '',
										userPreferred: 'Unknown'
									},
									image: {
										large: edge.node.image?.large || '/api/placeholder/230/345',
										medium: edge.node.image?.medium || '/api/placeholder/115/172'
									},
									role: edge.role || 'Staff'
								}
							: null
					)
					.filter((staff): staff is NonNullable<typeof staff> => staff !== null) || []
			);
		}) || []
	);

	const isLoading = $derived(animeQuery.isLoading);
	const isFetchingNextPage = $derived(animeQuery.isFetchingNextPage);
	const hasNextPage = $derived(animeQuery.hasNextPage);
</script>

<div class="mt-2 flex flex-col gap-4">
	<h2 class="text-xl font-semibold">Staff</h2>

	{#if isLoading && staff.length === 0}
		<PageLoader type="anime" />
	{:else}
		<StaffList
			{staff}
			{isLoading}
			{isFetchingNextPage}
			{hasNextPage}
			onFetchNextPage={() => animeQuery.fetchNextPage()}
		/>
	{/if}
</div>
