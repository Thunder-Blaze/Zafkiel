<script>
	import { goto } from '$app/navigation';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	let { avatar, name } = $props();
</script>

<Card class="p-5 border-border/50 bg-card/70 backdrop-blur-md">
	<div class="flex flex-col items-center gap-2.5">
		<div class="flex size-40 items-center justify-center overflow-hidden rounded-full bg-gradient-to-br from-primary to-primary/70 shadow-lg shrink-0 ring-4 ring-foreground/15">
			{#if avatar}
					<img src={avatar} alt={name} class="h-full w-full object-cover" />
			{:else}
				<Icon icon="solar:user-bold" class="h-6 w-6 text-primary-foreground" />
			{/if}
		</div>
		<div class="flex-1 min-w-0 gap-1 flex flex-col items-center text-left">
			<h3 class="font-semibold text-lg truncate leading-tight">@{name}</h3>
			<Button variant="outline" size="sm" class="mt-1 h-5 text-[10px] px-2" onclick={() => goto('/profile')}>
				View Profile
			</Button>
		</div>
	</div>
</Card>
