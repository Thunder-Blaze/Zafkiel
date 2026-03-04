<script lang="ts">
	import { tick } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { useConfigState } from '$lib/stores/config.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import type { Media, MediaSeason, MediaListStatus } from '$lib/types/anilist';
	import gsap from 'gsap';

	interface StatusOption {
		value: MediaListStatus;
		label: string;
		icon: string;
	}

	interface Props {
		items: Media[];
		mediaType?: 'anime' | 'manga';
		autoplayInterval?: number;
		statusOptions?: StatusOption[];
		addToListPending?: boolean;
		onAddToList?: (status: MediaListStatus, item: Media) => Promise<void> | void;
	}

	let {
		items = [],
		mediaType = 'anime',
		autoplayInterval = 5500,
		statusOptions = [],
		addToListPending = false,
		onAddToList,
	}: Props = $props();

	const config = useConfigState();
	const animEnabled = $derived(config.animations);

	// ── State ────────────────────────────────────────────────────────────────────
	let currentIndex = $state(0);
	let direction = $state<'left' | 'right'>('right');
	let displayedItem = $state<Media | undefined>(undefined);
	let addToListOpen = $state(false);

	let autoplayTimer: ReturnType<typeof setInterval> | null = null;
	let progressTween: gsap.core.Tween | undefined;
	let kenBurnsTween: gsap.core.Tween | undefined;
	let isAnimating = false;
	let prevIndex = -1;
	let isPaused = false;

	// ── DOM refs ──────────────────────────────────────────────────────────────────
	let bgSlotA: HTMLDivElement | undefined = $state();
	let bgSlotB: HTMLDivElement | undefined = $state();
	let activeBgSlot: 'a' | 'b' = 'a';

	let badgeEl: HTMLElement | undefined = $state();
	let titleEl: HTMLElement | undefined = $state();
	let metaEl: HTMLElement | undefined = $state();
	let descEl: HTMLElement | undefined = $state();
	let actionsEl: HTMLElement | undefined = $state();
	let coverWrapper: HTMLElement | undefined = $state();
	let progressBarEl: HTMLElement | undefined = $state();
	let shimmerEl: HTMLElement | undefined = $state();

	// ── Helpers ───────────────────────────────────────────────────────────────────
	function getBgImage(m: Media): string {
		return m.bannerImage || m.coverImage?.extraLarge || m.coverImage?.large || '';
	}

	function mediaTitle(m: Media): string {
		return m.title?.english || m.title?.romaji || m.title?.native || 'Unknown';
	}

	function formatDescription(desc: string | null | undefined): string {
		if (!desc) return '';
		const text = desc.replace(/<[^>]*>/g, '').replace(/\n/g, ' ');
		return text.length > 220 ? text.substring(0, 220) + '…' : text;
	}

	function seasonLabel(s: MediaSeason | undefined): string {
		if (!s) return '';
		return { WINTER: 'Winter', SPRING: 'Spring', SUMMER: 'Summer', FALL: 'Fall' }[s] ?? s;
	}

	function getContentEls(): HTMLElement[] {
		return [badgeEl, titleEl, metaEl, descEl, actionsEl].filter((e): e is HTMLElement => !!e);
	}

	// ── Ken Burns ─────────────────────────────────────────────────────────────────
	function startKenBurns(el: HTMLDivElement) {
		if (kenBurnsTween) kenBurnsTween.kill();
		kenBurnsTween = gsap.to(el, {
			scale: 1.07,
			x: '1.5%',
			duration: 14,
			ease: 'none',
			yoyo: true,
			repeat: -1,
		});
	}

	// ── Progress bar ─────────────────────────────────────────────────────────────
	function startProgress() {
		if (progressTween) progressTween.kill();
		if (!progressBarEl) return;
		gsap.set(progressBarEl, { width: '0%' });
		progressTween = gsap.to(progressBarEl, {
			width: '100%',
			duration: autoplayInterval / 1000,
			ease: 'none',
		});
		if (isPaused) progressTween.pause();
	}

	// ── Initial setup ─────────────────────────────────────────────────────────────
	$effect(() => {
		const first = items[0];
		if (!first || displayedItem !== undefined) return;
		displayedItem = first;

		Promise.resolve().then(() => {
			if (!bgSlotA || !bgSlotB) return;
			bgSlotA.style.backgroundImage = `url('${getBgImage(first)}')`;
			gsap.set(bgSlotA, { opacity: 1, scale: 1, zIndex: 1 });
			gsap.set(bgSlotB, { opacity: 0, scale: 1.07, zIndex: 0 });
			startKenBurns(bgSlotA);

			const els = getContentEls();
			gsap.fromTo(
				els,
				{ opacity: 0, y: 20, filter: 'blur(6px)' },
				{
					opacity: 1,
					y: 0,
					filter: 'blur(0px)',
					duration: 0.65,
					ease: 'power3.out',
					stagger: { amount: 0.32 },
					delay: 0.1,
				}
			);
			if (coverWrapper)
				gsap.fromTo(
					coverWrapper,
					{ opacity: 0, y: 28, scale: 0.88 },
					{ opacity: 1, y: 0, scale: 1, duration: 0.75, ease: 'back.out(1.6)', delay: 0.2 }
				);
			startProgress();
		});
	});

	// ── Transition effect ─────────────────────────────────────────────────────────
	$effect(() => {
		const idx = currentIndex;
		const newItem = items[idx];
		if (!newItem || !displayedItem || idx === prevIndex) return;
		if (!animEnabled) {
			prevIndex = idx;
			displayedItem = newItem;
			return;
		}

		prevIndex = idx;
		const dir = direction === 'right' ? 1 : -1;
		isAnimating = true;

		(async () => {
			const els = getContentEls();
			const activeSlot = activeBgSlot === 'a' ? bgSlotA : bgSlotB;
			const inactiveSlot = activeBgSlot === 'a' ? bgSlotB : bgSlotA;

			gsap.killTweensOf(els);
			if (coverWrapper) gsap.killTweensOf(coverWrapper);
			if (kenBurnsTween) kenBurnsTween.kill();

			// EXIT
			gsap.to(els, {
				opacity: 0,
				x: -24 * dir,
				filter: 'blur(5px)',
				duration: 0.22,
				ease: 'power2.in',
				stagger: { amount: 0.1 },
			});
			if (coverWrapper)
				gsap.to(coverWrapper, { opacity: 0, scale: 0.9, y: 14, duration: 0.22, ease: 'power2.in' });

			// BACKGROUND CROSSFADE
			if (inactiveSlot) {
				inactiveSlot.style.backgroundImage = `url('${getBgImage(newItem)}')`;
				gsap.set(inactiveSlot, { opacity: 0, scale: 1.07, zIndex: 2 });
				gsap.to(inactiveSlot, { opacity: 1, scale: 1, duration: 1.0, ease: 'power2.out' });
			}
			if (activeSlot) gsap.to(activeSlot, { opacity: 0, duration: 0.65, zIndex: 1 });

			// SHIMMER
			if (shimmerEl) {
				gsap.set(shimmerEl, { x: '-110%', opacity: 1 });
				gsap.to(shimmerEl, { x: '210%', opacity: 0.7, duration: 0.8, ease: 'power1.inOut' });
			}

			await new Promise<void>((res) => setTimeout(res, 250));

			activeBgSlot = activeBgSlot === 'a' ? 'b' : 'a';
			const newActive = activeBgSlot === 'a' ? bgSlotA : bgSlotB;
			if (newActive) {
				gsap.set(newActive, { zIndex: 2 });
				startKenBurns(newActive);
			}

			displayedItem = newItem;
			await tick();

			// ENTER
			gsap.set(els, { opacity: 0, x: 28 * dir, filter: 'blur(6px)' });
			if (coverWrapper) gsap.set(coverWrapper, { opacity: 0, y: 22, scale: 0.88 });
			gsap.to(els, {
				opacity: 1,
				x: 0,
				filter: 'blur(0px)',
				duration: 0.52,
				ease: 'power3.out',
				stagger: { amount: 0.28 },
			});
			if (coverWrapper)
				gsap.to(coverWrapper, {
					opacity: 1,
					y: 0,
					scale: 1,
					duration: 0.65,
					ease: 'back.out(1.7)',
					delay: 0.1,
				});

			isAnimating = false;
		})();
	});

	// ── Autoplay ──────────────────────────────────────────────────────────────────
	$effect(() => {
		if (items.length > 1) {
			autoplayTimer = setInterval(() => {
				if (!isPaused && !isAnimating) {
					direction = 'right';
					currentIndex = (currentIndex + 1) % items.length;
					startProgress();
				}
			}, autoplayInterval);
		}
		return () => {
			if (autoplayTimer) clearInterval(autoplayTimer);
			if (progressTween) progressTween.kill();
			if (kenBurnsTween) kenBurnsTween.kill();
		};
	});

	// ── Navigation ────────────────────────────────────────────────────────────────
	function prevSlide() {
		if (isAnimating) return;
		direction = 'left';
		currentIndex = (currentIndex - 1 + items.length) % items.length;
		startProgress();
	}

	function nextSlide() {
		if (isAnimating) return;
		direction = 'right';
		currentIndex = (currentIndex + 1) % items.length;
		startProgress();
	}

	function goToSlide(i: number) {
		if (isAnimating) return;
		direction = i > currentIndex ? 'right' : 'left';
		currentIndex = i;
		startProgress();
	}

	function handleMouseEnter() {
		isPaused = true;
		progressTween?.pause();
	}
	function handleMouseLeave() {
		isPaused = false;
		progressTween?.resume();
	}

	async function handleAddToList(status: MediaListStatus) {
		if (!displayedItem) return;
		addToListOpen = false;
		await onAddToList?.(status, displayedItem);
	}
</script>

{#if items.length > 0 && displayedItem}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="group relative h-[460px] w-full overflow-hidden"
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
	>
		<!-- Background slots -->
		<div
			bind:this={bgSlotA}
			class="absolute inset-0 bg-cover bg-center"
			style="z-index:1;will-change:transform,opacity"
		></div>
		<div
			bind:this={bgSlotB}
			class="absolute inset-0 bg-cover bg-center opacity-0"
			style="z-index:0;will-change:transform,opacity"
		></div>

		<!-- Shimmer sweep -->
		<div
			bind:this={shimmerEl}
			class="pointer-events-none absolute inset-0 z-[3]"
			style="background:linear-gradient(105deg,transparent 35%,rgba(255,255,255,0.07) 50%,transparent 65%);transform:translateX(-110%)"
		></div>

		<!-- Gradient overlays -->
		<div
			class="pointer-events-none absolute inset-0 z-[4] bg-linear-to-r from-background/92 via-background/55 to-background/5"
		></div>
		<div
			class="pointer-events-none absolute inset-0 z-[4] bg-linear-to-t from-background/88 via-transparent to-transparent"
		></div>

		<!-- Content -->
		<div
			class="relative z-[5] grid h-full grid-cols-[1fr_auto] items-end gap-4 px-8 pb-10 lg:px-14"
		>
			<div class="flex min-w-0 flex-col gap-2.5">
				<!-- Badge -->
				<div bind:this={badgeEl}>
					{#if mediaType === 'anime' && displayedItem.season && displayedItem.seasonYear}
						<p class="text-[11px] font-bold tracking-widest text-primary uppercase">
							{seasonLabel(displayedItem.season)}
							{displayedItem.seasonYear}
						</p>
					{:else if displayedItem.format}
						<p class="text-[11px] font-bold tracking-widest text-primary/80 uppercase">
							{displayedItem.format.replace(/_/g, ' ')}
						</p>
					{:else}
						<div class="h-4"></div>
					{/if}
				</div>

				<!-- Title -->
				<h1
					bind:this={titleEl}
					class="line-clamp-1 text-3xl leading-tight font-bold drop-shadow-lg md:text-4xl"
				>
					{mediaTitle(displayedItem)}
				</h1>

				<!-- Meta -->
				<div bind:this={metaEl} class="flex flex-wrap items-center gap-2 text-sm">
					{#if displayedItem.averageScore}
						<span class="flex items-center gap-1 font-semibold">
							<Icon icon="solar:star-bold" class="h-3.5 w-3.5 text-yellow-400" />
							{(displayedItem.averageScore / 10).toFixed(1)}
						</span>
					{/if}
					{#if displayedItem.format}
						<span class="rounded bg-foreground/10 px-2 py-0.5 text-xs font-medium">
							{displayedItem.format.replace(/_/g, ' ')}
						</span>
					{/if}
					{#if mediaType === 'anime' && displayedItem.episodes}
						<span class="text-xs text-muted-foreground">{displayedItem.episodes} eps</span>
					{/if}
					{#if mediaType === 'manga'}
						{#if displayedItem.chapters}
							<span class="text-xs text-muted-foreground">{displayedItem.chapters} ch</span>
						{/if}
						{#if displayedItem.volumes}
							<span class="text-xs text-muted-foreground">{displayedItem.volumes} vol</span>
						{/if}
					{/if}
					{#each (displayedItem.genres ?? []).slice(0, 3) as genre}
						<span
							class="rounded-md border border-border/40 bg-background/40 px-2.5 py-0.5 text-[11px] backdrop-blur-sm"
						>
							{genre}
						</span>
					{/each}
				</div>

				<!-- Description -->
				<div bind:this={descEl} class="h-[2.6em]">
					{#if displayedItem.description}
						<p class="line-clamp-2 max-w-lg text-xs leading-relaxed text-muted-foreground">
							{formatDescription(displayedItem.description)}
						</p>
					{/if}
				</div>

				<!-- Actions -->
				<div bind:this={actionsEl} class="flex gap-2 pt-0.5">
					<Button
						size="sm"
						onclick={() => goto(`/${mediaType}/${displayedItem!.id}`)}
						class="gap-2"
					>
						<Icon
							icon={mediaType === 'anime' ? 'solar:play-circle-bold' : 'solar:book-2-bold'}
							class="h-4 w-4"
						/>
						View Details
					</Button>

					{#if statusOptions.length > 0}
						<div class="relative">
							<Button
								size="sm"
								class="gap-2 bg-foreground text-background hover:bg-foreground/90"
								onclick={() => (addToListOpen = !addToListOpen)}
								disabled={addToListPending}
							>
								{#if addToListPending}
									<Icon icon="solar:spinner-bold" class="h-4 w-4 animate-spin" />
								{:else}
									<Icon icon="solar:add-circle-bold" class="h-4 w-4" />
								{/if}
								Add to List
							</Button>
							{#if addToListOpen}
								<!-- svelte-ignore a11y_no_static_element_interactions -->
								<div
									class="fixed inset-0 z-40"
									onclick={() => (addToListOpen = false)}
									aria-hidden="true"
								></div>
								<div
									class="absolute top-full left-0 z-50 mt-1.5 min-w-[190px] overflow-hidden rounded-lg border border-border bg-popover py-1 shadow-xl"
								>
									{#each statusOptions as opt}
										<button
											onclick={() => handleAddToList(opt.value)}
											class="flex w-full items-center gap-2.5 px-3 py-2 text-sm text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
										>
											<Icon icon={opt.icon} class="h-4 w-4 shrink-0 text-muted-foreground" />
											{opt.label}
										</button>
									{/each}
								</div>
							{/if}
						</div>
					{/if}
				</div>
			</div>

			<!-- Cover art -->
			<div bind:this={coverWrapper} class="relative mb-1 hidden shrink-0 md:block">
				{#if displayedItem.coverImage?.large || displayedItem.coverImage?.medium}
					<div class="relative">
						<CachedImage
							src={displayedItem.coverImage.large ?? displayedItem.coverImage.medium ?? ''}
							alt={mediaTitle(displayedItem)}
							class="h-52 w-36 rounded-xl object-cover shadow-2xl ring-2 ring-border/40"
						/>
						<div class="absolute -inset-3 -z-10 rounded-2xl bg-primary/20 opacity-70 blur-xl"></div>
					</div>
				{/if}
			</div>
		</div>

		<!-- Navigation arrows -->
		{#if items.length > 1}
			<button
				onclick={prevSlide}
				class="absolute top-1/2 left-4 z-[6] -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 group-hover:opacity-100 hover:scale-110 hover:bg-background/70"
				aria-label="Previous slide"
			>
				<Icon icon="solar:alt-arrow-left-bold" class="h-5 w-5" />
			</button>
			<button
				onclick={nextSlide}
				class="absolute top-1/2 right-4 z-[6] -translate-y-1/2 rounded-full bg-background/50 p-2.5 opacity-0 backdrop-blur-sm transition-all duration-200 group-hover:opacity-100 hover:scale-110 hover:bg-background/70"
				aria-label="Next slide"
			>
				<Icon icon="solar:alt-arrow-right-bold" class="h-5 w-5" />
			</button>
		{/if}

		<!-- Dots + progress bar -->
		{#if items.length > 1}
			<div class="absolute bottom-5 left-1/2 z-[6] flex -translate-x-1/2 gap-2">
				{#each items as _, i}
					<button
						onclick={() => goToSlide(i)}
						class="rounded-full transition-all duration-300 {i === currentIndex
							? 'h-1.5 w-7 bg-primary shadow-[0_0_6px_1px_hsl(var(--primary)/0.5)]'
							: 'h-1.5 w-1.5 bg-muted-foreground/40 hover:bg-muted-foreground/70'}"
						aria-label="Go to slide {i + 1}"
					></button>
				{/each}
			</div>
			<div class="absolute bottom-0 left-0 z-[6] h-0.5 w-full bg-border/20">
				<div bind:this={progressBarEl} class="h-full bg-primary/60" style="width:0%"></div>
			</div>
		{/if}
	</div>
{:else if items.length === 0}
	<div class="flex h-[460px] w-full items-center justify-center text-muted-foreground">
		<Icon icon="solar:spinner-bold" class="h-8 w-8 animate-spin" />
	</div>
{/if}
