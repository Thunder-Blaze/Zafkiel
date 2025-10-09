<script lang="ts">
	import { Story } from '@storybook/addon-svelte-csf';
	import ProfileDropdown from '$lib/components/ProfileDropdown.svelte';
</script>

<Story 
	name="Guest User"
	parameters={{
		layout: 'centered'
	}}
>
	<div class="p-8 bg-background">
		<ProfileDropdown />
	</div>
</Story>

<Story 
	name="Logged In User"
	parameters={{
		layout: 'centered'
	}}
>
	<div class="p-8 bg-background">
		<p class="text-sm text-muted-foreground mb-4">
			Note: The dropdown shows user info when authenticated. In Storybook, it defaults to Guest user.
		</p>
		<ProfileDropdown />
	</div>
</Story>

<Story 
	name="In Title Bar Context"
	parameters={{
		layout: 'fullscreen'
	}}
>
	<div class="h-screen bg-background">
		<div class="flex h-12 items-center justify-between border-b border-border/50 bg-background/95 backdrop-blur-xl px-4">
			<div class="flex items-center gap-2">
				<div class="h-6 w-6 rounded bg-primary/20"></div>
				<span class="text-sm font-semibold">Zafkiel</span>
			</div>
			
			<div class="flex items-center gap-4">
				<button class="text-sm text-muted-foreground hover:text-foreground">Theme</button>
				<ProfileDropdown />
				<div class="flex gap-1">
					<button class="h-8 w-8 rounded hover:bg-foreground/5">−</button>
					<button class="h-8 w-8 rounded hover:bg-foreground/5">□</button>
					<button class="h-8 w-8 rounded hover:bg-red-500/10">×</button>
				</div>
			</div>
		</div>
		
		<div class="p-8">
			<h2 class="text-xl font-semibold mb-2">Profile Dropdown in Title Bar</h2>
			<p class="text-sm text-muted-foreground">
				The profile dropdown appears in the title bar between the theme switcher and window controls.
				It shows the user's avatar (or a fallback icon), username, and provides quick access to profile, settings, and logout.
			</p>
		</div>
	</div>
</Story>
