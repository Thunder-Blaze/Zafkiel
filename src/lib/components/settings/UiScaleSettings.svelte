<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { toast } from 'svelte-sonner';
	import Icon from '@iconify/svelte';
	import { slide } from 'svelte/transition';
	
	const uiScale = useUiScale();
	let sliderValue = $state(uiScale.scale * 100);
	let scaleUpdateTimeout: ReturnType<typeof setTimeout>;
	
	// Update slider when scale changes
	$effect(() => {
		sliderValue = uiScale.scale * 100;
	});
	
	function handleScaleChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const scale = Number(target.value) / 100;
		sliderValue = Number(target.value);

		clearTimeout(scaleUpdateTimeout);
		scaleUpdateTimeout = setTimeout(async () => {
			try {
				await uiScale.setScale(scale);
				toast.success(`UI Scale set to ${scale.toFixed(2)}x`);
			} catch (error) {
				toast.error('Failed to update UI scale');
			}
		}, 300);
	}

	async function resetScale(): Promise<void> {
		try {
			await uiScale.resetScale();
			toast.success('UI Scale reset to 1.0x');
		} catch (error) {
			toast.error('Failed to reset UI scale');
		}
	}
</script>

<div transition:slide={{ duration: 300 }}>
	<Card>
		<CardHeader>
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-xl bg-primary/10">
						<Icon icon="solar:maximize-square-bold" class="h-5 w-5 text-primary" />
					</div>
				<div>
					<CardTitle>UI Scale</CardTitle>
					<CardDescription>Adjust interface size</CardDescription>
				</div>
			</div>
			<div class="flex items-center gap-2">
				<span class="rounded-lg bg-primary/10 px-3 py-1.5 text-lg font-medium">
					{sliderValue.toFixed(0)}%
				</span>
				<Button variant="outline" size="sm" onclick={resetScale}>
					<Icon icon="solar:restart-bold" class="mr-2 h-4 w-4" />
					Reset
				</Button>
			</div>
		</div>
	</CardHeader>
	<CardContent class="space-y-4 pb-8">
		<div class="space-y-3">
			<div class="flex items-center justify-between text-sm text-foreground/70">
				<span>50%</span>
				<span>200%</span>
			</div>
			<input
				type="range"
				min="50"
				max="200"
				step="5"
				value={sliderValue}
				oninput={handleScaleChange}
				class="h-3 w-full cursor-pointer appearance-none rounded-lg border border-border bg-accent transition-all focus:outline-none focus:ring-2 focus:ring-primary/50 [&::-moz-range-thumb]:h-6 [&::-moz-range-thumb]:w-6 [&::-moz-range-thumb]:appearance-none [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:border-2 [&::-moz-range-thumb]:border-primary [&::-moz-range-thumb]:bg-background [&::-moz-range-thumb]:shadow-lg [&::-moz-range-thumb]:transition-all [&::-moz-range-thumb]:hover:scale-110 [&::-webkit-slider-thumb]:h-6 [&::-webkit-slider-thumb]:w-6 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:border-2 [&::-webkit-slider-thumb]:border-primary [&::-webkit-slider-thumb]:bg-background [&::-webkit-slider-thumb]:shadow-lg [&::-webkit-slider-thumb]:transition-all [&::-webkit-slider-thumb]:hover:scale-110"
			/>
		</div>
		<p class="text-center text-sm text-foreground/70">
			Drag the slider to adjust the overall size of the interface
		</p>
	</CardContent>
</Card>
</div>
