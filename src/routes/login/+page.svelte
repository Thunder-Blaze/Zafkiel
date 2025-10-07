<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import Icon from '@iconify/svelte';

	let isLoggingIn = $state(false);
	let errorMessage = $state<string | null>(null);

	// Redirect if already authenticated
	$effect(() => {
		if (browser && $isAuthenticated) {
			goto('/');
		}
	});

	async function handleLogin() {
		isLoggingIn = true;
		errorMessage = null;

		try {
			await authStore.login();
			// Redirect happens via the effect above
		} catch (error) {
			errorMessage = error instanceof Error ? error.message : 'Failed to login';
			console.error('[Login] Error:', error);
		} finally {
			isLoggingIn = false;
		}
	}
</script>

<div
	class="flex min-h-screen items-center justify-center bg-gradient-to-br from-background via-background to-muted/20 p-4"
>
	<div class="w-full max-w-md space-y-8">
		<!-- Logo/Brand Section -->
		<div class="text-center">
			<div
				class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/10 ring-2 ring-primary/20"
			>
				<Icon icon="solar:play-circle-bold" class="h-10 w-10 text-primary" />
			</div>
			<h1 class="text-4xl font-bold tracking-tight">Zafkiel</h1>
			<p class="mt-2 text-muted-foreground">Your gateway to the anime universe</p>
		</div>

		<!-- Login Card -->
		<Card class="border-2 shadow-2xl">
			<CardHeader class="space-y-1 text-center">
				<CardTitle class="text-2xl">Welcome Back</CardTitle>
				<CardDescription>Sign in with your AniList account to continue</CardDescription>
			</CardHeader>
			<CardContent class="space-y-6">
				<!-- Login Button -->
				<Button
					class="h-12 w-full text-lg font-semibold"
					size="lg"
					onclick={handleLogin}
					disabled={isLoggingIn}
				>
					{#if isLoggingIn}
						<Icon icon="solar:refresh-circle-line-duotone" class="mr-2 h-5 w-5 animate-spin" />
						Connecting...
					{:else}
						<Icon icon="solar:login-3-bold" class="mr-2 h-5 w-5" />
						Sign in with AniList
					{/if}
				</Button>

				<!-- Error Message -->
				{#if errorMessage}
					<div
						class="flex items-start gap-2 rounded-lg border border-destructive/20 bg-destructive/10 p-4 text-sm text-destructive"
					>
						<Icon icon="solar:danger-circle-bold" class="mt-0.5 h-5 w-5 flex-shrink-0" />
						<p>{errorMessage}</p>
					</div>
				{/if}

				<!-- Info Section -->
				<div class="space-y-4 border-t pt-4">
					<h3 class="text-sm font-semibold text-muted-foreground">What you'll get:</h3>
					<ul class="space-y-3">
						<li class="flex items-start gap-3">
							<div class="mt-0.5 rounded-full bg-primary/10 p-1">
								<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
							</div>
							<div class="text-sm">
								<p class="font-medium">Sync your anime list</p>
								<p class="text-xs text-muted-foreground">Keep track of what you're watching</p>
							</div>
						</li>
						<li class="flex items-start gap-3">
							<div class="mt-0.5 rounded-full bg-primary/10 p-1">
								<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
							</div>
							<div class="text-sm">
								<p class="font-medium">Personalized recommendations</p>
								<p class="text-xs text-muted-foreground">Discover new anime based on your taste</p>
							</div>
						</li>
						<li class="flex items-start gap-3">
							<div class="mt-0.5 rounded-full bg-primary/10 p-1">
								<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
							</div>
							<div class="text-sm">
								<p class="font-medium">Cross-device sync</p>
								<p class="text-xs text-muted-foreground">Access your data anywhere</p>
							</div>
						</li>
					</ul>
				</div>

				<!-- Privacy Note -->
				<div class="rounded-lg bg-muted/50 p-3 text-center text-xs text-muted-foreground">
					<Icon icon="solar:shield-check-bold" class="mr-1 inline-block h-4 w-4" />
					We only access public data and your anime list. Your credentials are never stored.
				</div>
			</CardContent>
		</Card>

		<!-- Footer Links -->
		<div class="text-center text-sm text-muted-foreground">
			Don't have an AniList account?
			<a
				href="https://anilist.co/signup"
				target="_blank"
				rel="noopener noreferrer"
				class="font-medium text-primary hover:underline"
			>
				Create one here
			</a>
		</div>
	</div>
</div>
