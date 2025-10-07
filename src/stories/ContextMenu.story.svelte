<script lang="ts">
	import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { useContextMenu } from '$lib/hooks/useContextMenu';
	import { toast } from 'svelte-sonner';

	interface Props {
		variant?: 'default' | 'icons' | 'shortcuts' | 'disabled' | 'complex';
	}

	let { variant = 'default' }: Props = $props();

	// Default variant
	const defaultItems = [
		{
			id: 'option1',
			label: 'Option 1',
			onClick: () => {
				toast.success('Option 1 clicked!');
			}
		},
		{
			id: 'option2',
			label: 'Option 2',
			onClick: () => {
				toast.success('Option 2 clicked!');
			}
		},
		{
			id: 'option3',
			label: 'Option 3',
			onClick: () => {
				toast.success('Option 3 clicked!');
			}
		}
	];

	// Icons variant
	const iconItems = [
		{
			id: 'copy',
			label: 'Copy',
			icon: 'solar:copy-bold',
			onClick: () => {
				toast.success('Copy clicked!');
			}
		},
		{
			id: 'paste',
			label: 'Paste',
			icon: 'solar:clipboard-bold',
			onClick: () => {
				toast.success('Paste clicked!');
			}
		},
		{
			id: 'delete',
			label: 'Delete',
			icon: 'solar:trash-bin-trash-bold',
			onClick: () => {
				toast.error('Delete clicked!');
			}
		}
	];

	// Shortcuts variant
	const shortcutItems = [
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
			id: 'cut',
			label: 'Cut',
			icon: 'solar:scissors-bold',
			shortcut: '⌘X',
			onClick: () => {
				toast.info('Cut clicked!');
			}
		}
	];

	// Disabled variant
	const disabledItems = [
		{
			id: 'enabled',
			label: 'Enabled Item',
			icon: 'solar:check-circle-bold',
			onClick: () => {
				toast.success('Enabled clicked!');
			}
		},
		{
			id: 'disabled',
			label: 'Disabled Item',
			icon: 'solar:close-circle-bold',
			disabled: true,
			onClick: () => {}
		},
		{
			id: 'another',
			label: 'Another Item',
			icon: 'solar:star-shine-bold',
			onClick: () => {
				toast.success('Another clicked!');
			}
		}
	];

	// Complex variant
	const complexItems = [
		{
			id: 'view',
			label: 'View Details',
			icon: 'solar:eye-bold',
			shortcut: '⌘I',
			onClick: () => {
				toast.info('Viewing details...');
			}
		},
		{
			id: 'edit',
			label: 'Edit',
			icon: 'solar:pen-bold',
			shortcut: '⌘E',
			onClick: () => {
				toast.info('Opening editor...');
			}
		},
		{
			id: 'sep1',
			label: '',
			separator: true
		},
		{
			id: 'share',
			label: 'Share',
			icon: 'solar:share-bold',
			shortcut: '⌘S',
			onClick: () => {
				toast.info('Opening share dialog...');
			}
		},
		{
			id: 'download',
			label: 'Download',
			icon: 'solar:download-bold',
			disabled: true,
			onClick: () => {}
		},
		{
			id: 'sep2',
			label: '',
			separator: true
		},
		{
			id: 'delete',
			label: 'Delete',
			icon: 'solar:trash-bin-trash-bold',
			shortcut: 'Del',
			onClick: () => {
				toast.error('Deleting...');
			}
		}
	];

	// Select items based on variant - use a function to make it reactive
	const getItems = () => {
		switch (variant) {
			case 'icons':
				return iconItems;
			case 'shortcuts':
				return shortcutItems;
			case 'disabled':
				return disabledItems;
			case 'complex':
				return complexItems;
			default:
				return defaultItems;
		}
	};

	const { onContextMenu } = useContextMenu(getItems);

	const descriptions = {
		default: 'Basic context menu with simple text items',
		icons: 'Context menu with Solar icons from Iconify',
		shortcuts: 'Context menu with keyboard shortcuts',
		disabled: 'Context menu with disabled items',
		complex: 'Complete example with icons, shortcuts, separators, and disabled items'
	};
</script>

<div class="context-menu-story">
	<Card class="w-[400px]" style="cursor: context-menu;" oncontextmenu={onContextMenu} data-has-context-menu>
		<CardHeader>
			<CardTitle>Right-click here</CardTitle>
			<CardDescription>{descriptions[variant]}</CardDescription>
		</CardHeader>
		<CardContent>
			<div class="p-8 border-2 border-dashed border-muted-foreground/25 rounded-lg text-center">
				<p class="text-muted-foreground text-sm">
					Right-click anywhere in this card to open the context menu
				</p>
			</div>
		</CardContent>
	</Card>
</div>

<style>
	.context-menu-story {
		padding: 2rem;
		min-height: 400px;
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
