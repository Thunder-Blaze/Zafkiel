<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import { toast } from 'svelte-sonner';

	const uiScale = useUiScale();
	let sliderValue = $state(uiScale.scale * 100);

	// Update slider when scale changes
	$effect(() => {
		sliderValue = uiScale.scale * 100;
	});

	function handleScaleChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const scale = Number(target.value) / 100;
		sliderValue = Number(target.value);

		// Debounce the actual config update
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

	let scaleUpdateTimeout: ReturnType<typeof setTimeout>;

	async function resetScale(): Promise<void> {
		try {
			await uiScale.resetScale();
			toast.success('UI Scale reset to 1.0x');
		} catch (error) {
			toast.error('Failed to reset UI scale');
		}
	}
</script>

<div class="container mx-auto p-8">
	<h1 class="mb-8 text-4xl font-bold">Welcome to Zafkiel</h1>

	<div class="mb-8 grid gap-6 md:grid-cols-2">
		<!-- UI Scale Control -->
		<Card>
			<CardHeader>
				<CardTitle>UI Scale Control</CardTitle>
				<CardDescription>Adjust the overall size of the UI elements (50% - 200%)</CardDescription>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<div class="flex items-center justify-between">
						<span class="text-sm font-medium">Current Scale:</span>
						<span class="text-2xl font-bold">{sliderValue.toFixed(0)}%</span>
					</div>
					<input
						type="range"
						value={sliderValue}
						oninput={handleScaleChange}
						min="50"
						max="200"
						step="5"
						class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-secondary accent-primary"
					/>
					<div class="flex justify-between text-xs text-muted-foreground">
						<span>50%</span>
						<span>100%</span>
						<span>200%</span>
					</div>
				</div>
				<Button onclick={resetScale} variant="outline" class="w-full">
					Reset to Default (100%)
				</Button>
			</CardContent>
		</Card>

		<!-- Context Menu Info -->
		<Card>
			<CardHeader>
				<CardTitle>Custom Context Menu</CardTitle>
				<CardDescription>Right-click anywhere to try the custom context menu</CardDescription>
			</CardHeader>
			<CardContent>
				<ul class="space-y-2 text-sm">
					<li class="flex items-center gap-2">
						<span class="text-muted-foreground">•</span>
						<span>Reload page</span>
					</li>
					<li class="flex items-center gap-2">
						<span class="text-muted-foreground">•</span>
						<span>Navigate back/forward</span>
					</li>
					<li class="flex items-center gap-2">
						<span class="text-muted-foreground">•</span>
						<span>Inspect Element (opens devtools)</span>
					</li>
				</ul>
			</CardContent>
		</Card>
	</div>

	<!-- Navigation Links -->
	<div class="mb-8 flex flex-wrap gap-4">
		<Button href="/demo" variant="default">Go to Demo</Button>
		<Button href="/dropdown-demo" variant="default">Dropdown Demo</Button>
		<Button href="/config-demo" variant="outline">Go to Config Demo</Button>
		<Button href="/context-menu-demo" variant="outline">Go to Context Menu Demo</Button>
	</div>

	<!-- Sample Content -->
	<Card>
		<CardHeader>
			<CardTitle>Sample Content</CardTitle>
			<CardDescription>Scroll to see how the UI scale affects all elements</CardDescription>
		</CardHeader>
		<CardContent>
			<p class="text-sm leading-relaxed">
				Lorem ipsum dolor sit amet consectetur adipisicing elit. Nulla qui repudiandae maiores eum
				fuga iure voluptatum ex laudantium ducimus doloribus, architecto nihil inventore numquam ut
				accusantium, natus nesciunt non earum esse quaerat incidunt amet voluptatem. Doloremque
				eveniet ad fuga magni ipsum veritatis quidem aspernatur. Magni reiciendis, esse sint ipsa
				exercitationem, provident iure totam itaque at architecto est corporis sed laboriosam minima
				amet ipsam eligendi quae accusantium. Id nemo illum voluptatibus debitis! Expedita accusamus
				sint, qui rem eos enim non fugiat quas molestias nemo fuga provident ad libero quisquam
				quidem eum odio?
			</p>
		</CardContent>
	</Card>
</div>
