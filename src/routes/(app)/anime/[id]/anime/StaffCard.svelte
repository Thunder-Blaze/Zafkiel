<script lang="ts">
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';

	export interface Staff {
		id: number;
		name: {
			first?: string;
			last?: string;
			full?: string;
			native?: string;
			userPreferred?: string;
		};
		image: {
			large: string;
			medium: string;
		};
		role: string;
	}

	let { staff }: { staff: Staff } = $props();
</script>

<div
	class="flex w-full items-center justify-between rounded-xl border border-border/60 bg-card/40 p-1.5 shadow-sm transition-all hover:bg-card/60 hover:shadow-md"
>
	<a
		href="/staff/{staff.id}"
		class="group flex flex-1 items-center gap-3 overflow-hidden rounded-lg bg-muted/40 pr-3 transition-colors hover:bg-muted/60"
	>
		<CachedImage
			src={staff.image.large || staff.image.medium}
			alt={staff.name.full || 'Staff'}
			class="h-20 w-[60px] shrink-0 object-cover"
		/>
		<div class="flex min-w-0 flex-col overflow-hidden py-1">
			<span
				class="mb-0.5 block truncate text-[10px] font-bold tracking-wider text-muted-foreground uppercase"
			>
				{staff.role || 'STAFF'}
			</span>
			<span
				class="mb-1 truncate text-sm leading-none font-semibold text-foreground transition-colors group-hover:text-primary"
			>
				{staff.name.full}
			</span>
			{#if staff.name.native}
				<span class="truncate text-[10px] text-muted-foreground/60">{staff.name.native}</span>
			{/if}
		</div>
	</a>
</div>
