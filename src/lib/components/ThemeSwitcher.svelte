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
		open = false; // Close the sheet after switching
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
	<SheetContent side="right" class="w-full sm:max-w-lg overflow-y-auto">
		<SheetHeader>
			<SheetTitle class="flex items-center gap-2 text-xl">
				<Icon icon="solar:pallete-2-bold" class="h-6 w-6 text-primary" />
				Theme Selector
			</SheetTitle>
		</SheetHeader>
		
		<Separator class="my-4" />

		<div class="space-y-4">
			<Label class="text-base font-medium">Color Theme</Label>
			<div class="grid grid-cols-2 gap-4">
				{#each themeStore.availableThemes as theme}
					{@const isActive = themeStore.currentTheme === theme.id}
					{@const isLoaded = themeStore.loadedThemes.has(theme.id)}

					<button
						onclick={() => handleThemeSwitch(theme.id)}
						disabled={switchingTheme}
						class="group relative flex flex-col gap-3 rounded-xl border-2 p-4 transition-all duration-300 ease-out
							{isActive
							? 'scale-[1.02] border-primary/60 bg-primary/10 shadow-xl ring-2 ring-primary/20'
							: isLoaded
							? 'border-border hover:scale-[1.02] hover:border-primary/30 hover:bg-foreground/5 hover:shadow-lg'
							: 'border-border/50 opacity-60 hover:opacity-100'}
							{switchingTheme ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}"
					>
						<!-- Color Preview -->
						<div class="flex h-10 gap-1.5 overflow-hidden rounded-lg border border-border/50" data-theme={theme.id}>
							<div
								class="flex-1 rounded-md bg-primary transition-all duration-300 ease-out group-hover:scale-105 group-hover:shadow-md"
							></div>
							<div
								class="flex-1 rounded-md bg-background border border-border/50 transition-all duration-300 ease-out group-hover:scale-105 group-hover:shadow-md"
							></div>
							<div
								class="flex-1 rounded-md bg-accent transition-all duration-300 ease-out group-hover:scale-105 group-hover:shadow-md"
							></div>
						</div>

						<!-- Theme Name -->
						<div class="text-center transition-transform duration-200 group-hover:translate-y-[-1px]">
							<span class="text-sm font-medium text-foreground">{theme.name}</span>
							{#if !isLoaded}
								<p class="mt-1 text-xs text-foreground/50">Click to load</p>
							{/if}
						</div>

						<!-- Active Indicator with Checkmark -->
						{#if isActive}
							<div
								class="absolute -top-2 -right-2 flex h-7 w-7 items-center justify-center rounded-full bg-primary/90 shadow-lg transition-transform duration-300 ease-out"
							>
								<Icon icon="solar:check-circle-bold" class="h-5 w-5 text-background" />
							</div>
						{/if}

						<!-- Loading Indicator -->
						{#if !isLoaded && !isActive}
							<div
								class="absolute -top-2 -right-2 flex h-7 w-7 items-center justify-center rounded-full bg-foreground/10 backdrop-blur-sm"
							>
								<Icon icon="solar:download-minimalistic-bold" class="h-4 w-4 text-foreground/50" />
							</div>
						{/if}

						<!-- Hover Overlay Effect -->
						<div
							class="pointer-events-none absolute inset-0 rounded-xl bg-gradient-to-br from-transparent via-transparent to-foreground/5 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
						></div>
					</button>
				{/each}
			</div>
		</div>

		<Separator class="my-6" />

		<div class="space-y-3 rounded-xl border border-border/50 bg-foreground/5 p-4">
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
