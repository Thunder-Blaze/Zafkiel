<script lang="ts">
	import Icon from "@iconify/svelte";

	const { text = "Loading..." }: { text: string } = $props();
</script>

<div class="flex min-h-screen items-center justify-center">
		<div class="space-y-4 text-center">
			<Icon
				icon="solar:refresh-circle-line-duotone"
				class="mx-auto h-12 w-12 animate-spin text-primary"
			/>
			<p class="text-muted-foreground">{text}</p>
		</div>
	</div>
