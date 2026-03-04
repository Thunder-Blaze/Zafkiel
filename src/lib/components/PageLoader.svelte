<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import gsap from 'gsap';

	type PageType = 'anime' | 'manga' | 'character' | 'user' | 'staff' | 'studio' | 'default';

	interface Props {
		type?: PageType;
	}

	let { type = 'default' }: Props = $props();

	let container: HTMLDivElement | undefined = $state();
	let shimmerTl: gsap.core.Timeline | undefined;

	onMount(() => {
		if (!container) return;

		// Entrance: fade + slide in
		gsap.fromTo(
			container,
			{ opacity: 0, y: 16 },
			{ opacity: 1, y: 0, duration: 0.4, ease: 'power2.out' }
		);

		// Shimmer wave across skeleton elements
		const skeletons = container.querySelectorAll<HTMLElement>('.skel');
		if (skeletons.length === 0) return;

		shimmerTl = gsap.timeline({ repeat: -1, yoyo: true, defaults: { ease: 'sine.inOut' } });
		shimmerTl.to(skeletons, {
			opacity: 0.4,
			duration: 0.9,
			stagger: { amount: 0.5, from: 'start' },
		});
	});

	onDestroy(() => {
		shimmerTl?.kill();
	});
</script>

<div bind:this={container} class="w-full animate-in" aria-busy="true" aria-label="Loading...">
	{#if type === 'anime' || type === 'manga'}
		<!-- Banner skeleton -->
		<div class="skel relative mb-6 h-56 w-full rounded-xl bg-muted/70 md:h-72"></div>

		<!-- Cover + info row -->
		<div class="flex gap-5 px-2 md:px-0">
			<!-- Cover image -->
			<div
				class="skel hidden h-56 w-36 shrink-0 rounded-xl bg-muted/80 sm:block md:h-64 md:w-44"
			></div>
			<div class="flex-1 space-y-3 pt-2">
				<!-- Title -->
				<div class="skel h-8 w-3/4 rounded-lg bg-muted/80"></div>
				<!-- Subtitle -->
				<div class="skel h-5 w-1/2 rounded-lg bg-muted/60"></div>
				<!-- Badge row -->
				<div class="flex flex-wrap gap-2">
					<div class="skel h-6 w-16 rounded-full bg-muted/60"></div>
					<div class="skel h-6 w-20 rounded-full bg-muted/60"></div>
					<div class="skel h-6 w-14 rounded-full bg-muted/60"></div>
					<div class="skel h-6 w-18 rounded-full bg-muted/60"></div>
				</div>
				<!-- Action buttons -->
				<div class="flex gap-2 pt-1">
					<div class="skel h-9 w-32 rounded-lg bg-muted/70"></div>
					<div class="skel h-9 w-24 rounded-lg bg-muted/50"></div>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="mt-8 flex gap-1 border-b pb-0">
			{#each [80, 60, 70, 55, 65] as w}
				<div class="skel mx-1 h-9 rounded-t-lg bg-muted/60" style="width:{w}px"></div>
			{/each}
		</div>

		<!-- Content cards row -->
		<div class="mt-6 grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
			{#each Array(5) as _, i}
				<div class="space-y-2">
					<div class="skel aspect-[3/4] w-full rounded-lg bg-muted/70"></div>
					<div class="skel h-4 w-3/4 rounded bg-muted/60"></div>
					<div class="skel h-3 w-1/2 rounded bg-muted/40"></div>
				</div>
			{/each}
		</div>
	{:else if type === 'character' || type === 'staff'}
		<!-- Hero area -->
		<div class="flex flex-col items-center gap-6 pb-6 pt-4 md:flex-row md:items-start">
			<!-- Image -->
			<div class="skel h-52 w-36 shrink-0 rounded-2xl bg-muted/80 md:h-64 md:w-44"></div>
			<!-- Details -->
			<div class="w-full space-y-3 pt-2">
				<div class="skel h-8 w-2/3 rounded-lg bg-muted/80"></div>
				<div class="skel h-5 w-1/3 rounded-lg bg-muted/50"></div>
				<div class="flex flex-wrap gap-2 pt-1">
					{#each [60, 80, 55, 70] as w}
						<div class="skel h-6 rounded-full bg-muted/60" style="width:{w}px"></div>
					{/each}
				</div>
				<!-- Bio lines -->
				{#each [100, 90, 95, 75] as pct}
					<div class="skel h-4 rounded bg-muted/50" style="width:{pct}%"></div>
				{/each}
			</div>
		</div>

		<!-- Tabs -->
		<div class="mb-6 flex gap-1 border-b">
			{#each [80, 70, 90] as w}
				<div class="skel mx-1 h-9 rounded-t-lg bg-muted/60" style="width:{w}px"></div>
			{/each}
		</div>

		<!-- Card grid -->
		<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
			{#each Array(10) as _, i}
				<div class="space-y-2">
					<div class="skel aspect-[3/4] w-full rounded-lg bg-muted/70"></div>
					<div class="skel h-4 w-4/5 rounded bg-muted/50"></div>
				</div>
			{/each}
		</div>
	{:else if type === 'user'}
		<!-- Profile banner -->
		<div class="skel relative mb-0 h-44 w-full rounded-t-2xl bg-muted/70 md:h-56"></div>

		<!-- Avatar + name row -->
		<div class="-mt-12 flex items-end gap-4 px-4 pb-4">
			<div class="skel h-24 w-24 shrink-0 rounded-2xl bg-muted/90 ring-4 ring-background"></div>
			<div class="mb-2 space-y-2">
				<div class="skel h-7 w-40 rounded-lg bg-muted/80"></div>
				<div class="skel h-4 w-24 rounded bg-muted/50"></div>
			</div>
		</div>

		<!-- Stats row -->
		<div class="mt-2 flex gap-3 px-4">
			{#each Array(4) as _}
				<div class="skel h-16 flex-1 rounded-xl bg-muted/60"></div>
			{/each}
		</div>

		<!-- Tabs -->
		<div class="mt-6 flex gap-1 border-b px-4">
			{#each [80, 70, 85, 60] as w}
				<div class="skel mx-1 h-9 rounded-t-lg bg-muted/60" style="width:{w}px"></div>
			{/each}
		</div>

		<!-- Content rows -->
		<div class="mt-6 space-y-3 px-4">
			{#each Array(5) as _}
				<div class="skel h-16 w-full rounded-xl bg-muted/50"></div>
			{/each}
		</div>
	{:else if type === 'studio'}
		<!-- Studio header -->
		<div class="flex flex-col items-center gap-4 py-8">
			<div class="skel h-24 w-24 rounded-full bg-muted/70"></div>
			<div class="skel h-7 w-48 rounded-lg bg-muted/80"></div>
			<div class="flex gap-2">
				<div class="skel h-6 w-24 rounded-full bg-muted/50"></div>
				<div class="skel h-6 w-20 rounded-full bg-muted/50"></div>
			</div>
		</div>

		<!-- Card grid -->
		<div class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
			{#each Array(10) as _}
				<div class="space-y-2">
					<div class="skel aspect-[3/4] w-full rounded-lg bg-muted/70"></div>
					<div class="skel h-4 w-4/5 rounded bg-muted/50"></div>
				</div>
			{/each}
		</div>
	{:else}
		<!-- Default: simple centered shimmer block -->
		<div class="flex min-h-[400px] flex-col items-center justify-center gap-6">
			<div class="space-y-3 text-center">
				<div class="skel mx-auto h-12 w-12 rounded-full bg-muted/80"></div>
				<div class="skel mx-auto h-5 w-40 rounded-lg bg-muted/60"></div>
				<div class="skel mx-auto h-4 w-56 rounded bg-muted/40"></div>
			</div>
		</div>
	{/if}
</div>
