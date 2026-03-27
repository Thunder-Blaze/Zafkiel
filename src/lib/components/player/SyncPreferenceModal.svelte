<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import Icon from '@iconify/svelte';

	let { 
		open = $bindable(false), 
		animeTitle,
		onSelect 
	} = $props<{
		open: boolean;
		animeTitle: string;
		onSelect: (mode: 'yes' | 'no' | 'ask') => void;
	}>();

	function handleSelection(mode: 'yes' | 'no' | 'ask') {
		onSelect(mode);
		open = false;
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>AniList Progress Sync</Dialog.Title>
			<Dialog.Description>
				How would you like to track your progress for <strong>{animeTitle}</strong>?
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<Button 
				variant="outline" 
				class="flex h-16 items-center justify-start gap-4 px-4 text-left"
				onclick={() => handleSelection('yes')}
			>
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 text-primary">
					<Icon icon="solar:check-read-linear" class="h-6 w-6" />
				</div>
				<div>
					<div class="font-semibold">Always Sync</div>
					<div class="text-xs text-muted-foreground text-pretty">Automatically update progress at threshold</div>
				</div>
			</Button>

			<Button 
				variant="outline" 
				class="flex h-16 items-center justify-start gap-4 px-4 text-left"
				onclick={() => handleSelection('ask')}
			>
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-amber-500/10 text-amber-500">
					<Icon icon="solar:question-square-linear" class="h-6 w-6" />
				</div>
				<div>
					<div class="font-semibold">Ask for Each Episode</div>
					<div class="text-xs text-muted-foreground text-pretty">Show a prompt when you finish an episode</div>
				</div>
			</Button>

			<Button 
				variant="outline" 
				class="flex h-16 items-center justify-start gap-4 px-4 text-left"
				onclick={() => handleSelection('no')}
			>
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted text-muted-foreground">
					<Icon icon="solar:close-circle-linear" class="h-6 w-6" />
				</div>
				<div>
					<div class="font-semibold">Never Sync</div>
					<div class="text-xs text-muted-foreground text-pretty">Don't update AniList for this anime</div>
				</div>
			</Button>
		</div>
	</Dialog.Content>
</Dialog.Root>
