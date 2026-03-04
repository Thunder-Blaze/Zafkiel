<script lang="ts">
	import { tick } from 'svelte';
	import { goto } from '$app/navigation';
	import type { Media } from '$lib/types/anilist';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { useConfigState } from '$lib/stores/config.svelte';
	import CachedImage from '$lib/components/ui/CachedImage.svelte';
	import gsap from 'gsap';

	let {
		items = [],
		autoplayInterval = 5500,
	}: {
		items: Media[];
		autoplayInterval?: number;
	} = $props();

	const config = useConfigState();
	const animEnabled = $derived(config.animations);

	// ── State ───────────────────────────────────────────────────────────────────
	let currentIndex = $state(0);
	let direction = $state<'left' | 'right'>('right');
	let isPaused = $state(false);

	/** The item whose data is currently rendered in the DOM */
	let displayedItem = $state<Media | undefined>(undefined);

	let autoplayTimer: ReturnType<typeof setInterval> | null = null;
	let progressTween: gsap.core.Tween | undefined;
	let kenBurnsTween: gsap.core.Tween | undefined;
	let isAnimating = false;
	let prevIndex = -1;

	// ── DOM refs ────────────────────────────────────────────────────────────────
	let bgSlotA: HTMLDivElement | undefined = $state();
	let bgSlotB: HTMLDivElement | undefined = $state();
	let activeBgSlot: 'a' | 'b' = 'a';

	let badgeEl: HTMLElement | undefined = $state();
	let titleEl: HTMLElement | undefined = $state();
	let metaEl: HTMLElement | undefined = $state();
	let genresEl: HTMLElement | undefined = $state();
	let descEl: HTMLElement | undefined = $state();
	let actionsEl: HTMLElement | undefined = $state();
	let coverWrapper: HTMLElement | undefined = $state();
	let progressBarEl: HTMLElement | undefined = $state();
	let shimmerEl: HTMLElement | undefined = $state();

	// ── Helpers ─────────────────────────────────────────────────────────────────
	function getBgImage(m: Media): string {
		return m.bannerImage || m.coverImage?.extraLarge || m.coverImage?.large || '';
	}

	function formatDescription(desc: string | null | undefined): string {
		if (!desc) return '';
		const text = desc.replace(/<[^>]*>/g, '').replace(/\n/g, ' ');
		return text.length > 220 ? text.substring(0, 220) + '…' : text;
	}

	function handleViewDetails() {
		if (displayedItem) goto(`/${displayedItem.type?.toLowerCase() || 'anime'}/${displayedItem.id}`);
	}

	function getContentEls(): HTMLElement[] {
		return [badgeEl, titleEl, metaEl, genresEl, descEl, actionsEl].filter(
			(e): e is HTMLElement => !!e
		);
	}

	// ── Ken Burns on active background ──────────────────────────────────────────
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

	// ── Initial setup ────────────────────────────────────────────────────────────
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

			const contentEls = getContentEls();
			gsap.fromTo(
				contentEls,
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
			if (coverWrapper) {
				gsap.fromTo(
					coverWrapper,
					{ opacity: 0, y: 28, scale: 0.88 },
					{ opacity: 1, y: 0, scale: 1, duration: 0.75, ease: 'back.out(1.6)', delay: 0.18 }
				);
			}
		});
	});

	// ── Main GSAP transition ─────────────────────────────────────────────────────
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
			const contentEls = getContentEls();
			const activeSlot = activeBgSlot === 'a' ? bgSlotA : bgSlotB;
			const inactiveSlot = activeBgSlot === 'a' ? bgSlotB : bgSlotA;

			gsap.killTweensOf(contentEls);
			if (coverWrapper) gsap.killTweensOf(coverWrapper);
			if (kenBurnsTween) kenBurnsTween.kill();

			// ── EXIT ──────────────────────────────────────────────────────────────
			gsap.to(contentEls, {
				opacity: 0,
				x: -24 * dir,
				filter: 'blur(5px)',
				duration: 0.22,
				ease: 'power2.in',
				stagger: { amount: 0.1, from: dir === 1 ? 'start' : 'end' },
			});
			if (coverWrapper)
				gsap.to(coverWrapper, { opacity: 0, scale: 0.9, y: 14, duration: 0.22, ease: 'power2.in' });

			// ── BACKGROUND CROSSFADE ──────────────────────────────────────────────
			if (inactiveSlot) {
				inactiveSlot.style.backgroundImage = `url('${getBgImage(newItem)}')`;
				gsap.set(inactiveSlot, { opacity: 0, scale: 1.07, zIndex: 2 });
				gsap.to(inactiveSlot, { opacity: 1, scale: 1, duration: 1.0, ease: 'power2.out' });
			}
			if (activeSlot) gsap.to(activeSlot, { opacity: 0, duration: 0.65, zIndex: 1 });

			// ── SHIMMER SWEEP ─────────────────────────────────────────────────────
			if (shimmerEl) {
				gsap.set(shimmerEl, { x: '-110%', opacity: 1 });
				gsap.to(shimmerEl, { x: '210%', opacity: 0.7, duration: 0.8, ease: 'power1.inOut' });
			}

			await new Promise<void>((res) => setTimeout(res, 250));

			activeBgSlot = activeBgSlot === 'a' ? 'b' : 'a';
			const newActiveSlot = activeBgSlot === 'a' ? bgSlotA : bgSlotB;
			if (newActiveSlot) {
				gsap.set(newActiveSlot, { zIndex: 2 });
				startKenBurns(newActiveSlot);
			}

			displayedItem = newItem;
			await tick();

			// ── ENTER ─────────────────────────────────────────────────────────────
			gsap.set(contentEls, { opacity: 0, x: 28 * dir, filter: 'blur(6px)' });
			if (coverWrapper) gsap.set(coverWrapper, { opacity: 0, y: 22, scale: 0.88 });

			gsap.to(contentEls, {
				opacity: 1,
				x: 0,
				filter: 'blur(0px)',
				duration: 0.52,
				ease: 'power3.out',
				stagger: { amount: 0.28, from: 'start' },
			});
			if (coverWrapper) {
				gsap.to(coverWrapper, {
					opacity: 1,
					y: 0,
					scale: 1,
					duration: 0.65,
					ease: 'back.out(1.7)',
					delay: 0.1,
				});
			}
			isAnimating = false;
		})();
	});

	// ── GSAP progress bar ────────────────────────────────────────────────────────
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

	// ── Navigation ────────────────────────────────────────────────────────────────
	function goToSlide(index: number) {
		if (isAnimating) return;
		direction = index > currentIndex ? 'right' : 'left';
		currentIndex = index;
		startProgress();
	}

	function nextSlide() {
		if (isAnimating) return;
		direction = 'right';
		currentIndex = (currentIndex + 1) % items.length;
		startProgress();
	}

	function prevSlide() {
		if (isAnimating) return;
		direction = 'left';
		currentIndex = (currentIndex - 1 + items.length) % items.length;
		startProgress();
	}

	// ── Autoplay ──────────────────────────────────────────────────────────────────
	$effect(() => {
		if (items.length > 1) {
			startProgress();
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

	function handleMouseEnter() {
		isPaused = true;
		progressTween?.pause();
	}
	function handleMouseLeave() {
		isPaused = false;
		progressTween?.resume();
	}
</script>

{#if items.length > 0 && displayedItem}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="group relative h-[460px] w-full overflow-hidden"
		onmouseenter={handleMouseEnter}
		onmouseleave={handleMouseLeave}
	>
		<!-- Background slot A -->
		<div
			bind:this={bgSlotA}
			class="absolute inset-0 bg-cover bg-center"
			style="z-index:1;will-change:transform,opacity"
		></div>
		<!-- Background slot B -->
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
			<!-- Left text -->
			<div class="flex min-w-0 flex-col gap-2.5">
				<div bind:this={badgeEl}>
					{#if displayedItem.season && displayedItem.seasonYear}
						<p class="text-[11px] font-bold tracking-widest text-primary uppercase">
							{displayedItem.season}
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

				<h2
					bind:this={titleEl}
					class="line-clamp-1 text-3xl leading-tight font-bold drop-shadow-lg md:text-4xl"
				>
					{displayedItem.title?.english ||
						displayedItem.title?.romaji ||
						displayedItem.title?.native}
				</h2>

				<div bind:this={metaEl} class="flex flex-wrap items-center gap-2 text-sm">
					{#if displayedItem.averageScore}
						<span class="flex items-center gap-1 font-semibold">
							<Icon icon="solar:star-bold" class="h-3.5 w-3.5 text-yellow-400" />
							{(displayedItem.averageScore / 10).toFixed(1)}
						</span>
					{/if}
					{#if displayedItem.format}
						<span class="rounded bg-foreground/10 px-2 py-0.5 text-xs font-medium"
							>{displayedItem.format.replace(/_/g, ' ')}</span
						>
					{/if}
					{#if displayedItem.episodes}
						<span class="text-xs text-muted-foreground">{displayedItem.episodes} eps</span>
					{/if}
				</div>

				<div bind:this={genresEl} class="flex flex-wrap gap-1.5">
					{#each (displayedItem.genres ?? []).slice(0, 4) as genre}
						<span
							class="rounded-md border border-border/40 bg-background/40 px-2.5 py-0.5 text-[11px] font-medium backdrop-blur-sm"
							>{genre}</span
						>
					{/each}
				</div>

				<div bind:this={descEl} class="h-[2.6em]">
					{#if displayedItem.description}
						<p class="line-clamp-2 max-w-lg text-xs leading-relaxed text-muted-foreground">
							{formatDescription(displayedItem.description)}
						</p>
					{/if}
				</div>

				<div bind:this={actionsEl} class="flex items-center gap-2 pt-0.5">
					<Button size="sm" onclick={handleViewDetails}>
						<Icon icon="solar:play-bold" class="mr-1.5 h-4 w-4" />
						View Details
					</Button>
					<Button size="sm" class="gap-2 bg-foreground text-background hover:bg-foreground/90">
						<Icon icon="solar:add-circle-bold" class="h-4 w-4" />
						Add to List
					</Button>
				</div>
			</div>

			<!-- Right: cover art -->
			<div bind:this={coverWrapper} class="relative mb-1 hidden shrink-0 md:block">
				{#if displayedItem.coverImage?.large || displayedItem.coverImage?.medium}
					<div class="relative">
						<CachedImage
							src={displayedItem.coverImage.large ?? displayedItem.coverImage.medium ?? ''}
							alt={displayedItem.title?.romaji ?? ''}
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

		<!-- Dot indicators -->
		{#if items.length > 1}
			<div class="absolute bottom-5 left-1/2 z-[6] flex -translate-x-1/2 gap-2">
				{#each items as _, index}
					<button
						onclick={() => goToSlide(index)}
						class="rounded-full transition-all duration-300 {index === currentIndex
							? 'h-1.5 w-7 bg-primary shadow-[0_0_6px_1px_hsl(var(--primary)/0.5)]'
							: 'h-1.5 w-1.5 bg-muted-foreground/40 hover:bg-muted-foreground/70'}"
						aria-label="Go to slide {index + 1}"
					></button>
				{/each}
			</div>

			<!-- GSAP-driven progress bar -->
			<div class="absolute bottom-0 left-0 z-[6] h-0.5 w-full bg-border/20">
				<div bind:this={progressBarEl} class="h-full bg-primary/60" style="width:0%"></div>
			</div>
		{/if}
	</div>
{/if}
