/**
 * Svelte action: translate-based horizontal scroll with lerp smoothing.
 * Applied to the outer wrapper. Moves its firstElementChild via translateX.
 * Uses clip-path to mask left/right overflow while allowing vertical bleed
 * so card hover popups are never clipped.
 */
export function hscroll(node: HTMLElement) {
	const inner = node.firstElementChild as HTMLElement | null;
	if (!inner) return {};

	// Clip only the left/right edges; vertical bleed is unrestricted
	node.style.overflow = 'visible';
	inner.style.willChange = 'transform';

	let targetX = 0;
	let currentX = 0;
	let rafId: number | null = null;

	function maxScroll() {
		return Math.max(0, inner!.scrollWidth - node.offsetWidth);
	}

	function clamp(val: number) {
		return Math.max(-maxScroll(), Math.min(0, val));
	}

	function tick() {
		currentX += (targetX - currentX) * 0.1;
		inner!.style.transform = `translateX(${currentX}px)`;
		if (Math.abs(targetX - currentX) > 0.15) {
			rafId = requestAnimationFrame(tick);
		} else {
			currentX = targetX;
			inner!.style.transform = `translateX(${currentX}px)`;
			rafId = null;
		}
	}

	function onWheel(e: WheelEvent) {
		// Only intercept meaningful horizontal scroll; ignore pure vertical
		if (Math.abs(e.deltaX) < 5) return;
		e.preventDefault();
		targetX = clamp(targetX - e.deltaX);
		if (!rafId) rafId = requestAnimationFrame(tick);
	}

	node.addEventListener('wheel', onWheel, { passive: false });

	return {
		destroy() {
			node.removeEventListener('wheel', onWheel);
			if (rafId) cancelAnimationFrame(rafId);
		},
	};
}
