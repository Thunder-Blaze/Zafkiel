<script lang="ts">
	import { Story } from '@storybook/addon-svelte-csf';
	import TitleBar from '$lib/components/TitleBar.svelte';
</script>

<Story 
	name="Default"
	parameters={{
		layout: 'fullscreen'
	}}
>
	<div class="h-screen bg-background">
		<TitleBar />
		<div class="container mx-auto p-8">
			<h1 class="text-3xl font-bold mb-4">Title Bar Demo</h1>
			<p class="text-muted-foreground mb-4">
				This is the custom title bar that replaces the default Tauri window chrome.
			</p>
			
			<div class="space-y-4 text-sm">
				<div class="rounded-lg border border-border/50 bg-muted/30 p-4">
					<h3 class="font-semibold mb-2">Features:</h3>
					<ul class="space-y-2 text-muted-foreground">
						<li>• <strong>Draggable Area:</strong> Click and drag the title bar to move the window</li>
						<li>• <strong>App Branding:</strong> Zafkiel logo and name with Alpha badge</li>
						<li>• <strong>Navigation:</strong> Quick access to Home, Anime, Demo, and Settings</li>
						<li>• <strong>Theme Switcher:</strong> Button to open theme selection sheet</li>
						<li>• <strong>Window Controls:</strong> Minimize, Maximize/Restore, and Close buttons with Solar icons</li>
						<li>• <strong>Active State:</strong> Current page is highlighted in navigation</li>
						<li>• <strong>Backdrop Blur:</strong> Semi-transparent with blur effect</li>
					</ul>
				</div>

				<div class="rounded-lg border border-amber-500/20 bg-amber-500/5 p-4">
					<h3 class="font-semibold mb-2 text-amber-500">Note:</h3>
					<p class="text-sm text-muted-foreground">
						In Storybook, the window control buttons won't function as they require the Tauri runtime.
						The title bar will work fully when running the actual application. The "ALPHA" badge indicates early development status.
					</p>
				</div>
			</div>
		</div>
	</div>
</Story>

<Story 
	name="With Content"
	parameters={{
		layout: 'fullscreen'
	}}
>
	<div class="h-screen bg-background">
		<TitleBar />
		<div class="container mx-auto p-8">
			<h1 class="text-4xl font-bold mb-2">Welcome to Zafkiel</h1>
			<p class="text-xl text-muted-foreground mb-8">Your anime watching companion</p>
			
			<div class="grid gap-6 md:grid-cols-3">
				{#each Array(6) as _, i}
					<div class="rounded-xl border border-border/50 bg-card p-6">
						<div class="mb-4 h-32 rounded-lg bg-gradient-to-br from-primary/20 to-accent/20"></div>
						<h3 class="font-semibold mb-2">Card {i + 1}</h3>
						<p class="text-sm text-muted-foreground">Sample content to show how the title bar looks with real UI elements below it.</p>
					</div>
				{/each}
			</div>
		</div>
	</div>
</Story>
