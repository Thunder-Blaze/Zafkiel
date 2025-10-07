<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '@iconify/svelte';

	// Reactive state from theme store
	const currentTheme = $derived(themeStore.currentTheme);
	const availableThemes = $derived(themeStore.availableThemes);
	const isDark = $derived(themeStore.isDark);
	const isLoading = $derived(themeStore.isLoading);

	// Local loading state to prevent rapid switching
	let isSwitching = $state(false);

	// Debug logging
	$effect(() => {
		console.log('[ThemeSwitcher] State updated:', {
			currentTheme,
			availableThemesCount: availableThemes.length,
			isDark,
			isLoading,
			isSwitching,
		});
	});

	// Get current theme metadata
	const currentThemeData = $derived(availableThemes.find((t) => t.id === currentTheme));

	async function handleThemeChange(themeId: string) {
		if (isSwitching || currentTheme === themeId) return;

		isSwitching = true;
		try {
			await themeStore.switchTheme(themeId);
		} finally {
			// Small delay to prevent rapid switching
			setTimeout(() => {
				isSwitching = false;
			}, 300);
		}
	}

	function toggleDarkMode() {
		themeStore.setDarkMode(!isDark);
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button variant="outline" size="icon" disabled={isLoading || isSwitching} {...props}>
				{#if isLoading || isSwitching}
					<Icon icon="solar:refresh-circle-line-duotone" class="h-5 w-5 animate-spin" />
				{:else}
					<Icon icon="solar:pallete-2-bold-duotone" class="h-5 w-5" />
				{/if}
				<span class="sr-only">Theme Switcher</span>
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end" class="w-56">
		<DropdownMenu.Label>Theme</DropdownMenu.Label>
		<DropdownMenu.Separator />

		<!-- Theme options -->
		<DropdownMenu.Group>
			{#if availableThemes.length === 0}
				<DropdownMenu.Item disabled class="text-muted-foreground">
					<Icon icon="solar:refresh-circle-line-duotone" class="h-4 w-4 animate-spin" />
					Loading themes...
				</DropdownMenu.Item>
			{:else}
				{#each availableThemes as theme}
					{@const themeColors = isDark ? theme.colors.dark : theme.colors.light}
					<DropdownMenu.Item
						onclick={() => handleThemeChange(theme.id)}
						class="flex items-center gap-2"
					>
						<div class="flex flex-1 items-center gap-2">
							<!-- Color preview dots -->
							<div class="flex gap-0.5">
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: {themeColors.primary};"
								></div>
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: {themeColors.background};"
								></div>
								<div
									class="h-3 w-3 rounded-full border"
									style="background-color: {themeColors.accent};"
								></div>
							</div>
							<span>{theme.name}</span>
						</div>
						{#if currentTheme === theme.id}
							<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
						{/if}
					</DropdownMenu.Item>
				{/each}
			{/if}
		</DropdownMenu.Group>

		<DropdownMenu.Separator />

		<!-- Dark mode toggle -->
		<DropdownMenu.Item onclick={toggleDarkMode} class="flex items-center justify-between">
			<span class="flex items-center gap-2">
				<Icon
					icon={isDark ? 'solar:moon-bold-duotone' : 'solar:sun-bold-duotone'}
					class="h-4 w-4"
				/>
				{isDark ? 'Dark Mode' : 'Light Mode'}
			</span>
			{#if isDark}
				<Icon icon="solar:check-circle-bold" class="h-4 w-4 text-primary" />
			{/if}
		</DropdownMenu.Item>

		<DropdownMenu.Separator />

		<!-- Link to full settings -->
		<DropdownMenu.Item>
			<a href="/settings" class="flex w-full items-center gap-2">
				<Icon icon="solar:settings-bold-duotone" class="h-4 w-4" />
				<span>More Settings</span>
			</a>
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
