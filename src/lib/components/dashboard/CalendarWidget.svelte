<script lang="ts">
	import { Card } from '$lib/components/ui/card';
	import Icon from '@iconify/svelte';

	let currentTime = $state(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }));
	let currentDate = $state(new Date());

	$effect(() => {
		const interval = setInterval(() => {
			const now = new Date();
			currentTime = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
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
	<Card class="flex flex-row gap-2 flex-nowrap border-border/50 text-5xl p-4 bg-accent/20 backdrop-blur-md rounded-tl-[4rem] rounded-bl-[4rem]">
		<Icon icon="solar:clock-circle-bold" />
		<span class="tracking-wide font-bold">
			{currentTime}
		</span>
	</Card>

	<Card class="w-fit p-4 border-border/50 bg-gradient-to-br from-primary/15 to-primary/25 backdrop-blur-md">
		<div class="flex items-center justify-between">
			<!-- Date Section -->
			<div class="flex flex-col items-center justify-center px-2.5">
				<p class="text-[10px] font-semibold tracking-wider text-muted-foreground/80 mb-0.5">
					{currentDate.toLocaleDateString('en-US', { month: 'short' }).toUpperCase()}
				</p>
				<p class="text-4xl font-bold tracking-tight leading-none mb-0.5">{currentDate.getDate()}</p>
				<p class="text-[10px] font-semibold tracking-wider text-muted-foreground/80 mt-0.5">
					{getDayOfWeek(currentDate).toUpperCase()}
				</p>
			</div>
		</div>
	</Card>
</div>
