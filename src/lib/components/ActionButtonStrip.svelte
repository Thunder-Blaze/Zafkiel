<script lang="ts">
	import type { MediaListStatus } from "$lib/types/anilist";
	import Icon from "@iconify/svelte";

	let { userStatus = null }: { userStatus: MediaListStatus | null | undefined } = $props();

	// Status action handlers
	const handleStatusChange = (newStatus: MediaListStatus) => {
		// TODO: Implement API call to update status
		console.log('Status changed to:', newStatus);
	};
</script>

<div class="flex items-center justify-between gap-1 rounded-md bg-border p-1">
	<button
		onclick={() => handleStatusChange('PLANNING')}
		class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
		'PLANNING'
			? 'bg-primary/30'
			: 'bg-background/50'}"
		title="Plan to Watch"
	>
		<Icon icon="material-symbols:bookmark-outline" class="size-3.5" />
		Plan
	</button>
	<button
		onclick={() => handleStatusChange('CURRENT')}
		class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
		'CURRENT'
			? 'bg-primary/30'
			: 'bg-background/50'}"
		title="Watching"
	>
		<Icon icon="material-symbols:play-circle-outline" class="size-3.5" />
		Watch
	</button>
	<button
		onclick={() => handleStatusChange('COMPLETED')}
		class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
		'COMPLETED'
			? 'bg-primary/30'
			: 'bg-background/50'}"
		title="Completed"
	>
		<Icon icon="material-symbols:check-circle-outline" class="size-3.5" />
		Done
	</button>
	<button
		onclick={() => handleStatusChange('PAUSED')}
		class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
		'PAUSED'
			? 'bg-primary/30'
			: 'bg-background/50'}"
		title="Paused"
	>
		<Icon icon="material-symbols:pause-circle-outline" class="size-3.5" />
	</button>
	<button
		onclick={() => handleStatusChange('DROPPED')}
		class="flex flex-1 items-center justify-center gap-1 rounded px-2 py-1 text-[10px] font-medium transition-colors hover:bg-primary/20 {userStatus ===
		'DROPPED'
			? 'bg-primary/30'
			: 'bg-background/50'}"
		title="Dropped"
	>
		<Icon icon="material-symbols:cancel-outline" class="size-3.5" />
	</button>
</div>
