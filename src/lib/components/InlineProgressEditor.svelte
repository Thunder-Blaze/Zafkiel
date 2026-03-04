<script lang="ts">
	/**
	 * Inline progress editor: shows "X / Y", click → editable input.
	 * +1 button for quick increment. Saves via useUpdateListProgress.
	 * Optimistic update: updates display immediately, syncs on blur/enter.
	 */
	import { useUpdateListProgress } from '$lib/hooks/useAnilist.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';

	let {
		entryId,
		progress,
		total,
		onUpdate,
		class: className = '',
	}: {
		entryId: number;
		progress: number;
		total?: number | null;
		/** Callback after a successful save */
		onUpdate?: (newProgress: number) => void;
		class?: string;
	} = $props();

	const mutation = useUpdateListProgress();

	let editing = $state(false);
	let inputValue = $state('');
	let optimisticProgress = $state(0);
	let inputEl: HTMLInputElement | null = $state(null);

	// Keep optimisticProgress in sync when the prop changes externally
	$effect(() => {
		optimisticProgress = progress;
		inputValue = String(progress);
	});

	const isComplete = $derived(total != null && optimisticProgress >= total);
	const totalLabel = $derived(total != null ? String(total) : '?');

	async function save(newProgress: number) {
		if (newProgress === progress) {
			editing = false;
			return;
		}
		const clamped = total != null ? Math.min(newProgress, total) : newProgress;
		const prev = optimisticProgress;
		optimisticProgress = clamped; // optimistic
		editing = false;
		try {
			await mutation.mutateAsync({ entryId, progress: clamped });
			onUpdate?.(clamped);
		} catch {
			optimisticProgress = prev;
			toast.error('Failed to update progress');
		}
	}

	function startEdit() {
		inputValue = String(optimisticProgress);
		editing = true;
		// Focus after Svelte renders the input
		setTimeout(() => inputEl?.focus(), 0);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			const v = parseInt(inputValue, 10);
			if (!isNaN(v) && v >= 0) save(v);
			else editing = false;
		} else if (e.key === 'Escape') {
			editing = false;
		}
	}

	function handleBlur() {
		const v = parseInt(inputValue, 10);
		if (!isNaN(v) && v >= 0) save(v);
		else editing = false;
	}

	async function increment() {
		const next = optimisticProgress + 1;
		await save(next);
	}
</script>

<div class="flex items-center gap-1 {className}">
	{#if editing}
		<input
			bind:this={inputEl}
			bind:value={inputValue}
			type="number"
			min="0"
			max={total ?? undefined}
			class="w-12 rounded border bg-background px-1 py-0.5 text-center text-sm focus:ring-1 focus:ring-ring focus:outline-none"
			onkeydown={handleKeydown}
			onblur={handleBlur}
		/>
		<span class="text-xs text-muted-foreground">/ {totalLabel}</span>
	{:else}
		<button
			class="cursor-pointer rounded px-1 py-0.5 text-sm tabular-nums hover:bg-muted/60 focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none"
			onclick={startEdit}
			title="Click to edit progress"
		>
			{optimisticProgress} / {totalLabel}
		</button>
	{/if}

	{#if !isComplete}
		<button
			class="flex size-5 cursor-pointer items-center justify-center rounded border bg-card text-xs text-muted-foreground transition-colors hover:border-primary hover:text-primary disabled:cursor-not-allowed disabled:opacity-50"
			onclick={increment}
			disabled={mutation.isPending}
			title="+1 episode"
		>
			{#if mutation.isPending}
				<Icon icon="solar:refresh-circle-line-duotone" class="size-3 animate-spin" />
			{:else}
				<Icon icon="solar:add-circle-linear" class="size-3" />
			{/if}
		</button>
	{:else}
		<Icon icon="solar:check-circle-bold-duotone" class="size-4 text-green-500" />
	{/if}
</div>
