<script lang="ts">
	import Lenis from '@studio-freight/lenis';
	import { useConfigState } from '$lib/stores/config.svelte';

	const { children, wrapper = null }: { children: any; wrapper?: HTMLElement | null } = $props();

	let lenis: Lenis | null = $state(null);
	let rafId: number | null = $state(null);
	let contentEl = $state<HTMLElement | null>(null);
	let smoothScrollEnabled = $derived(useConfigState().smoothScroll);

	// Re-initialize whenever wrapper or contentEl become available
	$effect(() => {
		const scrollWrapper = wrapper;
		const scrollContent = contentEl;

		if (smoothScrollEnabled && scrollWrapper && scrollContent) {
			// Both wrapper (the fixed below-titlebar div) and content (this <main>)
			// must be provided so Lenis knows the full scrollable height.
			lenis = new Lenis({
				wrapper: scrollWrapper,
				content: scrollContent,
				smoothWheel: true,
				lerp: 0.1,
				orientation: 'vertical',
				gestureOrientation: 'vertical',
				syncTouch: false,
				syncTouchLerp: 0.075,
				touchInertiaMultiplier: 35,
				infinite: false,
				autoResize: true,
			});

			function raf(time: number) {
				lenis?.raf(time);
				rafId = requestAnimationFrame(raf);
			}

			rafId = requestAnimationFrame(raf);

			return () => {
				if (rafId !== null) {
					cancelAnimationFrame(rafId);
					rafId = null;
				}
				lenis?.destroy();
				lenis = null;
			};
		} else {
			// Clean up if smooth scroll is disabled or elements not ready
			if (rafId !== null) {
				cancelAnimationFrame(rafId);
				rafId = null;
			}
			if (lenis) {
				lenis.destroy();
				lenis = null;
			}
		}
	});
</script>

<main bind:this={contentEl}>
	{@render children()}
</main>
