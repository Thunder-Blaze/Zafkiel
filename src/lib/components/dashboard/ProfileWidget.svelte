<script>
	import { goto } from '$app/navigation';
	import { Card } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	let { avatar, name } = $props();
</script>

<Card class="border-border/50 bg-card/70 p-5 backdrop-blur-md">
	<div class="flex flex-col items-center gap-2.5">
		<div
			class="flex size-40 shrink-0 items-center justify-center overflow-hidden rounded-full bg-gradient-to-br from-primary to-primary/70 shadow-lg ring-4 ring-foreground/15"
		>
			{#if avatar}
				<img src={avatar} alt={name} class="h-full w-full object-cover" />
			{:else}
				<Icon icon="solar:user-bold" class="h-6 w-6 text-primary-foreground" />
			{/if}
		</div>
		<div class="flex min-w-0 flex-1 flex-col items-center gap-1 text-left">
			<h3 class="truncate text-lg leading-tight font-semibold">@{name}</h3>
			<Button
				variant="outline"
				size="sm"
				class="mt-1 h-5 px-2 text-[10px]"
				onclick={() => goto(`/user/${name}`)}
			>
				View Profile
			</Button>
		</div>
	</div>
</Card>
