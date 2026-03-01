<script lang="ts">
	import emblaCarouselSvelte from 'embla-carousel-svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import { getEmblaContext } from './context.js';
	import { cn, type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();

	const emblaCtx = getEmblaContext('<Carousel.Content/>');

	let containerRef: HTMLDivElement;

	function handleWheel(e: WheelEvent) {
		// Only handle horizontal scrolling (shift + wheel or trackpad horizontal scroll)
		if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
			e.preventDefault();
			if (e.deltaX > 0) {
				emblaCtx.scrollNext();
			} else {
				emblaCtx.scrollPrev();
			}
		}
		// Ignore vertical scrolling to allow page scroll
	}
</script>

<div
	bind:this={containerRef}
	data-slot="carousel-content"
	class=""
	onwheel={handleWheel}
	use:emblaCarouselSvelte={{
		options: {
			container: '[data-embla-container]',
			slides: '[data-embla-slide]',
			...emblaCtx.options,
			axis: emblaCtx.orientation === 'horizontal' ? 'x' : 'y',
		},
		plugins: emblaCtx.plugins,
	}}
	onemblaInit={emblaCtx.onInit}
>
	<div
		bind:this={ref}
		class={cn(
			'flex',
			emblaCtx.orientation === 'horizontal' ? '-ml-4' : '-mt-4 flex-col',
			className
		)}
		data-embla-container=""
		{...restProps}
	>
		{@render children?.()}
	</div>
</div>
