<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { authStore, isAuthenticated, currentUser } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';

	let isLoggingOut = $state(false);

	async function handleLogout(): Promise<void> {
		isLoggingOut = true;
		try {
			await authStore.logout();
			goto('/');
		} catch (error) {
			console.error('Logout failed:', error);
		} finally {
			isLoggingOut = false;
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:user-bold" class="h-5 w-5 text-primary" />
				</div>
				<div>
					<CardTitle>AniList Account</CardTitle>
					<CardDescription>Manage your AniList connection</CardDescription>
				</div>
			</div>
		</CardHeader>
		<CardContent class="space-y-6">
			{#if $isAuthenticated && $currentUser}
				<!-- User Info -->
				<div class="flex items-center gap-4 rounded-lg border border-border/50 bg-foreground/5 p-4">
					<div class="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
						{#if $currentUser.avatar?.large}
							<img
								src={$currentUser.avatar.large}
								alt={$currentUser.name}
								class="h-full w-full rounded-full object-cover"
							/>
						{:else}
							<Icon icon="solar:user-bold" class="h-6 w-6 text-primary" />
						{/if}
					</div>
					<div class="flex-1">
						<p class="font-medium text-foreground">{$currentUser.name}</p>
						<p class="text-sm text-foreground/70">Connected to AniList</p>
					</div>
				</div>

				<!-- Account Actions -->
				<div class="flex gap-3">
					<Button
						variant="outline"
						class="flex-1"
						onclick={() => window.open('https://anilist.co/user/' + $currentUser.name, '_blank')}
					>
						<Icon icon="solar:link-bold" class="mr-2 h-4 w-4" />
						View Profile
					</Button>
					<Button variant="outline" class="flex-1" onclick={handleLogout} disabled={isLoggingOut}>
						{#if isLoggingOut}
							<Icon icon="solar:refresh-circle-line-duotone" class="mr-2 h-4 w-4 animate-spin" />
							Logging out...
						{:else}
							<Icon icon="solar:logout-2-bold" class="mr-2 h-4 w-4" />
							Logout
						{/if}
					</Button>
				</div>
			{:else}
				<div class="rounded-lg border border-border/50 bg-foreground/5 p-6 text-center">
					<Icon icon="solar:link-broken-bold" class="mx-auto mb-3 h-12 w-12 text-foreground/50" />
					<p class="mb-2 font-medium text-foreground">Not Connected</p>
					<p class="mb-4 text-sm text-foreground/70">
						Connect your AniList account to sync your watchlist
					</p>
					<Button onclick={() => goto('/auth/login')}>
						<Icon icon="solar:login-2-bold" class="mr-2 h-4 w-4" />
						Connect AniList
					</Button>
				</div>
			{/if}
		</CardContent>
	</Card>
</div>
