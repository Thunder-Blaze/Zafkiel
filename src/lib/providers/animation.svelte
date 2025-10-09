<script lang="ts">
	import autoAnimate from '@formkit/auto-animate';
	import { ConfigService, type UiConfig } from '$lib/services/config';

	let { children } = $props();
	let animationsEnabled = $state(false);

	// Load config on mount
	$effect(() => {
		ConfigService.getUiConfig().then((config: UiConfig) => {
			if (config) {
				animationsEnabled = config.animations;
			}
		});
	});

</script>

{#if animationsEnabled}
	<div use:autoAnimate="{{ duration: 200 }}">
		{@render children?.()}
	</div>
{:else}
	{@render children?.()}
{/if}
