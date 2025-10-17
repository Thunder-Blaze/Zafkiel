<script lang="ts">
	import { page } from '$app/stores';
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { User } from 'lucide-svelte';

	const staffId = $derived($page.params.id ? parseInt($page.params.id) : 0);
</script>

<svelte:head>
	<title>Staff #{staffId} - Zafkiel</title>
	<meta name="description" content="Staff details on Zafkiel" />
</svelte:head>

<div class="container mx-auto px-4 py-8 max-w-7xl">
	<Card>
		<CardContent class="pt-6">
			<div class="text-center space-y-4">
				<User class="w-16 h-16 mx-auto text-muted-foreground" />
				<h2 class="text-2xl font-bold">Staff Details Coming Soon</h2>
				<p class="text-muted-foreground">
					Staff page for ID #{staffId} is under development.
				</p>
				<Button variant="outline" onclick={() => history.back()}>
					Go Back
				</Button>
			</div>
		</CardContent>
	</Card>
</div>
