<script lang="ts">
	import { useThemeState } from '$lib/stores/theme.svelte';
	let themeState = useThemeState();
</script>

<div
	class="inline-flex items-center gap-2 rounded-full border border-primary/30 bg-gradient-to-r from-primary/20 to-primary/10 px-3 py-1.5"
>
	<div class="h-2 w-2 animate-pulse rounded-full bg-primary"></div>
	<span class="text-sm font-semibold capitalize">{themeState.currentTheme?.name}</span>
</div>
