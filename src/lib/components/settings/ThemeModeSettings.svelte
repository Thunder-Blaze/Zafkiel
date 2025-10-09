<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';

	async function handleThemeModeChange(mode: 'light' | 'dark' | 'system'): Promise<void> {
		try {
			await themeStore.setThemeMode(mode);
			const modeText = mode === 'system' ? 'system preference' : `${mode} mode`;
			toast.success(`Theme mode set to ${modeText}`);
		} catch (error) {
			toast.error('Failed to update theme mode');
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:monitor-bold" class="h-5 w-5 text-primary" />
				</div>
				<div>
					<CardTitle>Theme Mode</CardTitle>
					<CardDescription>Choose your preferred color scheme</CardDescription>
				</div>
			</div>
		</CardHeader>
		<CardContent class="space-y-6">
			<!-- Theme Mode -->
			<div class="flex gap-2">
				<Button
					variant={themeStore.themeMode === 'light' ? 'default' : 'outline'}
					size="sm"
					class="flex-1 gap-2"
					onclick={() => handleThemeModeChange('light')}
				>
					<Icon icon="ph:sun-bold" class="h-4 w-4" />
					Light
				</Button>
				<Button
					variant={themeStore.themeMode === 'dark' ? 'default' : 'outline'}
					size="sm"
					class="flex-1 gap-2"
					onclick={() => handleThemeModeChange('dark')}
				>
					<Icon icon="ph:moon-bold" class="h-4 w-4" />
					Dark
				</Button>
				<Button
					variant={themeStore.themeMode === 'system' ? 'default' : 'outline'}
					size="sm"
					class="flex-1 gap-2"
					onclick={() => handleThemeModeChange('system')}
				>
					<Icon icon="ph:monitor-bold" class="h-4 w-4" />
					System
				</Button>
			</div>
		</CardContent>
	</Card>
</div>
