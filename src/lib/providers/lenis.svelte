<script lang="ts">
	import Lenis from '@studio-freight/lenis';
	import { useConfigState } from '$lib/stores/config.svelte';

	const { children } = $props();

	let lenis: Lenis | null = $state(null);
	let rafId: number | null = $state(null);
	let smoothScrollEnabled = $derived(useConfigState().smoothScroll);

	// Initialize/destroy Lenis based on config
	$effect(() => {
		if (smoothScrollEnabled) {
			// Initialize Lenis
			lenis = new Lenis({
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

			// Cleanup
			return () => {
				if (rafId !== null) {
					cancelAnimationFrame(rafId);
					rafId = null;
				}
				lenis?.destroy();
				lenis = null;
			};
		} else {
			// Clean up if smooth scroll is disabled
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

<main>
	{@render children()}
</main>
