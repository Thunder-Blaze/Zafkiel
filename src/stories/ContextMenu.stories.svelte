<script lang="ts">
	import { Story, Template } from '@storybook/addon-svelte-csf';
	import ContextMenuStory from './ContextMenu.story.svelte';

	const meta = {
		title: 'Components/ContextMenu',
		component: ContextMenuStory,
		tags: ['autodocs'],
		parameters: {
			layout: 'padded',
			docs: {
				description: {
					component:
						'A custom right-click context menu component built with shadcn-svelte styling. Supports icons, shortcuts, separators, and disabled states.',
				},
			},
		},
		argTypes: {
			variant: {
				control: 'select',
				options: ['default', 'icons', 'shortcuts', 'disabled', 'complex'],
				description: 'The variant of the context menu to display',
			},
		},
	};
</script>

<Story name="Default" args={{ variant: 'default' }}>
	<ContextMenuStory variant="default" />
</Story>

<Story name="With Icons" args={{ variant: 'icons' }}>
	<ContextMenuStory variant="icons" />
</Story>

<Story name="With Shortcuts" args={{ variant: 'shortcuts' }}>
	<ContextMenuStory variant="shortcuts" />
</Story>

<Story name="With Disabled Items" args={{ variant: 'disabled' }}>
	<ContextMenuStory variant="disabled" />
</Story>

<Story name="Complex" args={{ variant: 'complex' }}>
	<ContextMenuStory variant="complex" />
</Story>
