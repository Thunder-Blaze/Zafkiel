<script lang="ts">
	let { count = 10 }: { count?: number } = $props();
</script>

{#each Array(count) as _}
	<div class="flex items-center gap-3 border-b px-3 py-2.5 last:border-b-0">
		<!-- Cover thumbnail -->
		<div class="size-10 shrink-0 animate-pulse rounded bg-muted"></div>
		<!-- Title -->
		<div class="min-w-0 flex-1">
			<div class="mb-1.5 h-3.5 w-2/3 animate-pulse rounded bg-muted"></div>
			<div class="h-2.5 w-1/3 animate-pulse rounded bg-muted/70"></div>
		</div>
		<!-- Status badge -->
		<div class="h-5 w-20 shrink-0 animate-pulse rounded-full bg-muted/70"></div>
		<!-- Score -->
		<div class="h-4 w-8 shrink-0 animate-pulse rounded bg-muted/70"></div>
		<!-- Progress -->
		<div class="h-4 w-14 shrink-0 animate-pulse rounded bg-muted/70"></div>
	</div>
{/each}
