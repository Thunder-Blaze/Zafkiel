<script lang="ts">
	import { goto } from '$app/navigation';
	import { Card } from '$lib/components/ui/card';
	import Icon from '@iconify/svelte';
	import type { User } from '$lib/types/anilist';

	let { currentUser } = $props<{ currentUser: User | null }>();
</script>

<Card class="border-border/50 bg-card/70 p-5 backdrop-blur-md">
	<div class="flex flex-col items-center gap-2.5">
		<div
			class="flex size-30 shrink-0 items-center justify-center overflow-hidden rounded-full bg-gradient-to-br from-primary to-primary/70 shadow-lg ring-4 ring-foreground/15"
		>
			{#if currentUser?.avatar?.large}
				<img
					src={currentUser.avatar.large}
					alt={currentUser?.name}
					class="h-full w-full object-cover"
				/>
			{:else}
				<Icon icon="solar:user-bold" class="h-6 w-6 text-primary-foreground" />
			{/if}
		</div>
		<div class="flex min-w-0 flex-1 flex-col items-center gap-1 text-center">
			<h3 class="truncate text-lg leading-tight font-semibold">@{currentUser?.name || 'User'}</h3>

			<div class="mt-2 flex w-full items-center justify-center gap-4 text-sm text-foreground/80">
				<div class="flex flex-col items-center">
					<span class="font-bold text-primary">{currentUser?.statistics?.anime?.count || 0}</span>
					<span class="text-xs tracking-wider text-muted-foreground uppercase">Anime</span>
				</div>
				<div class="h-6 w-px bg-border/50"></div>
				<div class="flex flex-col items-center">
					<span class="font-bold text-primary">{currentUser?.statistics?.manga?.count || 0}</span>
					<span class="text-xs tracking-wider text-muted-foreground uppercase">Manga</span>
				</div>
			</div>
		</div>
	</div>
</Card>
