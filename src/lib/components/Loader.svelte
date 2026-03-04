<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { loaderEntrance } from '$lib/utils/gsap-animations';

	const { text = 'Loading...' }: { text: string } = $props();

	let containerEl = $state<HTMLElement | null>(null);
	let dot1 = $state<HTMLElement | null>(null);
	let dot2 = $state<HTMLElement | null>(null);
	let dot3 = $state<HTMLElement | null>(null);

	let dotsTl: { kill: () => void } | null = null;

	onMount(async () => {
		if (!browser) return;
		const { gsap } = await import('gsap');

		// Entrance
		loaderEntrance(containerEl);

		// Bouncing dots loop
		const dots = [dot1, dot2, dot3].filter(Boolean) as HTMLElement[];
		const tl = gsap.timeline({ repeat: -1, repeatDelay: 0.2 });
		tl.to(dots, { y: -10, duration: 0.35, stagger: 0.1, ease: 'sine.inOut' }).to(dots, {
			y: 0,
			duration: 0.35,
			stagger: 0.1,
			ease: 'sine.inOut',
		});
		dotsTl = tl;
	});

	onDestroy(() => {
		dotsTl?.kill();
	});
</script>

<div bind:this={containerEl} class="flex min-h-screen items-center justify-center bg-background">
	<div class="flex flex-col items-center gap-6">
		<!-- Logo / brand mark -->
		<div data-loader-item class="flex items-center gap-3">
			<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10 ring-1 ring-primary/20">
				<svg viewBox="0 0 24 24" class="h-5 w-5 text-primary" fill="none" stroke="currentColor" stroke-width="2">
					<polygon points="5,3 19,12 5,21" stroke-linejoin="round" stroke-linecap="round" />
				</svg>
			</div>
			<span class="text-xl font-bold tracking-tight">Zafkiel</span>
		</div>

		<!-- Bouncing dots -->
		<div data-loader-item class="flex items-center gap-2">
			<span bind:this={dot1} class="h-2.5 w-2.5 rounded-full bg-primary/70"></span>
			<span bind:this={dot2} class="h-2.5 w-2.5 rounded-full bg-primary/70"></span>
			<span bind:this={dot3} class="h-2.5 w-2.5 rounded-full bg-primary/70"></span>
		</div>

		<!-- Status text -->
		<p data-loader-item class="text-sm text-muted-foreground">{text}</p>
	</div>
</div>
