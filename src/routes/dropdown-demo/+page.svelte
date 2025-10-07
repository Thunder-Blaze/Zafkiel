<script lang="ts">
	import {
		Card,
		CardContent,
		CardDescription,
		CardHeader,
		CardTitle,
	} from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import { useUiScale } from '$lib/hooks/useUiScale.svelte';
	import Icon from '@iconify/svelte';

	const uiScale = useUiScale();
	let sliderValue = $state(uiScale.scale * 100);
	let selectedFramework = $state('sveltekit');
	let selectedSize = $state('');

	$effect(() => {
		sliderValue = uiScale.scale * 100;
	});

	function handleScaleChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const scale = Number(target.value) / 100;
		sliderValue = Number(target.value);

		clearTimeout(scaleUpdateTimeout);
		scaleUpdateTimeout = setTimeout(async () => {
			await uiScale.setScale(scale);
		}, 300);
	}

	let scaleUpdateTimeout: ReturnType<typeof setTimeout>;

	const frameworks = [
		{ value: 'sveltekit', label: 'SvelteKit' },
		{ value: 'react', label: 'React' },
		{ value: 'vue', label: 'Vue' },
		{ value: 'angular', label: 'Angular' },
		{ value: 'nextjs', label: 'Next.js' },
		{ value: 'nuxt', label: 'Nuxt' },
	];

	const sizes = [
		{ value: 'xs', label: 'Extra Small' },
		{ value: 'sm', label: 'Small' },
		{ value: 'md', label: 'Medium' },
		{ value: 'lg', label: 'Large' },
		{ value: 'xl', label: 'Extra Large' },
	];

	const frameworkTriggerContent = $derived(
		frameworks.find((f) => f.value === selectedFramework)?.label ?? 'Select a framework'
	);

	const sizeTriggerContent = $derived(
		sizes.find((s) => s.value === selectedSize)?.label ?? 'Select size'
	);
</script>

<div class="container mx-auto p-8">
	<div class="mb-8">
		<h1 class="mb-2 text-4xl font-bold">Dropdown & Select Demo</h1>
		<p class="text-muted-foreground">
			Test dropdown positioning and visual effects at different UI scales
		</p>
	</div>

	<div class="mb-8 grid gap-6 md:grid-cols-2">
		<!-- UI Scale Control -->
		<Card>
			<CardHeader>
				<CardTitle>UI Scale: {sliderValue.toFixed(0)}%</CardTitle>
				<CardDescription>Adjust to test dropdown positioning</CardDescription>
			</CardHeader>
			<CardContent>
				<input
					type="range"
					value={sliderValue}
					oninput={handleScaleChange}
					min="50"
					max="200"
					step="5"
					class="h-2 w-full cursor-pointer appearance-none rounded-lg bg-secondary accent-primary"
				/>
				<div class="mt-2 flex justify-between text-xs text-muted-foreground">
					<span>50%</span>
					<span>100%</span>
					<span>200%</span>
				</div>
			</CardContent>
		</Card>

		<!-- Dropdown Menu Demo -->
		<Card>
			<CardHeader>
				<CardTitle>Dropdown Menu</CardTitle>
				<CardDescription>Test blur effects and positioning</CardDescription>
			</CardHeader>
			<CardContent class="flex flex-wrap gap-4">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button variant="outline">
							<Icon icon="solar:menu-dots-bold" class="size-4" />
							Actions
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-56">
						<DropdownMenu.Label>My Account</DropdownMenu.Label>
						<DropdownMenu.Separator />
						<DropdownMenu.Group>
							<DropdownMenu.Item>
								<Icon icon="solar:user-bold" class="size-4" />
								<span>Profile</span>
								<DropdownMenu.Shortcut>⇧⌘P</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
							<DropdownMenu.Item>
								<Icon icon="solar:settings-bold" class="size-4" />
								<span>Settings</span>
								<DropdownMenu.Shortcut>⌘S</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
							<DropdownMenu.Item>
								<Icon icon="solar:keyboard-bold" class="size-4" />
								<span>Keyboard shortcuts</span>
								<DropdownMenu.Shortcut>⌘K</DropdownMenu.Shortcut>
							</DropdownMenu.Item>
						</DropdownMenu.Group>
						<DropdownMenu.Separator />
						<DropdownMenu.Group>
							<DropdownMenu.Item>
								<Icon icon="solar:users-group-rounded-bold" class="size-4" />
								<span>Team</span>
							</DropdownMenu.Item>
							<DropdownMenu.Sub>
								<DropdownMenu.SubTrigger>
									<Icon icon="solar:user-plus-bold" class="size-4" />
									<span>Invite users</span>
								</DropdownMenu.SubTrigger>
								<DropdownMenu.SubContent>
									<DropdownMenu.Item>
										<Icon icon="solar:letter-bold" class="size-4" />
										<span>Email</span>
									</DropdownMenu.Item>
									<DropdownMenu.Item>
										<Icon icon="solar:chat-round-bold" class="size-4" />
										<span>Message</span>
									</DropdownMenu.Item>
									<DropdownMenu.Separator />
									<DropdownMenu.Item>
										<Icon icon="solar:add-circle-bold" class="size-4" />
										<span>More...</span>
									</DropdownMenu.Item>
								</DropdownMenu.SubContent>
							</DropdownMenu.Sub>
						</DropdownMenu.Group>
						<DropdownMenu.Separator />
						<DropdownMenu.Item>
							<Icon icon="solar:help-bold" class="size-4" />
							<span>Support</span>
						</DropdownMenu.Item>
						<DropdownMenu.Item disabled>
							<Icon icon="solar:cloud-bold" class="size-4" />
							<span>API (Coming soon)</span>
						</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item variant="destructive">
							<Icon icon="solar:logout-2-bold" class="size-4" />
							<span>Log out</span>
							<DropdownMenu.Shortcut>⇧⌘Q</DropdownMenu.Shortcut>
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>

				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button>
							<Icon icon="solar:widget-5-bold" class="size-4" />
							Options
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content>
						<DropdownMenu.Item>
							<Icon icon="solar:refresh-bold" class="size-4" />
							Refresh
						</DropdownMenu.Item>
						<DropdownMenu.Item>
							<Icon icon="solar:copy-bold" class="size-4" />
							Duplicate
						</DropdownMenu.Item>
						<DropdownMenu.Item>
							<Icon icon="solar:share-bold" class="size-4" />
							Share
						</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item variant="destructive">
							<Icon icon="solar:trash-bin-trash-bold" class="size-4" />
							Delete
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</CardContent>
		</Card>

		<!-- Select Demo -->
		<Card class="md:col-span-2">
			<CardHeader>
				<CardTitle>Select Component</CardTitle>
				<CardDescription>Test select dropdown with blur effects</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="flex flex-wrap items-end gap-4">
					<div class="min-w-[200px] flex-1 space-y-2">
						<label for="framework-select" class="text-sm font-medium"> Choose a framework </label>
						<Select.Root type="single" bind:value={selectedFramework} items={frameworks}>
							<Select.Trigger id="framework-select" class="w-full">
								{frameworks.find((f) => f.value === selectedFramework)?.label ||
									'Select a framework'}
							</Select.Trigger>
							<Select.Content>
								<Select.Group>
									<Select.Label>Frameworks</Select.Label>
									{#each frameworks as framework}
										<Select.Item value={framework.value} label={framework.label}>
											{framework.label}
										</Select.Item>
									{/each}
								</Select.Group>
							</Select.Content>
						</Select.Root>
					</div>

					<div class="min-w-[200px] flex-1 space-y-2">
						<label for="size-select" class="text-sm font-medium"> Choose a size </label>
						<Select.Root type="single">
							<Select.Trigger id="size-select" class="w-full">Select size</Select.Trigger>
							<Select.Content>
								<Select.Group>
									<Select.Label>Sizes</Select.Label>
									<Select.Item value="xs" label="Extra Small">Extra Small</Select.Item>
									<Select.Item value="sm" label="Small">Small</Select.Item>
									<Select.Item value="md" label="Medium">Medium</Select.Item>
									<Select.Item value="lg" label="Large">Large</Select.Item>
									<Select.Item value="xl" label="Extra Large">Extra Large</Select.Item>
								</Select.Group>
							</Select.Content>
						</Select.Root>
					</div>
				</div>

				{#if selectedFramework}
					<div class="mt-4 rounded-lg bg-muted p-4">
						<p class="text-sm">
							<span class="font-medium">Selected:</span>
							{frameworks.find((f) => f.value === selectedFramework)?.label}
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>

	<!-- Test Positions -->
	<Card>
		<CardHeader>
			<CardTitle>Position Testing</CardTitle>
			<CardDescription>Test dropdowns at different positions on the screen</CardDescription>
		</CardHeader>
		<CardContent>
			<div class="grid min-h-[400px] grid-cols-3 gap-4">
				<!-- Top Left -->
				<div class="flex items-start justify-start">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Top Left</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Top Center -->
				<div class="flex items-start justify-center">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Top Center</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Top Right -->
				<div class="flex items-start justify-end">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Top Right</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Middle Left -->
				<div class="flex items-center justify-start">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Middle Left</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Center -->
				<div class="flex items-center justify-center">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Center</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Middle Right -->
				<div class="flex items-center justify-end">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Middle Right</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Bottom Left -->
				<div class="flex items-end justify-start">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Bottom Left</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Bottom Center -->
				<div class="flex items-end justify-center">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Bottom Center</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Bottom Right -->
				<div class="flex items-end justify-end">
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" size="sm">Bottom Right</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content>
							<DropdownMenu.Item>Item 1</DropdownMenu.Item>
							<DropdownMenu.Item>Item 2</DropdownMenu.Item>
							<DropdownMenu.Item>Item 3</DropdownMenu.Item>
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>
			</div>
		</CardContent>
	</Card>

	<div class="mt-8">
		<Button href="/" variant="outline">← Back to Home</Button>
	</div>
</div>
