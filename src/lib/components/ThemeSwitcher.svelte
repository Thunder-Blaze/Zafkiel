<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import { themeManager } from '$lib/services/theme';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '@iconify/svelte';

	const currentTheme = $derived(themeStore.currentTheme);
	const availableThemes = $derived(themeStore.availableThemes);
	const isDark = $derived(themeStore.isDark);
	const isLoading = $derived(themeStore.isLoading);

	let loadingThemeId = $state<string | null>(null);

	async function handleThemeClick(themeId: string) {
		if (currentTheme === themeId) return;

		if (!themeManager.isThemeLoaded(themeId)) {
			loadingThemeId = themeId;
			try {
				await themeManager.loadTheme(themeId);
			} finally {
				loadingThemeId = null;
			}
		} else {
			await themeStore.switchTheme(themeId);
		}
	}

	function toggleDarkMode() {
		themeStore.toggleDarkMode();
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button variant="outline" size="icon" disabled={isLoading} {...props}>
				{#if isLoading}
					<Icon icon="solar:refresh-circle-line-duotone" class="h-5 w-5 animate-spin" />
				{:else}
					<Icon icon="solar:pallete-2-bold-duotone" class="h-5 w-5" />
				{/if}
				<span class="sr-only">Theme Switcher</span>
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-56 max-h-[400px] overflow-y-auto">
		<DropdownMenu.Label>Themes</DropdownMenu.Label>
		<DropdownMenu.Separator />

		<DropdownMenu.Item onclick={toggleDarkMode} class="flex items-center justify-between">
			<span>Dark Mode</span>
			{#if isDark}
				<Icon icon="solar:moon-bold" class="h-4 w-4" />
			{:else}
				<Icon icon="solar:sun-bold" class="h-4 w-4" />
			{/if}
		</DropdownMenu.Item>

		<DropdownMenu.Separator />

		<DropdownMenu.Group>
			{#if availableThemes.length === 0}
				<DropdownMenu.Item disabled class="text-muted-foreground">
					<Icon icon="solar:refresh-circle-line-duotone" class="h-4 w-4 animate-spin" />
					Loading themes...
				</DropdownMenu.Item>
			{:else}
				{#each availableThemes as theme}
					{@const isLoaded = themeManager.isThemeLoaded(theme.id)}
					{@const isCurrentTheme = currentTheme === theme.id}
					{@const isLoadingThis = loadingThemeId === theme.id}

					<DropdownMenu.Item
						onclick={() => handleThemeClick(theme.id)}
						class="flex items-center gap-3 cursor-pointer"
					>
						<div class="flex gap-0.5">
							{#if isLoaded}
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: hsl(var(--primary));"
								></div>
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: hsl(var(--background));"
								></div>
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: hsl(var(--accent));"
								></div>
							{:else}
								<div class="h-3 w-3 rounded-full border bg-muted"></div>
								<div class="h-3 w-3 rounded-full border bg-muted"></div>
								<div class="h-3 w-3 rounded-full border bg-muted"></div>
							{/if}
						</div>

						<div class="flex-1 flex flex-col">
							<span class="text-sm font-medium">{theme.name}</span>
							{#if !isLoaded && !isCurrentTheme}
								<span class="text-xs text-muted-foreground">Click to load</span>
							{:else if isLoadingThis}
								<span class="text-xs text-muted-foreground">Loading...</span>
							{:else if isLoaded && !isCurrentTheme}
								<span class="text-xs text-muted-foreground">Click to switch</span>
							{/if}
						</div>

						{#if isCurrentTheme}
							<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
						{:else if isLoadingThis}
							<Icon icon="solar:refresh-circle-line-duotone" class="h-4 w-4 animate-spin" />
						{/if}
					</DropdownMenu.Item>
				{/each}
			{/if}
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
