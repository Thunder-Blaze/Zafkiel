<script lang="ts">
	import ArrowLeftIcon from '@lucide/svelte/icons/arrow-left';
	import type { WithoutChildren } from 'bits-ui';
	import { getEmblaContext } from './context.js';
	import { cn } from '$lib/utils.js';
	import { Button, type Props } from '$lib/components/ui/button/index.js';

	let {
		ref = $bindable(null),
		class: className,
		variant = 'outline',
		size = 'icon',
		...restProps
	}: WithoutChildren<Props> = $props();

	const emblaCtx = getEmblaContext('<Carousel.Previous/>');

	let holdInterval: ReturnType<typeof setInterval> | null = null;

	function handleMouseDown() {
		emblaCtx.scrollPrev();
		holdInterval = setInterval(() => {
			emblaCtx.scrollPrev();
		}, 300);
	}

	function handleMouseUp() {
		if (holdInterval) {
			clearInterval(holdInterval);
			holdInterval = null;
		}
	}

	function handleMouseLeave() {
		handleMouseUp();
	}
</script>

<Button
	data-slot="carousel-previous"
	{variant}
	{size}
	aria-disabled={!emblaCtx.canScrollPrev}
	class={cn(
		'absolute size-8 rounded-full',
		emblaCtx.orientation === 'horizontal'
			? 'top-1/2 -left-12 -translate-y-1/2'
			: '-top-12 left-1/2 -translate-x-1/2 rotate-90',
		className
	)}
	onmousedown={handleMouseDown}
	onmouseup={handleMouseUp}
	onmouseleave={handleMouseLeave}
	onkeydown={emblaCtx.handleKeyDown}
	{...restProps}
	bind:ref
>
	<ArrowLeftIcon class="size-4" />
	<span class="sr-only">Previous slide</span>
</Button>
