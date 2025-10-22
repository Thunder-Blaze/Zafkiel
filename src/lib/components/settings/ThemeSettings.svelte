<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { useThemeState } from '$lib/stores/theme.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';

	let themeState = useThemeState();

	let themes = $derived(themeState.themes.values());

	let switchingTheme = $state(false);

	async function handleThemeSwitch(themeId: string): Promise<void> {
		if (switchingTheme) return;
		switchingTheme = true;
		themeState.setTheme(themeState.themes.get(themeId)!);
		switchingTheme = false;
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:pallete-2-bold" class="h-5 w-5 text-primary" />
				</div>
				<div>
					<CardTitle>Theme</CardTitle>
					<CardDescription>Choose your color theme</CardDescription>
				</div>
			</div>
		</CardHeader>
		<CardContent class="space-y-4">
			<Separator />

			<!-- Theme Selector -->
			<div class="space-y-3">
				<Label class="text-base font-medium">Color Theme</Label>
				<div class="grid grid-cols-2 gap-3 lg:grid-cols-3">
					{#each themes as theme}
						{@const isActive = themeState.currentThemeId === theme.id}
						{@const isLoaded = themeState.themes.get(theme.id)?.linkElement !== undefined}

						<button
							onclick={() => handleThemeSwitch(theme.id)}
							disabled={switchingTheme}
							data-theme={theme.id}
							class={`group relative flex w-full flex-col gap-3 rounded-lg  border bg-background/80 p-4 text-left transition-all hover:scale-[1.025]
							${
								isActive
									? 'border-primary/60 bg-primary/5'
									: 'border-border hover:border-border/80 hover:bg-background'
							}
							${switchingTheme ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}
							${isLoaded ? '' : 'grayscale'}`}
						>
							<!-- Theme Name -->
							<span class="text-sm font-medium">{theme.name}</span>

							<!-- Color Preview - 4 color swatches in a row -->
							{#if isLoaded}
								<div class="flex gap-3">
									<div class="h-8 w-full rounded-md bg-primary"></div>
									<div class="h-8 w-full rounded-md bg-secondary"></div>
									<div class="h-8 w-full rounded-md bg-accent"></div>
									<div class="h-8 w-full rounded-md bg-foreground"></div>
								</div>
							{:else}
								<div class="flex gap-3">
									<div
										class="flex h-8 w-full items-center justify-center rounded-md bg-primary/20 text-xs italic opacity-70"
									>
										Click to Load
									</div>
								</div>
							{/if}

							<!-- Active Indicator with Checkmark -->
							{#if isActive}
								<div
									class="absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full bg-primary shadow-lg"
								>
									<Icon icon="solar:check-read-broken" class="h-5 w-5 text-primary-foreground" />
								</div>
							{/if}

							<!-- Loading Indicator -->
							{#if !isLoaded && !isActive}
								<div
									class="absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full bg-muted"
								>
									<Icon
										icon="solar:download-minimalistic-bold"
										class="h-3.5 w-3.5 text-muted-foreground"
									/>
								</div>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		</CardContent>
	</Card>
</div>
