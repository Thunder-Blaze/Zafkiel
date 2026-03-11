<script lang="ts">
	import { useThemeState } from '$lib/stores/theme.svelte';
	import { Button } from '$lib/components/ui/button';
	import {
		Sheet,
		SheetContent,
		SheetHeader,
		SheetTitle,
		SheetTrigger,
	} from '$lib/components/ui/sheet';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';

	let open = $state(false);
	let switchingTheme = $state(false);
	let scrollContainerRef: HTMLDivElement | null = $state(null);
	let themeState = useThemeState();

	let themes = $derived.by(() => Array.from(themeState.themes.values()));

	async function handleThemeModeChange(mode: 'light' | 'dark' | 'system'): Promise<void> {
		try {
			themeState.setThemeMode(mode);
			const modeText = mode === 'system' ? 'system preference' : `${mode} mode`;
			toast.success(`Theme mode set to ${modeText}`);
		} catch (error) {
			toast.error('Failed to update theme mode');
		}
	}

	async function handleThemeSwitch(themeId: string): Promise<void> {
		if (switchingTheme) return;
		switchingTheme = true;
		themeState.setTheme(themeState.themes.get(themeId)!);
		switchingTheme = false;
	}

	// Prevent scroll propagation to background
	function handleWheel(e: WheelEvent): void {
		if (!scrollContainerRef) return;

		const { scrollTop, scrollHeight, clientHeight } = scrollContainerRef;
		const isScrollingDown = e.deltaY > 0;
		const isScrollingUp = e.deltaY < 0;

		const isAtTop = scrollTop === 0;
		const isAtBottom = scrollTop + clientHeight >= scrollHeight;

		// Prevent default if we're not at the boundaries or if we're scrolling away from boundaries
		if ((!isAtTop && isScrollingUp) || (!isAtBottom && isScrollingDown)) {
			e.stopPropagation();
		} else if ((isAtTop && isScrollingUp) || (isAtBottom && isScrollingDown)) {
			e.preventDefault();
			e.stopPropagation();
		}
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
	<SheetContent
		side="right"
		class="mt-[3rem] flex h-[calc(100vh-3rem)] w-full flex-col gap-0 sm:max-w-xs"
	>
		<SheetHeader class="flex-shrink-0">
			<SheetTitle class="flex items-center gap-2 text-xl">
				<Icon icon="solar:pallete-2-bold" class="h-6 w-6 text-primary" />
				Theme Selector
			</SheetTitle>
		</SheetHeader>

		<div
			bind:this={scrollContainerRef}
			onwheel={handleWheel}
			class="flex-1 overflow-y-auto px-4 py-2"
			data-lenis-prevent="true"
			style="overscroll-behavior: contain; touch-action: pan-y;"
		>
			<div class="flex flex-col gap-4">
				<!-- Theme Mode Selector -->
				<div class="rounded-lg border border-border/50 bg-muted/20 p-4">
					<Label class="mb-3 block text-sm font-semibold">Color Mode</Label>
					<div class="flex gap-2">
						<Button
							variant={themeState.currentThemeMode === 'light' ? 'default' : 'outline'}
							size="sm"
							class="flex-1 gap-1.5 text-xs"
							onclick={() => handleThemeModeChange('light')}
						>
							<Icon icon="ph:sun-bold" class="h-3.5 w-3.5" />
							Light
						</Button>
						<Button
							variant={themeState.currentThemeMode === 'dark' ? 'default' : 'outline'}
							size="sm"
							class="flex-1 gap-1.5 text-xs"
							onclick={() => handleThemeModeChange('dark')}
						>
							<Icon icon="ph:moon-bold" class="h-3.5 w-3.5" />
							Dark
						</Button>
						<Button
							variant={themeState.currentThemeMode === 'system' ? 'default' : 'outline'}
							size="sm"
							class="flex-1 gap-1.5 text-xs"
							onclick={() => handleThemeModeChange('system')}
						>
							<Icon icon="ph:monitor-bold" class="h-3.5 w-3.5" />
							Auto
						</Button>
					</div>
				</div>

				<Separator class="my-2" />

				<Label class="text-sm font-semibold">Theme Styles</Label>

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

			<Separator class="my-6" />

			<div class="mb-4 rounded-xl border border-border/50 bg-foreground/5 p-4">
				<div class="flex items-center gap-2">
					<Icon icon="solar:lightbulb-bolt-bold" class="h-5 w-5 text-primary" />
					<span class="text-sm font-medium">Pro Tip</span>
				</div>
				<p class="text-xs leading-relaxed text-foreground/70">
					Themes are loaded on-demand to improve performance. Unloaded themes show a download icon.
					Once loaded, they're cached for instant switching.
				</p>
			</div>
		</div>
	</SheetContent>
</Sheet>
