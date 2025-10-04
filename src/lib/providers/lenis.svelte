<script lang="ts">
	const { children } = $props();
	import Lenis from '@studio-freight/lenis';

	$effect(() => {
		const lenis = new Lenis({
			smoothWheel: true,
			lerp: 0.1,
			orientation: 'vertical',
			gestureOrientation: 'vertical',
			syncTouch: false,
			syncTouchLerp: 0.075,
			touchInertiaMultiplier: 35,
			infinite: false,
			autoResize: true
		});

		function raf(time: number) {
			lenis.raf(time);
			requestAnimationFrame(raf);
		}

		requestAnimationFrame(raf);
	});
</script>

<main>
	{@render children()}
</main>
