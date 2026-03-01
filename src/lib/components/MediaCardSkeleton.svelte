<script lang="ts">
	let { count = 10 }: { count?: number } = $props();
</script>

{#each Array(count) as _}
	<div class="flex flex-col overflow-hidden rounded-lg border bg-card">
		<!-- Cover placeholder -->
		<div class="aspect-2/3 w-full animate-pulse bg-muted"></div>
		<!-- Info placeholder -->
		<div class="space-y-2 p-2">
			<div class="h-3 w-full animate-pulse rounded bg-muted"></div>
			<div class="h-3 w-2/3 animate-pulse rounded bg-muted"></div>
			<div class="h-2.5 w-1/2 animate-pulse rounded bg-muted/70"></div>
		</div>
	</div>
{/each}
