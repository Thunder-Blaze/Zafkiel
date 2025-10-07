<script lang="ts">
	import { Toaster as Sonner, type ToasterProps as SonnerProps } from 'svelte-sonner';
	import { mode } from 'mode-watcher';

	let { ...restProps }: SonnerProps = $props();
</script>

<Sonner
	theme={mode.current}
	class="toaster group"
	style="--normal-bg: var(--color-popover); --normal-text: var(--color-popover-foreground); --normal-border: var(--color-border);"
	toastOptions={{
		unstyled: false,
		classes: {
			toast:
				'[-webkit-backdrop-filter:blur(24px)] [backdrop-filter:blur(24px)] !bg-popover/95 border-border/50 shadow-2xl shadow-black/20 ring-1 ring-black/5 dark:shadow-black/40',
		},
	}}
	{...restProps}
/>
