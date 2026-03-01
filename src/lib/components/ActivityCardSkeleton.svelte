<script lang="ts">
	let { count = 5 }: { count?: number } = $props();
</script>

{#each Array(count) as _}
	<div class="flex gap-3 rounded-lg border bg-card p-4">
		<!-- Avatar placeholder -->
		<div class="h-10 w-10 shrink-0 animate-pulse rounded-full bg-muted"></div>
		<div class="flex-1 space-y-2 pt-1">
			<div class="h-3 w-1/3 animate-pulse rounded bg-muted"></div>
			<div class="h-3 w-2/3 animate-pulse rounded bg-muted"></div>
			<div class="h-2.5 w-1/4 animate-pulse rounded bg-muted/70"></div>
		</div>
	</div>
{/each}
