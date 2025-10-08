<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';

	interface Props {
		show18Plus?: boolean;
		showInList?: boolean;
		selectedGenres?: string[];
		onshow18PlusChange?: (value: boolean) => void;
		onshowInListChange?: (value: boolean) => void;
		onselectedGenresChange?: (value: string[]) => void;
	}

	let {
		show18Plus = $bindable(false),
		showInList = $bindable(true),
		selectedGenres = $bindable<string[]>([]),
	}: Props = $props();

	const availableGenres = [
		'Action', 'Adventure', 'Comedy', 'Drama', 'Fantasy', 'Horror',
		'Mecha', 'Music', 'Mystery', 'Psychological', 'Romance', 'Sci-Fi',
		'Slice of Life', 'Sports', 'Supernatural', 'Thriller'
	];

	function toggleGenre(genre: string): void {
		if (selectedGenres.includes(genre)) {
			selectedGenres = selectedGenres.filter((g) => g !== genre);
		} else {
			selectedGenres = [...selectedGenres, genre];
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:filter-bold" class="h-5 w-5 text-primary" />
				</div>
			<div>
				<CardTitle>Content Filters</CardTitle>
				<CardDescription>Manage what content you see</CardDescription>
			</div>
		</div>
	</CardHeader>
	<CardContent class="space-y-6">
		<!-- 18+ Content -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Show 18+ Content</Label>
				<p class="text-sm text-foreground/70">Display adult content in search and browse</p>
			</div>
			<Switch bind:checked={show18Plus} />
		</div>

		<!-- Show in List -->
		<div class="flex items-center justify-between rounded-lg border border-border/50 bg-foreground/5 p-4">
			<div class="space-y-0.5">
				<Label class="text-base font-medium">Show in My List</Label>
				<p class="text-sm text-foreground/70">Display filtered content in your watchlist</p>
			</div>
			<Switch bind:checked={showInList} />
		</div>
	</CardContent>
</Card>
</div>

<!-- Genre Preferences -->
<div transition:slide={{ duration: 300, delay: 50 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
					<Icon icon="solar:star-bold" class="h-5 w-5 text-primary" />
				</div>
			<div>
				<CardTitle>Genre Preferences</CardTitle>
				<CardDescription>Select your favorite genres</CardDescription>
			</div>
		</div>
	</CardHeader>
	<CardContent>
		<div class="flex flex-wrap gap-2">
			{#each availableGenres as genre}
				<button
					onclick={() => toggleGenre(genre)}
					class="rounded-full border-2 px-4 py-2 text-sm font-medium transition-all duration-200 hover:scale-105
						{selectedGenres.includes(genre)
						? 'border-primary/60 bg-primary/10 text-primary shadow-lg'
						: 'border-border/50 bg-foreground/5 text-foreground/70 hover:border-primary/30 hover:bg-foreground/10'}"
				>
					{genre}
				</button>
			{/each}
		</div>
	</CardContent>
</Card>
</div>
