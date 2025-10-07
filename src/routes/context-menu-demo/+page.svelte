<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { useContextMenu } from '$lib/hooks/useContextMenu';
	import { toast } from 'svelte-sonner';

	// Example 1: Simple context menu
	const simpleMenuItems = [
		{
			id: 'copy',
			label: 'Copy',
			icon: 'solar:copy-bold',
			shortcut: '⌘C',
			onClick: () => {
				toast.success('Copy clicked!');
			}
		},
		{
			id: 'paste',
			label: 'Paste',
			icon: 'solar:clipboard-bold',
			shortcut: '⌘V',
			onClick: () => {
				toast.success('Paste clicked!');
			}
		},
		{
			id: 'sep1',
			label: '',
			separator: true
		},
		{
			id: 'delete',
			label: 'Delete',
			icon: 'solar:trash-bin-trash-bold',
			shortcut: 'Del',
			onClick: () => {
				toast.error('Delete clicked!');
			}
		}
	];

	const { onContextMenu: onSimpleContextMenu } = useContextMenu(simpleMenuItems);

	// Example 2: Dynamic context menu based on anime data
	let selectedAnime = $state('One Piece');

	const animeMenuItems = [
		{
			id: 'view',
			label: 'View Details',
			icon: 'solar:eye-bold',
			onClick: () => {
				toast.info(`Viewing ${selectedAnime}`);
			}
		},
		{
			id: 'add-to-list',
			label: 'Add to List',
			icon: 'solar:add-circle-bold',
			onClick: () => {
				toast.success(`Added ${selectedAnime} to list`);
			}
		},
		{
			id: 'sep1',
			label: '',
			separator: true
		},
		{
			id: 'watching',
			label: 'Mark as Watching',
			icon: 'solar:play-circle-bold',
			onClick: () => {
				toast.success(`${selectedAnime} marked as watching`);
			}
		},
		{
			id: 'completed',
			label: 'Mark as Completed',
			icon: 'solar:check-circle-bold',
			onClick: () => {
				toast.success(`${selectedAnime} marked as completed`);
			}
		},
		{
			id: 'plan',
			label: 'Plan to Watch',
			icon: 'solar:bookmark-bold',
			onClick: () => {
				toast.info(`${selectedAnime} added to plan`);
			}
		},
		{
			id: 'sep2',
			label: '',
			separator: true
		},
		{
			id: 'share',
			label: 'Share',
			icon: 'solar:share-bold',
			shortcut: '⌘S',
			onClick: () => {
				toast.info(`Sharing ${selectedAnime}`);
			}
		},
		{
			id: 'favorite',
			label: 'Add to Favorites',
			icon: 'solar:star-bold',
			onClick: () => {
				toast.success(`${selectedAnime} added to favorites`);
			}
		}
	];

	const { onContextMenu: onAnimeContextMenu } = useContextMenu(animeMenuItems);

	// Example 3: Text selection menu
	let selectedText = $state('');

	const handleTextSelection = (e: MouseEvent) => {
		e.preventDefault();
		const selection = window.getSelection();
		const text = selection?.toString() || '';

		if (text) {
			selectedText = text;

			const textMenuItems = [
				{
					id: 'copy-text',
					label: 'Copy',
					icon: 'solar:copy-bold',
					shortcut: '⌘C',
					onClick: async () => {
						await navigator.clipboard.writeText(text);
						toast.success('Copied to clipboard!');
					}
				},
				{
					id: 'search',
					label: `Search "${text.substring(0, 20)}${text.length > 20 ? '...' : ''}"`,
					icon: 'solar:magnifer-bold',
					onClick: () => {
						window.open(`https://www.google.com/search?q=${encodeURIComponent(text)}`, '_blank');
						toast.info('Opening search...');
					}
				},
				{
					id: 'sep1',
					label: '',
					separator: true
				},
				{
					id: 'translate',
					label: 'Translate',
					icon: 'solar:global-bold',
					onClick: () => {
						toast.info(`Translating: ${text}`);
					}
				}
			];

			// Use context menu hook directly
			const menu = useContextMenu(textMenuItems);
			menu.onContextMenu(e);
		} else {
			onSimpleContextMenu(e);
		}
	};

	// Example 4: Disabled items
	const disabledMenuItems = [
		{
			id: 'enabled',
			label: 'Enabled Item',
			icon: 'solar:check-circle-bold',
			onClick: () => {
				toast.success('Enabled item clicked!');
			}
		},
		{
			id: 'disabled',
			label: 'Disabled Item',
			icon: 'solar:close-circle-bold',
			disabled: true,
			onClick: () => {
				toast.error('This should not appear');
			}
		},
		{
			id: 'sep1',
			label: '',
			separator: true
		},
		{
			id: 'another',
			label: 'Another Enabled',
			icon: 'solar:star-shine-bold',
			onClick: () => {
				toast.success('Another enabled clicked!');
			}
		}
	];

	const { onContextMenu: onDisabledContextMenu } = useContextMenu(disabledMenuItems);
</script>

<div class="container mx-auto p-8 max-w-6xl">
	<div class="mb-8">
		<h1 class="text-4xl font-bold mb-2">Custom Context Menu Demo</h1>
		<p class="text-muted-foreground">
			Right-click on any card to see custom context menus in action
		</p>
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<!-- Simple Context Menu -->
		<Card class="cursor-context-menu" oncontextmenu={onSimpleContextMenu} data-has-context-menu>
			<CardHeader>
				<CardTitle>Simple Context Menu</CardTitle>
				<CardDescription>Right-click here for basic options</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<p class="text-sm">
						This card demonstrates a simple context menu with copy, paste, and delete options.
					</p>
					<div class="flex gap-2">
						<Badge>Copy</Badge>
						<Badge>Paste</Badge>
						<Badge variant="destructive">Delete</Badge>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Anime Context Menu -->
		<Card class="cursor-context-menu" oncontextmenu={onAnimeContextMenu} data-has-context-menu>
			<CardHeader>
				<CardTitle>Anime Actions Menu</CardTitle>
				<CardDescription>Right-click for anime-specific actions</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<div class="flex items-center gap-2">
						<span class="text-2xl">🎬</span>
						<span class="font-semibold">{selectedAnime}</span>
					</div>
					<p class="text-sm text-muted-foreground">
						View details, add to list, change status, share, or favorite this anime.
					</p>
					<div class="flex gap-2 flex-wrap">
						<Badge>View</Badge>
						<Badge>Add to List</Badge>
						<Badge variant="secondary">Watching</Badge>
						<Badge variant="secondary">Completed</Badge>
						<Badge>Share</Badge>
						<Badge>Favorite</Badge>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Text Selection Menu -->
		<Card class="cursor-context-menu" oncontextmenu={handleTextSelection} data-has-context-menu>
			<CardHeader>
				<CardTitle>Text Selection Menu</CardTitle>
				<CardDescription>Select text and right-click</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<p class="text-sm select-text">
						<strong>Try selecting this text:</strong> Attack on Titan is a dark fantasy anime series
						adapted from the manga of the same name by Hajime Isayama. The series follows Eren Yeager
						and his friends in their fight against giant humanoid creatures called Titans.
					</p>
					{#if selectedText}
						<div class="p-3 bg-muted rounded">
							<p class="text-xs text-muted-foreground mb-1">Selected:</p>
							<p class="text-sm font-mono">{selectedText}</p>
						</div>
					{/if}
					<div class="flex gap-2">
						<Badge>Copy</Badge>
						<Badge>Search</Badge>
						<Badge>Translate</Badge>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Disabled Items Menu -->
		<Card class="cursor-context-menu" oncontextmenu={onDisabledContextMenu} data-has-context-menu>
			<CardHeader>
				<CardTitle>Disabled Items Demo</CardTitle>
				<CardDescription>See how disabled menu items work</CardDescription>
			</CardHeader>
			<CardContent>
				<div class="space-y-4">
					<p class="text-sm">
						This context menu includes both enabled and disabled items. Disabled items appear
						grayed out and cannot be clicked.
					</p>
					<div class="flex gap-2">
						<Badge>Enabled</Badge>
						<Badge variant="outline">Disabled</Badge>
						<Badge>Another Enabled</Badge>
					</div>
				</div>
			</CardContent>
		</Card>
	</div>

	<!-- Info Section -->
	<Card class="mt-6">
		<CardHeader>
			<CardTitle>✨ Features</CardTitle>
			<CardDescription>What makes this context menu special</CardDescription>
		</CardHeader>
		<CardContent>
			<div class="grid gap-4 md:grid-cols-2">
				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>🎨</span> Styled with shadcn-svelte
					</h3>
					<p class="text-sm text-muted-foreground">
						Uses your app's theme and design system automatically
					</p>
				</div>

				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>⚡</span> High Performance
					</h3>
					<p class="text-sm text-muted-foreground">
						Efficient rendering with Svelte 5 runes and optimized positioning
					</p>
				</div>

				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>🔧</span> Highly Customizable
					</h3>
					<p class="text-sm text-muted-foreground">
						Icons, shortcuts, separators, and disabled states
					</p>
				</div>

				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>📱</span> Smart Positioning
					</h3>
					<p class="text-sm text-muted-foreground">
						Automatically adjusts to stay within viewport bounds
					</p>
				</div>

				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>⌨️</span> Keyboard Support
					</h3>
					<p class="text-sm text-muted-foreground">
						Close with Escape key, navigate with keyboard
					</p>
				</div>

				<div class="space-y-2">
					<h3 class="font-semibold flex items-center gap-2">
						<span>🎯</span> Easy to Use
					</h3>
					<p class="text-sm text-muted-foreground">
						Simple hook-based API with TypeScript support
					</p>
				</div>
			</div>
		</CardContent>
	</Card>
</div>

<style>
	:global(.cursor-context-menu) {
		cursor: context-menu;
	}

	.select-text {
		user-select: text;
		-webkit-user-select: text;
		-moz-user-select: text;
		-ms-user-select: text;
	}
</style>
