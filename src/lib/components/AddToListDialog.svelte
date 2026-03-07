<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import Icon from '@iconify/svelte';
	import { toast } from 'svelte-sonner';
	import type { Media, MediaListStatus, MediaListEntry, MediaDate } from '$lib/types/anilist';
	import { useSaveListEntry, useDeleteListEntry } from '$lib/hooks/useAnilist.svelte';

	interface Props {
		open: boolean;
		media: Media;
		onClose?: () => void;
	}

	let { open = $bindable(), media, onClose }: Props = $props();

	const saveMutation = useSaveListEntry();
	const deleteEntryMutation = useDeleteListEntry();

	const existing = $derived(media.mediaListEntry as MediaListEntry | undefined);
	const isManga = $derived(media.type === 'MANGA');
	const mediaTitle = $derived(media.title?.userPreferred ?? media.title?.romaji ?? 'Unknown');
	const totalEpisodes = $derived(media.episodes ?? null);
	const totalChapters = $derived(media.chapters ?? null);
	const totalVolumes = $derived((media as unknown as { volumes?: number }).volumes ?? null);

	// ── Form state ────────────────────────────────────────────────────────────
	let selectedStatus = $state<MediaListStatus | null>(null);
	let score = $state(0);
	let progress = $state(0);
	let progressVolumes = $state(0);
	let repeat = $state(0);
	let isPrivate = $state(false);
	let hiddenFromStatus = $state(false);
	let notes = $state('');
	let startedAt = $state('');
	let completedAt = $state('');

	// Seed / reset whenever the dialog opens for a (different) media
	$effect(() => {
		const e = media.mediaListEntry as MediaListEntry | undefined;
		selectedStatus = e?.status ?? null;
		score = e?.score ?? 0;
		progress = e?.progress ?? 0;
		progressVolumes = e?.progressVolumes ?? 0;
		repeat = e?.repeat ?? 0;
		isPrivate = false;
		hiddenFromStatus = e?.hiddenFromStatusLists ?? false;
		notes = e?.notes ?? '';
		startedAt = mediaDateToString(e?.startedAt);
		completedAt = mediaDateToString(e?.finishedAt);
	});

	// ── Helpers ───────────────────────────────────────────────────────────────
	function mediaDateToString(d?: MediaDate): string {
		if (!d?.year) return '';
		return [
			d.year.toString().padStart(4, '0'),
			(d.month ?? 1).toString().padStart(2, '0'),
			(d.day ?? 1).toString().padStart(2, '0'),
		].join('-');
	}

	function stringToFuzzyDate(s: string): { year: number; month: number; day: number } | null {
		if (!s) return null;
		const [y, m, d] = s.split('-').map(Number);
		if (!y) return null;
		return { year: y, month: m ?? 1, day: d ?? 1 };
	}

	const isPending = $derived(saveMutation.isPending || deleteEntryMutation.isPending);

	const statuses = $derived<
		{ value: MediaListStatus; label: string; icon: string; color: string }[]
	>([
		{
			value: 'CURRENT',
			label: isManga ? 'Reading' : 'Watching',
			icon: 'solar:play-circle-bold',
			color: 'bg-green-500/20 text-green-400 border-green-500/40',
		},
		{
			value: 'PLANNING',
			label: 'Planning',
			icon: 'solar:bookmark-bold',
			color: 'bg-blue-500/20  text-blue-400  border-blue-500/40',
		},
		{
			value: 'COMPLETED',
			label: 'Completed',
			icon: 'solar:check-circle-bold',
			color: 'bg-primary/20   text-primary   border-primary/40',
		},
		{
			value: 'PAUSED',
			label: 'Paused',
			icon: 'solar:pause-circle-bold',
			color: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/40',
		},
		{
			value: 'DROPPED',
			label: 'Dropped',
			icon: 'solar:close-circle-bold',
			color: 'bg-red-500/20   text-red-400   border-red-500/40',
		},
		{
			value: 'REPEATING',
			label: isManga ? 'Re-reading' : 'Rewatching',
			icon: 'solar:refresh-circle-bold',
			color: 'bg-purple-500/20 text-purple-400 border-purple-500/40',
		},
	]);

	const scoreLabel = $derived(() => {
		if (score === 0) return 'No score';
		const labels = [
			'',
			'1 – Appalling',
			'2 – Horrible',
			'3 – Very Bad',
			'4 – Bad',
			'5 – Average',
			'6 – Fine',
			'7 – Good',
			'8 – Very Good',
			'9 – Great',
			'10 – Masterpiece',
		];
		return labels[score] ?? `${score}/10`;
	});

	// ── Actions ───────────────────────────────────────────────────────────────
	async function handleSave() {
		if (!selectedStatus) {
			toast.error('Please select a status');
			return;
		}
		try {
			await saveMutation.mutateAsync({
				...(existing?.id ? { id: existing.id } : { media_id: media.id }),
				status: selectedStatus,
				score: score || null,
				progress: progress || null,
				...(isManga && progressVolumes > 0 ? { progress_volumes: progressVolumes } : {}),
				repeat: repeat || null,
				private: isPrivate || null,
				hidden_from_status_lists: hiddenFromStatus || null,
				notes: notes.trim() || null,
				started_at: stringToFuzzyDate(startedAt),
				completed_at: stringToFuzzyDate(completedAt),
			} as Record<string, unknown>);
			toast.success(existing?.id ? 'List entry updated' : `${mediaTitle} added to list`);
			open = false;
			onClose?.();
		} catch {
			toast.error('Failed to save list entry');
		}
	}

	async function handleRemove() {
		if (!existing?.id) return;
		try {
			await deleteEntryMutation.mutateAsync(existing.id);
			toast.success(`Removed ${mediaTitle} from list`);
			open = false;
			onClose?.();
		} catch {
			toast.error('Failed to remove from list');
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-w-lg gap-0 overflow-hidden p-0">
		<!-- Banner / cover header ─────────────────────────────────────────── -->
		<div class="relative h-28 w-full overflow-hidden bg-muted">
			{#if media.bannerImage}
				<CachedImage src={media.bannerImage} alt="" class="h-full w-full object-cover opacity-60" />
			{/if}
			<div
				class="absolute inset-0 bg-linear-to-t from-background via-background/60 to-transparent"
			></div>
			<div class="absolute inset-x-0 bottom-0 flex items-end gap-3 px-5 pb-3">
				<div class="h-16 w-12 shrink-0 overflow-hidden rounded-md shadow-lg ring-2 ring-border">
					<CachedImage
						src={media.coverImage?.large ?? media.coverImage?.medium ?? ''}
						alt={mediaTitle}
						class="h-full w-full object-cover"
					/>
				</div>
				<div class="min-w-0">
					<h2 class="line-clamp-2 text-base leading-snug font-bold">{mediaTitle}</h2>
					<div class="mt-0.5 flex items-center gap-2 text-xs text-muted-foreground">
						{#if media.format}<span>{media.format.replace(/_/g, ' ')}</span>{/if}
						{#if totalEpisodes}<span>·</span><span>{totalEpisodes} ep</span>{/if}
						{#if totalChapters}<span>·</span><span>{totalChapters} ch</span>{/if}
						{#if media.averageScore}
							<span>·</span>
							<span class="flex items-center gap-0.5">
								<Icon icon="solar:star-bold" class="size-3 text-primary" />{media.averageScore}%
							</span>
						{/if}
					</div>
				</div>
			</div>
		</div>

		<!-- Scrollable form body ──────────────────────────────────────────── -->
		<div class="flex max-h-[70vh] flex-col gap-5 overflow-y-auto px-5 py-5">
			<!-- Status ─────────────────────────────────────────────────────── -->
			<div>
				<p class="mb-2 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
					Status
				</p>
				<div class="grid grid-cols-3 gap-1.5">
					{#each statuses as s}
						<button
							type="button"
							onclick={() => (selectedStatus = s.value)}
							class="flex items-center gap-1.5 rounded-md border px-2.5 py-2 text-xs font-medium transition-all hover:scale-[1.02]
								{selectedStatus === s.value
								? s.color
								: 'border-border bg-background text-muted-foreground hover:bg-accent'}"
						>
							<Icon icon={s.icon} class="size-3.5 shrink-0" />{s.label}
						</button>
					{/each}
				</div>
			</div>

			<!-- Score ──────────────────────────────────────────────────────── -->
			<div>
				<div class="mb-2 flex items-center justify-between">
					<p class="text-xs font-semibold tracking-wide text-muted-foreground uppercase">Score</p>
					<span class="text-xs font-medium text-primary">{scoreLabel()}</span>
				</div>
				<div class="flex items-center gap-3">
					<button
						type="button"
						onclick={() => (score = Math.max(0, score - 1))}
						class="flex h-7 w-7 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
						disabled={score <= 0}
					>
						<Icon icon="solar:minus-circle-linear" class="size-4" />
					</button>
					<div class="flex flex-1 items-center gap-1">
						{#each Array.from({ length: 10 }, (_, i) => i + 1) as n}
							<button
								type="button"
								onclick={() => (score = score === n ? 0 : n)}
								class="h-2 flex-1 rounded-full transition-all {n <= score
									? 'bg-primary'
									: 'bg-border hover:bg-muted-foreground/40'}"
								title={n.toString()}
							></button>
						{/each}
					</div>
					<button
						type="button"
						onclick={() => (score = Math.min(10, score + 1))}
						class="flex h-7 w-7 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
						disabled={score >= 10}
					>
						<Icon icon="solar:add-circle-linear" class="size-4" />
					</button>
				</div>
			</div>

			<!-- Progress / Volumes / Rewatches ─────────────────────────────── -->
			<div class="grid {isManga ? 'grid-cols-3' : 'grid-cols-2'} gap-3">
				<!-- Episode / Chapter progress -->
				<div>
					<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						{isManga ? 'Chapter' : 'Episode'}
					</p>
					<div class="flex items-center gap-1">
						<button
							type="button"
							onclick={() => (progress = Math.max(0, progress - 1))}
							class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
							disabled={progress <= 0}
						>
							<Icon icon="solar:minus-circle-linear" class="size-3.5" />
						</button>
						<input
							type="number"
							min="0"
							max={totalEpisodes ?? totalChapters ?? undefined}
							bind:value={progress}
							class="h-8 w-full min-w-0 rounded-md border bg-background px-2 text-center text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						/>
						<button
							type="button"
							onclick={() =>
								(progress = Math.min(totalEpisodes ?? totalChapters ?? Infinity, progress + 1))}
							class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
							disabled={(totalEpisodes ?? totalChapters) != null &&
								progress >= (totalEpisodes ?? totalChapters ?? 0)}
						>
							<Icon icon="solar:add-circle-linear" class="size-3.5" />
						</button>
					</div>
					{#if totalEpisodes ?? totalChapters}
						<p class="mt-1 text-right text-[10px] text-muted-foreground">
							of {totalEpisodes ?? totalChapters}
						</p>
					{/if}
				</div>

				<!-- Volume progress (manga only) -->
				{#if isManga}
					<div>
						<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
							Volume
						</p>
						<div class="flex items-center gap-1">
							<button
								type="button"
								onclick={() => (progressVolumes = Math.max(0, progressVolumes - 1))}
								class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
								disabled={progressVolumes <= 0}
							>
								<Icon icon="solar:minus-circle-linear" class="size-3.5" />
							</button>
							<input
								type="number"
								min="0"
								max={totalVolumes ?? undefined}
								bind:value={progressVolumes}
								class="h-8 w-full min-w-0 rounded-md border bg-background px-2 text-center text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
							/>
							<button
								type="button"
								onclick={() =>
									(progressVolumes = Math.min(totalVolumes ?? Infinity, progressVolumes + 1))}
								class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
								disabled={totalVolumes != null && progressVolumes >= totalVolumes}
							>
								<Icon icon="solar:add-circle-linear" class="size-3.5" />
							</button>
						</div>
						{#if totalVolumes}
							<p class="mt-1 text-right text-[10px] text-muted-foreground">of {totalVolumes}</p>
						{/if}
					</div>
				{/if}

				<!-- Repeat count -->
				<div>
					<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						{isManga ? 'Re-reads' : 'Rewatches'}
					</p>
					<div class="flex items-center gap-1">
						<button
							type="button"
							onclick={() => (repeat = Math.max(0, repeat - 1))}
							class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
							disabled={repeat <= 0}
						>
							<Icon icon="solar:minus-circle-linear" class="size-3.5" />
						</button>
						<input
							type="number"
							min="0"
							bind:value={repeat}
							class="h-8 w-full min-w-0 rounded-md border bg-background px-2 text-center text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						/>
						<button
							type="button"
							onclick={() => (repeat = repeat + 1)}
							class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md border bg-background hover:bg-accent"
						>
							<Icon icon="solar:add-circle-linear" class="size-3.5" />
						</button>
					</div>
				</div>
			</div>

			<!-- Dates ──────────────────────────────────────────────────────── -->
			<div class="grid grid-cols-2 gap-3">
				<div>
					<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						Start Date
					</p>
					<div class="relative">
						<input
							type="date"
							bind:value={startedAt}
							class="h-9 w-full rounded-md border bg-background px-3 text-sm scheme-dark outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						/>
						{#if startedAt}
							<button
								type="button"
								onclick={() => (startedAt = '')}
								class="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								<Icon icon="solar:close-circle-linear" class="size-3.5" />
							</button>
						{/if}
					</div>
				</div>
				<div>
					<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
						Finish Date
					</p>
					<div class="relative">
						<input
							type="date"
							bind:value={completedAt}
							class="h-9 w-full rounded-md border bg-background px-3 text-sm scheme-dark outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
						/>
						{#if completedAt}
							<button
								type="button"
								onclick={() => (completedAt = '')}
								class="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								<Icon icon="solar:close-circle-linear" class="size-3.5" />
							</button>
						{/if}
					</div>
				</div>
			</div>

			<!-- Notes ──────────────────────────────────────────────────────── -->
			<div>
				<p class="mb-1.5 text-xs font-semibold tracking-wide text-muted-foreground uppercase">
					Notes
				</p>
				<textarea
					bind:value={notes}
					placeholder="Your private notes…"
					rows={2}
					class="w-full resize-none rounded-md border bg-background px-3 py-2 text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20"
				></textarea>
			</div>

			<!-- Toggles ────────────────────────────────────────────────────── -->
			<div class="flex items-center gap-5">
				<button
					type="button"
					onclick={() => (hiddenFromStatus = !hiddenFromStatus)}
					class="flex items-center gap-2 text-xs transition-colors {hiddenFromStatus
						? 'text-foreground'
						: 'text-muted-foreground hover:text-foreground'}"
				>
					<div
						class="flex h-4 w-4 items-center justify-center rounded border {hiddenFromStatus
							? 'border-primary bg-primary'
							: 'border-border bg-background'}"
					>
						{#if hiddenFromStatus}<Icon
								icon="solar:check-bold"
								class="size-2.5 text-primary-foreground"
							/>{/if}
					</div>
					Hide from status lists
				</button>
				<button
					type="button"
					onclick={() => (isPrivate = !isPrivate)}
					class="flex items-center gap-2 text-xs transition-colors {isPrivate
						? 'text-foreground'
						: 'text-muted-foreground hover:text-foreground'}"
				>
					<div
						class="flex h-4 w-4 items-center justify-center rounded border {isPrivate
							? 'border-primary bg-primary'
							: 'border-border bg-background'}"
					>
						{#if isPrivate}<Icon
								icon="solar:check-bold"
								class="size-2.5 text-primary-foreground"
							/>{/if}
					</div>
					Private
				</button>
			</div>

			<!-- Actions ────────────────────────────────────────────────────── -->
			<div class="flex items-center gap-2 border-t pt-4">
				{#if existing?.id}
					<Button
						variant="destructive"
						size="sm"
						class="mr-auto"
						onclick={handleRemove}
						disabled={isPending}
					>
						<Icon icon="solar:trash-bin-2-linear" class="mr-1.5 size-4" />Remove
					</Button>
				{/if}
				<Button
					variant="outline"
					size="sm"
					onclick={() => {
						open = false;
						onClose?.();
					}}
					disabled={isPending}
				>
					Cancel
				</Button>
				<Button size="sm" onclick={handleSave} disabled={isPending || !selectedStatus}>
					{#if isPending}
						<Icon icon="solar:refresh-circle-line-duotone" class="mr-1.5 size-4 animate-spin" />
					{:else}
						<Icon icon="solar:check-circle-linear" class="mr-1.5 size-4" />
					{/if}
					Save
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
