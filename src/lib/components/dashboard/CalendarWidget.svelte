<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import Icon from '@iconify/svelte';

	let currentTime = $state(
		new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false })
	);
	let currentDate = $state(new Date());

	$effect(() => {
		const interval = setInterval(() => {
			const now = new Date();
			currentTime = now.toLocaleTimeString([], {
				hour: '2-digit',
				minute: '2-digit',
				hour12: false,
			});
			currentDate = now;
		}, 1000);

		return () => clearInterval(interval);
	});

	const getDayOfWeek = (date: Date) => {
		return date.toLocaleDateString('en-US', { weekday: 'short' });
	};
</script>

<div class="relative flex items-end gap-4">
	<!-- Time Section -->
	<Card
		class="flex flex-row flex-nowrap gap-2 rounded-tl-[4rem] rounded-bl-[4rem] border-border/50 bg-accent/20 p-4 text-5xl backdrop-blur-md"
	>
		<Icon icon="solar:clock-circle-bold" />
		<span class="font-bold tracking-wide">
			{currentTime}
		</span>
	</Card>

	<Card
		class="w-fit border-border/50 bg-gradient-to-br from-primary/15 to-primary/25 p-4 backdrop-blur-md"
	>
		<div class="flex items-center justify-between">
			<!-- Date Section -->
			<div class="flex flex-col items-center justify-center px-2.5">
				<p class="mb-0.5 text-[10px] font-semibold tracking-wider text-muted-foreground/80">
					{currentDate.toLocaleDateString('en-US', { month: 'short' }).toUpperCase()}
				</p>
				<p class="mb-0.5 text-4xl leading-none font-bold tracking-tight">{currentDate.getDate()}</p>
				<p class="mt-0.5 text-[10px] font-semibold tracking-wider text-muted-foreground/80">
					{getDayOfWeek(currentDate).toUpperCase()}
				</p>
			</div>
		</div>
	</Card>
</div>
