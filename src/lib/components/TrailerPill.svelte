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

<div class="flex flex-col gap-2">
	<h4 class="text-sm font-medium text-muted-foreground">Trailer</h4>
	<a
		href={formatTrailerLink(trailer)}
		target="_blank"
		rel="noopener noreferrer"
		class="group relative flex h-12 w-36 items-center justify-center overflow-hidden rounded-full shadow-lg transition-all duration-300 hover:shadow-primary/10"
	>
		<!-- Background image with overlay -->
		<img
			src={banner}
			alt="Trailer Banner"
			class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
		/>
		<div class="absolute inset-0 bg-black/40 transition-colors duration-300 group-hover:bg-black/30"></div>

		<!-- Centered Play Icon -->
		<Icon
			icon="solar:play-bold"
			class="relative z-10 size-6 text-white drop-shadow-md transition-transform duration-300 group-hover:scale-110"
		/>
	</a>
</div>
