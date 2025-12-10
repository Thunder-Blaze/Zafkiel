<script lang="ts">
	import Icon from '@iconify/svelte';
	import { formatTrailerLink } from '../utils/data-filters';

	let {
		trailer,
		banner,
	}: {
		trailer: { id?: string; site?: string; thumbnail?: string };
		banner: string;
	} = $props();
</script>

<div class="flex flex-col gap-1">
	<h4 class="text-sm font-semibold text-muted-foreground">Trailer</h4>
	<a
		href={formatTrailerLink(trailer)}
		target="_blank"
		rel="noopener noreferrer"
		class="relative flex h-9 w-30 items-center justify-center overflow-hidden rounded-full"
	>
		<img
			src={banner}
			alt="Trailer"
			class="absolute h-full w-full scale-150 object-cover opacity-80"
		/>
		<Icon
			icon="solar:play-bold"
			class="scale-150 text-white"
			style={'filter: drop-shadow(0 0 6px rgba(0, 0, 0, 0.5));'}
		/>
	</a>
</div>
