<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { configStore, uiConfig, isAuthenticated } from '$lib/stores/config';
	import { ConfigService, type AppConfig, type UiConfig } from '$lib/services/config';
	import { toast } from 'svelte-sonner';

	let tokenInput = $state('');
	let configPath = $state('');
	let isLoading = $state(false);

	// Reactive state from stores
	let config = $state<AppConfig | null>(null);
	let ui = $state<UiConfig | null>(null);
	let authenticated = $state(false);

	// Subscribe to stores
	$effect(() => {
		const unsubConfig = configStore.subscribe((value) => {
			config = value;
		});
		const unsubUi = uiConfig.subscribe((value) => {
			ui = value as UiConfig | null;
		});
		const unsubAuth = isAuthenticated.subscribe((value) => {
			authenticated = value;
		});

		return () => {
			unsubConfig();
			unsubUi();
			unsubAuth();
		};
	});

	onMount(async () => {
		// Initialize config store
		await configStore.init();

		// Get config path
		try {
			configPath = await ConfigService.getConfigPath();
		} catch (error) {
			console.error('Failed to get config path:', error);
		}
	});

	const handleSetToken = async () => {
		if (!tokenInput.trim()) {
			toast.error('Please enter a token');
			return;
		}

		isLoading = true;
		try {
			await configStore.setAniListToken(tokenInput);
			toast.success('Token saved and encrypted successfully!');
			tokenInput = '';
		} catch (error) {
			toast.error(`Failed to set token: ${error}`);
		} finally {
			isLoading = false;
		}
	};

	const handleClearToken = async () => {
		isLoading = true;
		try {
			await configStore.clearAniListToken();
			toast.success('Token cleared successfully!');
		} catch (error) {
			toast.error(`Failed to clear token: ${error}`);
		} finally {
			isLoading = false;
		}
	};

	const handleGetToken = async () => {
		try {
			const token = await ConfigService.getAniListToken();
			if (token) {
				toast.success(`Decrypted token: ${token.substring(0, 20)}...`);
			} else {
				toast.info('No token set');
			}
		} catch (error) {
			toast.error(`Failed to get token: ${error}`);
		}
	};

	const handleThemeChange = async (theme: string) => {
		try {
			await configStore.updateTheme(theme);
			toast.success(`Theme changed to ${theme}`);
		} catch (error) {
			toast.error(`Failed to update theme: ${error}`);
		}
	};
</script>

<div class="container mx-auto p-8 max-w-6xl">
	<div class="mb-8">
		<h1 class="text-4xl font-bold mb-2">Config System Demo</h1>
		<p class="text-muted-foreground">
			Test the encrypted config system with AniList token management
		</p>
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<!-- Authentication Card -->
		<Card>
			<CardHeader>
				<CardTitle>AniList Authentication</CardTitle>
				<CardDescription>
					Token is encrypted using AES-256-GCM
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<!-- Status -->
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium">Status:</span>
					{#if authenticated}
						<Badge class="bg-green-500">Authenticated</Badge>
					{:else}
						<Badge variant="secondary">Not Authenticated</Badge>
					{/if}
				</div>

				<Separator />

				<!-- Token Input -->
				<div class="space-y-2">
					<Label for="token">Access Token</Label>
					<Input
						id="token"
						type="password"
						placeholder="Enter AniList access token"
						bind:value={tokenInput}
						disabled={isLoading}
					/>
				</div>

				<!-- Actions -->
				<div class="flex gap-2">
					<Button
						onclick={handleSetToken}
						disabled={isLoading || !tokenInput.trim()}
						class="flex-1"
					>
						Set Token
					</Button>
					{#if authenticated}
						<Button
							variant="outline"
							onclick={handleGetToken}
							disabled={isLoading}
						>
							View Token
						</Button>
						<Button
							variant="destructive"
							onclick={handleClearToken}
							disabled={isLoading}
						>
							Clear
						</Button>
					{/if}
				</div>
			</CardContent>
		</Card>

		<!-- UI Config Card -->
		<Card>
			<CardHeader>
				<CardTitle>UI Configuration</CardTitle>
				<CardDescription>
					Customize the application appearance
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<!-- Theme Selection -->
				<div class="space-y-2">
					<Label>Theme</Label>
					<div class="flex gap-2">
						<Button
							variant={ui?.theme === 'catppuccin' ? 'default' : 'outline'}
							onclick={() => handleThemeChange('catppuccin')}
							class="flex-1"
						>
							Catppuccin
						</Button>
						<Button
							variant={ui?.theme === 'dark' ? 'default' : 'outline'}
							onclick={() => handleThemeChange('dark')}
							class="flex-1"
						>
							Dark
						</Button>
						<Button
							variant={ui?.theme === 'light' ? 'default' : 'outline'}
							onclick={() => handleThemeChange('light')}
							class="flex-1"
						>
							Light
						</Button>
					</div>
				</div>

				<Separator />

				<!-- Toggle Settings -->
				<div class="space-y-3">
					<div class="flex items-center justify-between">
						<Label for="glow">Glow Effects</Label>
						<Switch
							id="glow"
							checked={ui?.glow_effects ?? false}
							onCheckedChange={(checked) => configStore.updateGlowEffects(checked)}
						/>
					</div>

					<div class="flex items-center justify-between">
						<Label for="animations">Animations</Label>
						<Switch
							id="animations"
							checked={ui?.animations ?? false}
							onCheckedChange={(checked) => configStore.updateAnimations(checked)}
						/>
					</div>

					<div class="flex items-center justify-between">
						<Label for="smooth-scroll">Smooth Scroll</Label>
						<Switch
							id="smooth-scroll"
							checked={ui?.smooth_scroll ?? false}
							onCheckedChange={(checked) => configStore.updateSmoothScroll(checked)}
						/>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Config Info Card -->
		<Card class="md:col-span-2">
			<CardHeader>
				<CardTitle>Configuration Details</CardTitle>
				<CardDescription>
					Current configuration state and file location
				</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<div class="flex items-start gap-2">
						<span class="text-sm font-medium min-w-[120px]">Config Path:</span>
						<code class="text-sm bg-muted px-2 py-1 rounded">{configPath}</code>
					</div>

					<div class="flex items-start gap-2">
						<span class="text-sm font-medium min-w-[120px]">Encryption Key:</span>
						{#if config?.security.encryption_key}
							<code class="text-sm bg-muted px-2 py-1 rounded">
								{config.security.encryption_key.substring(0, 20)}...
							</code>
						{:else}
							<span class="text-sm text-muted-foreground">Not generated yet</span>
						{/if}
					</div>
				</div>

				<Separator />

				<!-- Raw Config Display -->
				<div class="space-y-2">
					<Label>Raw Configuration (RON format)</Label>
					<pre class="bg-muted p-4 rounded-lg overflow-x-auto text-xs">
{JSON.stringify(config, null, 2)}
					</pre>
				</div>
			</CardContent>
		</Card>
	</div>
</div>
