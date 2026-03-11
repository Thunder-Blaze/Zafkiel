<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	interface Props {
		src: string | undefined | null;
		alt?: string;
		class?: string;
		cookie?: string | null;
		referer?: string | null;
	}

	const { src, alt = '', class: className = '', cookie = null, referer = null }: Props = $props();

	let dataSrc = $state<string | null>(null);
	let loading = $state(false);
	let failed = $state(false);

	// Fetch image as base64 whenever src changes.
	$effect(() => {
		const url = src;
		if (!url) {
			dataSrc = null;
			return;
		}

		loading = true;
		failed = false;
		dataSrc = null;

		invoke<string>('fetch_image_base64', {
			url,
			cookie: cookie ?? null,
			referer: referer ?? null,
		})
			.then((dataUrl) => {
				dataSrc = dataUrl;
				loading = false;
			})
			.catch(() => {
				failed = true;
				loading = false;
			});
	});
</script>

{#if dataSrc}
	<img src={dataSrc} {alt} class={className} />
{:else if loading}
	<div class="animate-pulse bg-muted {className}"></div>
{:else if failed}
	<div
		class="flex items-center justify-center bg-muted/50 text-[10px] text-muted-foreground {className}"
	>
		✕
	</div>
{/if}
