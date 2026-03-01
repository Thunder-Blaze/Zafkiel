<script lang="ts">
	import Icon from '@iconify/svelte';

	// Placeholder for now as the type isn't fully defined in the user's snippet
	export let activities: any[] = [];
	export let isLoading: boolean = false;
</script>

{#if isLoading}
	<div class="flex items-center justify-center p-8">
		<Icon icon="solar:refresh-circle-line-duotone" class="h-8 w-8 animate-spin text-primary" />
	</div>
{:else if activities.length > 0}
	<div class="space-y-4">
		{#each activities as activity}
			<div class="rounded-lg border border-border bg-card p-4">
				<p class="text-sm">{JSON.stringify(activity)}</p>
			</div>
		{/each}
	</div>
{:else}
	<div
		class="rounded-lg border border-dashed border-muted-foreground/30 bg-muted/50 p-8 text-center"
	>
		<Icon icon="lucide:activity" class="mx-auto mb-3 size-12 text-muted-foreground/50" />
		<p class="text-muted-foreground">No recent activity.</p>
	</div>
{/if}
