<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  import { Button } from '$lib/components/ui/button';
  import Icon from '@iconify/svelte';
  import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte';
  import ProfileDropdown from '$lib/components/ProfileDropdown.svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { Window, getCurrentWindow } from '@tauri-apps/api/window';
  import type { UnlistenFn } from '@tauri-apps/api/event';

  const appWindow: Window | null = browser ? getCurrentWindow() : null;

  let isMaximized = $state(false);
  let isFullscreen = $state(false);

  // --- IMPROVEMENT 1: Added onDestroy and event listeners ---
  let unlisten: UnlistenFn | null = null;

  onMount(async () => {
    if (!appWindow) return;

    try {
      // Check initial states
      isMaximized = await appWindow.isMaximized();
      isFullscreen = await appWindow.isFullscreen();

      // Listen for window resize events to sync state
      unlisten = await appWindow.onResized(async () => {
        isMaximized = await appWindow.isMaximized();
        isFullscreen = await appWindow.isFullscreen();
      });

    } catch (error) {
      console.error('[TitleBar] Failed to initialize window:', error);
    }
  });

  // Clean up the event listener when the component is destroyed
  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });

  // --- IMPROVEMENT 2: Simplified functions with async/await ---
  async function minimize(): Promise<void> {
    try {
      await appWindow?.minimize();
    } catch (err) {
      console.error('[TitleBar] Failed to minimize:', err);
    }
  }

  async function toggleMaximize(): Promise<void> {
    try {
      await appWindow?.toggleMaximize();
      // State will update automatically via the onResized listener
    } catch (err) {
      console.error('[TitleBar] Failed to toggle maximize:', err);
    }
  }

  async function close(): Promise<void> {
    try {
      await appWindow?.close();
    } catch (err) {
      console.error('[TitleBar] Failed to close window:', err);
    }
  }

  const navItems = [
    { path: '/', icon: 'solar:home-bold', label: 'Home' },
    { path: '/anime', icon: 'solar:video-library-bold', label: 'Anime' },
    { path: '/media-demo', icon: 'solar:gallery-bold', label: 'Demo' },
    { path: '/settings', icon: 'solar:settings-bold', label: 'Settings' },
  ];
</script>

{#if browser && !isFullscreen}
  <div
    data-tauri-drag-region
    class="fixed top-0 left-0 right-0 z-[999999] flex h-12 select-none items-center justify-between border-b border-border/50 bg-background/95 backdrop-blur-xl"
  >
    <div class="flex h-full items-center gap-2 pl-3" data-tauri-drag-region>
      <div class="flex items-center gap-2 px-2" data-tauri-drag-region>
        <div class="flex h-7 w-7 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-primary/70 shadow-lg shadow-primary/20">
          <Icon icon="solar:clock-circle-bold" class="h-4 w-4 text-primary-foreground" />
        </div>
        <div class="flex items-baseline gap-1" data-tauri-drag-region>
          <span class="text-lg font-bold tracking-tight">Zafkiel</span>
          <sup class="text-[8px] font-semibold text-accent-foreground -top-2 border border-accent/60 bg-accent px-1 py-1.5 rounded">
            ALPHA
          </sup>
        </div>
      </div>

      <div class="ml-2 flex h-full items-center gap-1">
        {#each navItems as item}
          {@const isActive = page.url.pathname === item.path}
          <Button
            variant="ghost"
            size="sm"
            class="h-8 gap-2 {isActive ? 'bg-primary/10 text-primary' : 'text-foreground/70 hover:text-foreground'}"
            onclick={() => goto(item.path)}
          >
            <Icon icon={item.icon} class="h-4 w-4" />
            <span class="text-xs font-medium">{item.label}</span>
          </Button>
        {/each}
      </div>
    </div>

    <div class="flex h-full items-center">
      <div class="px-2">
        <ProfileDropdown />
      </div>

      <div class="px-2">
        <ThemeSwitcher />
      </div>

      <div class="flex h-full items-center" data-tauri-drag-region="false">
        <button
          type="button"
          onclick={minimize}
          class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
          aria-label="Minimize"
        >
          <Icon icon="solar:minus-circle-bold" class="h-5 w-5 text-foreground" />
        </button>

        <button
          type="button"
          onclick={toggleMaximize}
          class="flex h-full w-11 items-center justify-center transition-colors hover:bg-foreground/5 active:bg-foreground/10"
          aria-label={isMaximized ? 'Restore' : 'Maximize'}
        >
          <Icon
            icon={isMaximized ? 'solar:quit-full-screen-square-bold' : 'solar:full-screen-square-bold'}
            class="h-5 w-5 text-foreground"
          />
        </button>

        <button
          type="button"
          onclick={close}
          class="flex h-full w-11 items-center justify-center transition-colors hover:bg-red-500 hover:text-white active:bg-red-600"
          aria-label="Close"
        >
          <Icon icon="solar:close-circle-bold" class="h-5 w-5" />
        </button>
      </div>
    </div>
  </div>

  <div class="h-12"></div>
{/if}

<style lang="postcss">
  [data-tauri-drag-region] {
    -webkit-app-region: drag;
  }

  [data-tauri-drag-region="false"],
  :global(button),
  :global(a),
  :global([role="button"]) {
    -webkit-app-region: no-drag;
  }
</style>
