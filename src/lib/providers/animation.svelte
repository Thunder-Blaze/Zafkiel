<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { afterNavigate } from '$app/navigation';
	import autoAnimate from '@formkit/auto-animate';
	import { ConfigService, type UiConfig } from '$lib/services/config';

	let { children } = $props();
	let animationsEnabled = $state(false);
	let contentEl = $state<HTMLElement | null>(null);

	// Load config on mount
	$effect(() => {
		ConfigService.getUiConfig().then((config: UiConfig) => {
			if (config) {
				animationsEnabled = config.animations;
			}
		});
	});

	// GSAP page-enter animation after each navigation
	if (browser) {
		afterNavigate(() => {
			if (!animationsEnabled || !contentEl) return;
			import('gsap').then(({ gsap }) => {
				gsap.fromTo(
					contentEl!,
					{ opacity: 0.85, y: 10 },
					{ opacity: 1, y: 0, duration: 0.4, ease: 'power3.out', clearProps: 'all' }
				);
			});
		});
	}
</script>

{#if animationsEnabled}
	<div bind:this={contentEl} class="contents" use:autoAnimate={{ duration: 200 }}>
		{@render children?.()}
	</div>
{:else}
	{@render children?.()}
{/if}
