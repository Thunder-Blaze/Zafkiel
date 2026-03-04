/**
 * GSAP animation utilities for consistent, performant animations across the app.
 * All functions are SSR-safe (only run in the browser).
 */
import { browser } from '$app/environment';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

if (browser) {
	gsap.registerPlugin(ScrollTrigger);
}

// ── Easing presets ────────────────────────────────────────────────────────────
export const ease = {
	smooth: 'power2.out',
	snappy: 'power3.out',
	bouncy: 'back.out(1.6)',
	expo: 'expo.out',
	elastic: 'elastic.out(1, 0.4)',
	spring: 'power4.out',
} as const;

// ── Entrance animations ───────────────────────────────────────────────────────

/**
 * Fade and slide up a single element
 */
export function fadeSlideUp(
	el: Element | null,
	options?: { delay?: number; duration?: number; y?: number }
) {
	if (!browser || !el) return;
	const { delay = 0, duration = 0.55, y = 24 } = options ?? {};
	gsap.fromTo(
		el,
		{ opacity: 0, y },
		{ opacity: 1, y: 0, duration, delay, ease: ease.smooth, clearProps: 'all' }
	);
}

/**
 * Stagger-animate a list of children inside a container
 */
export function staggerChildren(
	container: Element | null,
	options?: {
		delay?: number;
		stagger?: number;
		duration?: number;
		y?: number;
		selector?: string;
	}
) {
	if (!browser || !container) return;
	const { delay = 0, stagger = 0.07, duration = 0.5, y = 20, selector = ':scope > *' } =
		options ?? {};
	gsap.fromTo(
		container.querySelectorAll(selector),
		{ opacity: 0, y },
		{
			opacity: 1,
			y: 0,
			duration,
			delay,
			stagger,
			ease: ease.smooth,
			clearProps: 'all',
		}
	);
}

/**
 * Animate items appearing one by one (for card grids / horizontal lists)
 */
export function staggerCards(
	items: NodeList | Element[],
	options?: { delay?: number; stagger?: number; duration?: number; x?: number; y?: number }
) {
	if (!browser || !items.length) return;
	const { delay = 0, stagger = 0.06, duration = 0.45, x = 0, y = 16 } = options ?? {};
	gsap.fromTo(
		Array.from(items),
		{ opacity: 0, y, x },
		{
			opacity: 1,
			y: 0,
			x: 0,
			duration,
			delay,
			stagger,
			ease: ease.snappy,
			clearProps: 'all',
		}
	);
}

// ── Scroll-triggered animations ───────────────────────────────────────────────

/**
 * Trigger a fade-slide-up when the element scrolls into view.
 * Returns the ScrollTrigger instance so it can be killed on destroy.
 */
export function scrollReveal(
	el: Element | null,
	options?: {
		y?: number;
		duration?: number;
		start?: string;
		scroller?: Element | string;
	}
) {
	if (!browser || !el) return null;
	const { y = 32, duration = 0.6, start = 'top 92%', scroller } = options ?? {};

	gsap.set(el, { opacity: 0, y });
	return ScrollTrigger.create({
		trigger: el,
		start,
		scroller,
		once: true,
		onEnter() {
			gsap.to(el, { opacity: 1, y: 0, duration, ease: ease.smooth, clearProps: 'all' });
		},
	});
}

/**
 * Stagger-reveal children of a container when it scrolls into view.
 */
export function scrollStaggerReveal(
	container: Element | null,
	options?: {
		selector?: string;
		y?: number;
		stagger?: number;
		duration?: number;
		start?: string;
		scroller?: Element | string;
	}
) {
	if (!browser || !container) return null;
	const {
		selector = ':scope > *',
		y = 24,
		stagger = 0.07,
		duration = 0.5,
		start = 'top 92%',
		scroller,
	} = options ?? {};

	const children = Array.from(container.querySelectorAll(selector));
	if (!children.length) return null;

	gsap.set(children, { opacity: 0, y });
	return ScrollTrigger.create({
		trigger: container,
		start,
		scroller,
		once: true,
		onEnter() {
			gsap.to(children, {
				opacity: 1,
				y: 0,
				duration,
				stagger,
				ease: ease.smooth,
				clearProps: 'all',
			});
		},
	});
}

// ── Loading animations ────────────────────────────────────────────────────────

/**
 * Pulsing dot loading animation for arrays of dot elements.
 * Returns a GSAP timeline so it can be killed on destroy.
 */
export function loaderDots(dots: Element[]) {
	if (!browser || !dots.length) return null;
	const tl = gsap.timeline({ repeat: -1 });
	tl.to(dots, {
		y: -10,
		duration: 0.4,
		stagger: 0.12,
		ease: 'sine.inOut',
	}).to(dots, {
		y: 0,
		duration: 0.4,
		stagger: 0.12,
		ease: 'sine.inOut',
	});
	return tl;
}

/**
 * Animated spinner ring using GSAP (more polished than CSS spin).
 */
export function spinnerAnimation(el: Element | null) {
	if (!browser || !el) return null;
	return gsap.to(el, {
		rotation: 360,
		duration: 0.9,
		repeat: -1,
		ease: 'none',
	});
}

/**
 * Entrance animation for the full loading screen.
 */
export function loaderEntrance(container: Element | null) {
	if (!browser || !container) return null;
	const tl = gsap.timeline();
	tl.fromTo(container, { opacity: 0 }, { opacity: 1, duration: 0.4, ease: ease.smooth });
	tl.fromTo(
		container.querySelectorAll('[data-loader-item]'),
		{ opacity: 0, y: 18, scale: 0.95 },
		{ opacity: 1, y: 0, scale: 1, duration: 0.55, stagger: 0.1, ease: ease.bouncy }
	);
	return tl;
}

/**
 * Loader exit animation (before unmounting).
 */
export function loaderExit(container: Element | null): Promise<void> {
	return new Promise((resolve) => {
		if (!browser || !container) {
			resolve();
			return;
		}
		gsap.to(container, {
			opacity: 0,
			y: -10,
			duration: 0.35,
			ease: ease.snappy,
			onComplete: resolve,
		});
	});
}

// ── Page transition helpers ───────────────────────────────────────────────────

/**
 * Animate a page entering from the right (or left).
 */
export function pageEnter(el: Element | null, direction: 'left' | 'right' = 'right') {
	if (!browser || !el) return null;
	const x = direction === 'right' ? 40 : -40;
	return gsap.fromTo(
		el,
		{ opacity: 0, x },
		{ opacity: 1, x: 0, duration: 0.5, ease: ease.snappy, clearProps: 'all' }
	);
}

/**
 * Animate a page exiting.
 */
export function pageExit(el: Element | null, direction: 'left' | 'right' = 'right'): Promise<void> {
	return new Promise((resolve) => {
		if (!browser || !el) {
			resolve();
			return;
		}
		const x = direction === 'right' ? -30 : 30;
		gsap.to(el, {
			opacity: 0,
			x,
			duration: 0.3,
			ease: ease.snappy,
			onComplete: resolve,
		});
	});
}

// ── Micro-interaction helpers ─────────────────────────────────────────────────

/**
 * Quick pop scale effect — great for button clicks.
 */
export function popClick(el: Element | null) {
	if (!browser || !el) return;
	gsap.fromTo(el, { scale: 0.92 }, { scale: 1, duration: 0.35, ease: ease.bouncy });
}

/**
 * Highlight pulse — good for newly added cards or focus indicators.
 */
export function highlightPulse(el: Element | null) {
	if (!browser || !el) return;
	const tl = gsap.timeline();
	tl.to(el, { scale: 1.04, duration: 0.2, ease: ease.smooth });
	tl.to(el, { scale: 1, duration: 0.4, ease: ease.bouncy });
	return tl;
}

// ── Svelte action: scroll-reveal ─────────────────────────────────────────────

/**
 * Svelte use: action that applies an IntersectionObserver-based reveal to the node.
 * Works with custom scroll containers without needing ScrollTrigger scroller config.
 * Usage: <div use:gsapReveal>
 */
export function gsapReveal(
	node: HTMLElement,
	params?: { y?: number; duration?: number; delay?: number }
) {
	if (!browser) return {};

	const { y = 28, duration = 0.6, delay = 0 } = params ?? {};
	gsap.set(node, { opacity: 0, y });

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					gsap.to(node, { opacity: 1, y: 0, duration, delay, ease: ease.smooth, clearProps: 'all' });
					observer.disconnect();
				}
			});
		},
		{ threshold: 0.08 }
	);
	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
		},
	};
}

/**
 * Svelte use: action for stagger-reveal of direct children using IntersectionObserver.
 * Usage: <ul use:gsapStagger>
 */
export function gsapStagger(
	node: HTMLElement,
	params?: {
		selector?: string;
		y?: number;
		stagger?: number;
		duration?: number;
		delay?: number;
	}
) {
	if (!browser) return {};

	const { selector = ':scope > *', y = 20, stagger = 0.07, duration = 0.5, delay = 0 } =
		params ?? {};
	const children = Array.from(node.querySelectorAll(selector)) as Element[];
	if (!children.length) return {};

	gsap.set(children, { opacity: 0, y });

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					gsap.to(children, {
						opacity: 1,
						y: 0,
						duration,
						delay,
						stagger,
						ease: ease.smooth,
						clearProps: 'all',
					});
					observer.disconnect();
				}
			});
		},
		{ threshold: 0.05 }
	);
	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
		},
	};
}
