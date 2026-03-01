<script lang="ts">
	/**
	 * Settings panel for media list preferences.
	 * Stored in localStorage (no backend config field yet).
	 */
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';
	import { browser } from '$app/environment';
	import { toast } from 'svelte-sonner';

	const STORAGE_KEY = 'zafkiel:list_settings';

	type ViewMode = 'list' | 'grid';
	type SortBy =
		| 'UPDATED_AT_DESC'
		| 'UPDATED_AT'
		| 'ADDED_TIME_DESC'
		| 'ADDED_TIME'
		| 'SCORE_DESC'
		| 'TITLE_ROMAJI'
		| 'PROGRESS_DESC';
	type ScoreFormat = '10' | '100' | 'stars' | 'smiley';

	interface ListSettingsData {
		defaultView: ViewMode;
		defaultSort: SortBy;
		scoreFormat: ScoreFormat;
		showPrivate: boolean;
		showAdult: boolean;
	}

	function loadSettings(): ListSettingsData {
		if (!browser) return defaults();
		try {
			const raw = localStorage.getItem(STORAGE_KEY);
			if (raw) return { ...defaults(), ...JSON.parse(raw) };
		} catch {}
		return defaults();
	}

	function defaults(): ListSettingsData {
		return {
			defaultView: 'list',
			defaultSort: 'UPDATED_AT_DESC',
			scoreFormat: '10',
			showPrivate: false,
			showAdult: false,
		};
	}

	let settings = $state<ListSettingsData>(loadSettings());

	function save() {
		if (browser) {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
		}
		toast.success('List settings saved');
	}

	const VIEW_OPTIONS: { label: string; value: ViewMode; icon: string }[] = [
		{ label: 'List', value: 'list', icon: 'solar:list-bold-duotone' },
		{ label: 'Grid', value: 'grid', icon: 'solar:widget-2-bold-duotone' },
	];

	const SORT_OPTIONS: { label: string; value: SortBy }[] = [
		{ label: 'Last Updated', value: 'UPDATED_AT_DESC' },
		{ label: 'Oldest Updated', value: 'UPDATED_AT' },
		{ label: 'Last Added', value: 'ADDED_TIME_DESC' },
		{ label: 'First Added', value: 'ADDED_TIME' },
		{ label: 'Score (High → Low)', value: 'SCORE_DESC' },
		{ label: 'Title (A–Z)', value: 'TITLE_ROMAJI' },
		{ label: 'Progress (High → Low)', value: 'PROGRESS_DESC' },
	];

	const SCORE_OPTIONS: { label: string; value: ScoreFormat; emoji: string }[] = [
		{ label: '10 Point', value: '10', emoji: '🔢' },
		{ label: '100 Point', value: '100', emoji: '💯' },
		{ label: 'Stars (1–5)', value: 'stars', emoji: '⭐' },
		{ label: 'Smiley', value: 'smiley', emoji: '😊' },
	];
</script>

<Card>
	<CardHeader>
		<CardTitle class="flex items-center gap-2">
			<Icon icon="solar:list-heart-bold-duotone" class="size-5 text-primary" />
			List Settings
		</CardTitle>
		<CardDescription>
			Customize how your anime and manga lists are displayed and sorted.
		</CardDescription>
	</CardHeader>
	<CardContent class="space-y-6">
		<!-- Default View -->
		<div>
			<Label class="mb-2 block text-sm font-medium">Default View</Label>
			<div class="flex gap-2">
				{#each VIEW_OPTIONS as opt}
					<button
						onclick={() => (settings.defaultView = opt.value)}
						class="flex flex-1 items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-colors
							{settings.defaultView === opt.value
							? 'border-primary bg-primary/10 text-primary'
							: 'bg-card hover:bg-muted'}"
					>
						<Icon icon={opt.icon} class="size-4" />
						{opt.label}
					</button>
				{/each}
			</div>
		</div>

		<!-- Default Sort -->
		<div>
			<Label for="list-sort" class="mb-2 block text-sm font-medium">Default Sort</Label>
			<select
				id="list-sort"
				bind:value={settings.defaultSort}
				class="w-full rounded-md border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
			>
				{#each SORT_OPTIONS as opt}
					<option value={opt.value}>{opt.label}</option>
				{/each}
			</select>
		</div>

		<!-- Score Format -->
		<div>
			<Label class="mb-2 block text-sm font-medium">Score Format</Label>
			<div class="grid grid-cols-2 gap-2">
				{#each SCORE_OPTIONS as opt}
					<button
						onclick={() => (settings.scoreFormat = opt.value)}
						class="flex items-center gap-2 rounded-lg border px-3 py-2 text-sm font-medium transition-colors
							{settings.scoreFormat === opt.value
							? 'border-primary bg-primary/10 text-primary'
							: 'bg-card hover:bg-muted'}"
					>
						<span>{opt.emoji}</span>
						{opt.label}
					</button>
				{/each}
			</div>
		</div>

		<!-- Toggles -->
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<div>
					<Label class="text-sm font-medium">Show Private Entries</Label>
					<p class="text-xs text-muted-foreground">Include private entries in your list view</p>
				</div>
				<Switch
					checked={settings.showPrivate}
					onCheckedChange={(v) => (settings.showPrivate = v)}
				/>
			</div>

			<div class="flex items-center justify-between">
				<div>
					<Label class="text-sm font-medium">Show Adult Content</Label>
					<p class="text-xs text-muted-foreground">Display 18+ media in lists (requires AniList setting)</p>
				</div>
				<Switch
					checked={settings.showAdult}
					onCheckedChange={(v) => (settings.showAdult = v)}
				/>
			</div>
		</div>

		<!-- Save -->
		<Button onclick={save} class="w-full">
			<Icon icon="solar:diskette-bold-duotone" class="mr-2 size-4" />
			Save List Settings
		</Button>
	</CardContent>
</Card>
