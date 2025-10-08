<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetTrigger } from '$lib/components/ui/sheet';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';

	let open = $state(false);
	let switchingTheme = $state(false);

	async function handleThemeSwitch(themeId: string): Promise<void> {
		if (switchingTheme) return;

		const isLoaded = themeStore.loadedThemes.has(themeId);

		// If theme is not loaded, load it first
		if (!isLoaded) {
			switchingTheme = true;
			try {
				await themeStore.loadTheme(themeId);
				toast.success(`Loaded ${themeId} theme`);
			} catch (error) {
				toast.error('Failed to load theme');
				switchingTheme = false;
				return;
			}
		}

		// Now switch to it if it's not already active
		if (themeStore.currentTheme !== themeId) {
			try {
				await themeStore.switchTheme(themeId);
				toast.success(`Switched to ${themeId} theme`);
			} catch (error) {
				toast.error('Failed to switch theme');
			}
		}

		switchingTheme = false;
		// open = false; // Close the sheet after switching
	}
</script>

<Sheet bind:open>
	<SheetTrigger>
		{#snippet child({ props })}
			<Button
				variant="outline"
				size="icon"
				class="rounded-xl transition-all duration-300 hover:scale-105 hover:shadow-lg"
				{...props}
			>
				<Icon icon="solar:pallete-2-bold" class="h-5 w-5" />
			</Button>
		{/snippet}
	</SheetTrigger>
	<SheetContent side="right" class="w-full sm:max-w-xs overflow-y-auto gap-0">
		<SheetHeader>
			<SheetTitle class="flex items-center gap-2 text-xl">
				<Icon icon="solar:pallete-2-bold" class="h-6 w-6 text-primary" />
				Theme Selector
			</SheetTitle>
		</SheetHeader>

		<div class="px-4">
			<div class="flex flex-col gap-4">
				{#each themeStore.availableThemes as theme}
					{@const isActive = themeStore.currentTheme === theme.id}
					{@const isLoaded = themeStore.loadedThemes.has(theme.id)}

					<button
						onclick={() => handleThemeSwitch(theme.id)}
						disabled={switchingTheme}
						data-theme={theme.id}
						class={`group relative flex w-full flex-col gap-3 rounded-lg  hover:scale-[1.025] border p-4 text-left transition-all bg-background/80
							${isActive
							? 'border-primary/60 bg-primary/5'
							: 'border-border hover:border-border/80 hover:bg-background'}
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
								<div class="h-8 w-full rounded-md bg-primary/20 flex items-center justify-center italic text-xs animate-pulse">
									Click to Load
								</div>
							</div>
						{/if}

						<!-- Active Indicator with Checkmark -->
						{#if isActive}
							<div
								class="absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full bg-primary shadow-lg"
							>
								<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary-foreground" />
							</div>
						{/if}

						<!-- Loading Indicator -->
						{#if !isLoaded && !isActive}
							<div
								class="absolute -top-2 -right-2 flex h-6 w-6 items-center justify-center rounded-full bg-muted"
							>
								<Icon icon="solar:download-minimalistic-bold" class="h-3.5 w-3.5 text-muted-foreground" />
							</div>
						{/if}
					</button>
				{/each}
			</div>
		</div>

		<Separator class="my-6" />

		<div class="m-3 mt-0 rounded-xl border border-border/50 bg-foreground/5 p-4">
			<div class="flex items-center gap-2">
				<Icon icon="solar:lightbulb-bolt-bold" class="h-5 w-5 text-primary" />
				<span class="text-sm font-medium">Pro Tip</span>
			</div>
			<p class="text-xs text-foreground/70 leading-relaxed">
				Themes are loaded on-demand to improve performance. Unloaded themes show a download icon. Once loaded, they're cached for instant switching.
			</p>
		</div>
	</SheetContent>
</Sheet>
