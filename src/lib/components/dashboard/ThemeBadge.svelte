<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
</script>

<div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-gradient-to-r from-primary/20 to-primary/10 border border-primary/30">
	<div class="h-2 w-2 rounded-full bg-primary animate-pulse"></div>
	<span class="text-sm font-semibold capitalize">{themeStore.currentTheme}</span>
</div>
