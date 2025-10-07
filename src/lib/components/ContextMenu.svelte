<script lang="ts">
	import { contextMenuStore, type ContextMenuItem } from '$lib/stores/context-menu';
	import { onMount } from 'svelte';
	import { fade, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import Icon from '@iconify/svelte';

	let menuState = $state({ isOpen: false, x: 0, y: 0, items: [] as ContextMenuItem[] });
	let menuElement: HTMLDivElement | null = $state(null);

	// Subscribe to context menu state
	$effect(() => {
		const unsubscribe = contextMenuStore.subscribe((state) => {
			menuState = state;
		});

		return () => {
			unsubscribe();
		};
	});

	// Handle click outside to close menu
	$effect(() => {
		if (menuState.isOpen) {
			const handleClickOutside = (e: MouseEvent) => {
				if (menuElement && !menuElement.contains(e.target as Node)) {
					contextMenuStore.close();
				}
			};

			const handleEscape = (e: KeyboardEvent) => {
				if (e.key === 'Escape') {
					contextMenuStore.close();
				}
			};

			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleEscape);

			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleEscape);
			};
		}
	});

	// Adjust menu position if it goes off-screen
	$effect(() => {
		if (menuState.isOpen && menuElement) {
			const rect = menuElement.getBoundingClientRect();
			const viewportWidth = window.innerWidth;
			const viewportHeight = window.innerHeight;

			let adjustedX = menuState.x;
			let adjustedY = menuState.y;

			// Adjust horizontal position
			if (rect.right > viewportWidth) {
				adjustedX = viewportWidth - rect.width - 10;
			}

			// Adjust vertical position
			if (rect.bottom > viewportHeight) {
				adjustedY = viewportHeight - rect.height - 10;
			}

			if (adjustedX !== menuState.x || adjustedY !== menuState.y) {
				menuElement.style.left = `${adjustedX}px`;
				menuElement.style.top = `${adjustedY}px`;
			}
		}
	});

	const handleItemClick = async (item: ContextMenuItem) => {
		if (item.disabled) return;
		await contextMenuStore.handleItemClick(item);
	};

	// Check if icon is an Iconify icon (starts with a prefix like "solar:")
	const isIconifyIcon = (icon: string | undefined) => {
		return icon && icon.includes(':');
	};
</script>

{#if menuState.isOpen}
	<div
		bind:this={menuElement}
		class="pointer-events-auto fixed z-[9999] min-w-[200px]"
		style="left: {menuState.x}px; top: {menuState.y}px;"
		transition:scale={{ duration: 150, easing: quintOut, start: 0.95 }}
		role="menu"
		tabindex="-1"
	>
		<div
			class="animate-fade-in rounded-lg border bg-popover/95 p-1 text-popover-foreground shadow-2xl ring-1 shadow-black/20 ring-black/5 [backdrop-filter:blur(24px)] [-webkit-backdrop-filter:blur(24px)] dark:shadow-black/40"
		>
			{#each menuState.items as item (item.id)}
				{#if item.separator}
					<div class="-mx-1 my-1 h-px bg-border" role="separator"></div>
				{:else}
					<button
						class="relative flex w-full cursor-pointer items-center gap-2 rounded-md px-3 py-2 text-sm transition-colors outline-none select-none hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground disabled:pointer-events-none disabled:opacity-50"
						class:cursor-not-allowed={item.disabled}
						class:opacity-50={item.disabled}
						disabled={item.disabled}
						onclick={() => handleItemClick(item)}
						role="menuitem"
						tabindex={item.disabled ? -1 : 0}
					>
						{#if item.icon}
							<span class="h-4 w-4 flex-shrink-0">
								{#if isIconifyIcon(item.icon)}
									<Icon icon={item.icon} class="h-4 w-4" />
								{:else}
									{@html item.icon}
								{/if}
							</span>
						{/if}
						<span class="flex-1 text-left">{item.label}</span>
						{#if item.shortcut}
							<span class="ml-auto text-xs tracking-widest text-muted-foreground"
								>{item.shortcut}</span
							>
						{/if}
					</button>
				{/if}
			{/each}
		</div>
	</div>
{/if}

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	.animate-fade-in {
		animation: fade-in 150ms ease-out;
	}
</style>
