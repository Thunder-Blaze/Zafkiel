<script lang="ts">
	import { ConfigService } from '$lib/services/config';
	import type { ExtensionConfig } from '$lib/types/config';
	import { toast } from 'svelte-sonner';
	import { onMount } from 'svelte';
	import { fade, slide } from 'svelte/transition';
	import Icon from '@iconify/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { extensionStore } from '$lib/stores/extensionStore.svelte';
	import type { CatalogExtension } from '$lib/types/extensions';
	import ExtensionSettingsDialog from '$lib/components/extensions/ExtensionSettingsDialog.svelte';

	let config = $state<ExtensionConfig>({ repositories: [] });
	let repoInput = $state('');
	let isSaving = $state(false);
	let settingsOpen = $state<string | null>(null);
	onMount(async () => {
		extensionStore.init();
		try {
			const appConfig = await ConfigService.getConfig();
			config = appConfig.extensions || { repositories: [] };
		} catch (e) {
			console.error('Failed to load config', e);
		}
	});
	async function addRepo() {
		if (!repoInput) return;
		try {
			// Basic URL validation
			new URL(repoInput);
		} catch (e) {
			toast.error('Invalid URL');
			return;
		}
		if (config.repositories.includes(repoInput)) {
			toast.error('Repository already exists');
			return;
		}
		config.repositories.push(repoInput);
		repoInput = '';
		await saveConfig();
	}
	async function removeRepo(repo: string) {
		config.repositories = config.repositories.filter((r) => r !== repo);
		await saveConfig();
	}
	async function saveConfig() {
		isSaving = true;
		try {
			await ConfigService.updateExtensionConfig(config);
			toast.success('Extension repositories updated');
			// Re-init extension store to fetch from new repos
			await extensionStore.init();
		} catch (e) {
			toast.error('Failed to save repositories');
			console.error(e);
		} finally {
			isSaving = false;
		}
	}

	// ── Derived state ──────────────────────────────────────────────────────────

	const installed = $derived(
		extensionStore.catalog.filter((ext) => extensionStore.isInstalled(ext.id))
	);
	const available = $derived(
		extensionStore.catalog.filter((ext) => !extensionStore.isInstalled(ext.id))
	);

	// ── Type helpers ──────────────────────────────────────────────────────────

	function typeLabel(type: CatalogExtension['type']): string {
		return type === 'source' ? 'Source' : type === 'torrent' ? 'Torrent' : 'Other';
	}

	function typeIcon(type: CatalogExtension['type']): string {
		return type === 'source'
			? 'solar:play-stream-bold'
			: type === 'torrent'
				? 'solar:download-minimalistic-bold'
				: 'solar:widget-bold';
	}
</script>

<div class="mx-auto max-w-4xl space-y-10 px-6 py-8" in:fade={{ duration: 200 }}>
	<!-- Page header -->
	<div class="space-y-1">
		<h1 class="text-2xl font-bold tracking-tight">Extensions</h1>
		<p class="text-sm text-muted-foreground">
			Extend Zafkiel with streaming sources, torrent providers, and more.
		</p>
	</div>

	<!-- Repositories section -->
	<section class="space-y-4 rounded-xl border border-border/60 bg-card/40 p-5 backdrop-blur-sm">
		<h2 class="flex items-center gap-2 text-base font-semibold text-foreground/80">
			<Icon icon="solar:server-bold" class="h-4 w-4" />
			Extension Repositories
		</h2>
		<div class="space-y-2">
			{#each config.repositories as repo}
				<div class="flex items-center justify-between rounded-lg bg-muted/40 px-3 py-2 text-sm">
					<span class="truncate">{repo}</span>
					<Button
						variant="ghost"
						size="icon"
						class="h-7 w-7 text-destructive hover:bg-destructive/10"
						disabled={isSaving}
						onclick={() => removeRepo(repo)}
					>
						<Icon icon="solar:trash-bin-trash-bold" class="h-4 w-4" />
					</Button>
				</div>
			{/each}
		</div>
		<form
			class="flex items-center gap-2"
			onsubmit={(e) => {
				e.preventDefault();
				addRepo();
			}}
		>
			<input
				type="url"
				bind:value={repoInput}
				placeholder="https://example.com/api/registry.json"
				class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
			/>
			<Button type="submit" size="sm" disabled={isSaving || !repoInput} class="shrink-0">
				Add Repository
			</Button>
		</form>
	</section>
	<!-- Installed section -->
	{#if installed.length > 0}
		<section class="space-y-4" in:slide={{ duration: 200 }}>
			<h2 class="text-base font-semibold text-foreground/80">
				Installed
				<span class="ml-1.5 text-xs font-normal text-muted-foreground">{installed.length}</span>
			</h2>

			<div class="grid gap-3 sm:grid-cols-2">
				{#each installed as ext (ext.id)}
					{@const status = extensionStore.getStatus(ext.id)}
					<div
						class="relative overflow-hidden rounded-xl border border-border/60 bg-card/80 p-4 shadow-sm backdrop-blur-sm"
					>
						<!-- Error banner -->
						{#if status.kind === 'error'}
							<div
								class="mb-3 rounded-lg bg-destructive/10 px-3 py-2 text-xs text-destructive"
								in:slide={{ duration: 150 }}
							>
								<span class="font-semibold">Error: </span>{status.message}
							</div>
						{/if}

						<div class="flex items-start gap-3">
							<!-- Icon -->
							<div
								class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg bg-muted/60 text-foreground/70"
							>
								{#if ext.iconUrl}
									<img
										src={ext.iconUrl}
										alt={ext.name}
										class="h-7 w-7 rounded object-contain"
										onerror={(e) => {
											(e.currentTarget as HTMLImageElement).style.display = 'none';
										}}
									/>
								{:else}
									<Icon icon={typeIcon(ext.type)} class="h-5 w-5" />
								{/if}
							</div>

							<!-- Info -->
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<span class="truncate font-medium">{ext.name}</span>
									<Badge variant="outline" class="shrink-0 px-1.5 py-0 text-[10px]">
										{typeLabel(ext.type)}
									</Badge>
									{#if ext.requiresAuth}
										<span title="Requires authentication">
											<Icon
												icon="solar:lock-keyhole-bold"
												class="h-3.5 w-3.5 shrink-0 text-muted-foreground"
											/>
										</span>
									{/if}
								</div>
								<p class="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
									{ext.description}
								</p>
								{#if status.kind === 'installed' || status.kind === 'ready'}
									<p class="mt-1 text-[10px] text-muted-foreground/60">
										v{status.entry.version} · {status.entry.author}
									</p>
								{/if}
							</div>

							<!-- Actions -->
							<div class="flex shrink-0 flex-col gap-1.5">
								{#if status.kind === 'error' && status.canReinstall}
									<Button
										size="sm"
										variant="outline"
										class="h-7 text-xs"
										disabled={extensionStore.isBusy(ext.id)}
										onclick={() => extensionStore.reinstall(ext.id)}
									>
										{#if extensionStore.isBusy(ext.id)}
											<Icon icon="solar:refresh-bold" class="h-3.5 w-3.5 animate-spin" />
										{:else}
											Reinstall
										{/if}
									</Button>
								{/if}
								{#if (status.kind === 'installed' || status.kind === 'ready') && ext.settings && ext.settings.length > 0}
									<Button
										size="sm"
										variant="outline"
										class="h-7 text-xs"
										onclick={() => {
											settingsOpen = ext.id;
										}}
									>
										<Icon icon="solar:settings-bold" class="mr-1 h-3.5 w-3.5" />
										Settings
									</Button>
								{/if}
								{#if status.kind === 'loading' || status.kind === 'downloading'}
									<div class="flex h-7 items-center gap-1.5 text-xs text-muted-foreground">
										<Icon icon="solar:refresh-bold" class="h-3.5 w-3.5 animate-spin" />
										{status.kind === 'downloading' ? 'Downloading…' : 'Loading…'}
									</div>
								{:else}
									<Button
										size="sm"
										variant="ghost"
										class="h-7 text-xs text-destructive hover:bg-destructive/10 hover:text-destructive"
										disabled={extensionStore.isBusy(ext.id)}
										onclick={() => extensionStore.uninstall(ext.id)}
									>
										{#if extensionStore.isBusy(ext.id)}
											<Icon icon="solar:refresh-bold" class="h-3.5 w-3.5 animate-spin" />
										{:else}
											Uninstall
										{/if}
									</Button>
								{/if}
							</div>
						</div>

						<!-- Tags -->
						{#if ext.tags && ext.tags.length > 0}
							<div class="mt-3 flex flex-wrap gap-1">
								{#each ext.tags as tag (tag)}
									<span class="rounded-full bg-muted px-2 py-0.5 text-[10px] text-muted-foreground">
										{tag}
									</span>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</section>
	{/if}

	<!-- Available section -->
	<section class="space-y-4">
		<h2 class="text-base font-semibold text-foreground/80">
			Available
			<span class="ml-1.5 text-xs font-normal text-muted-foreground">{available.length}</span>
		</h2>

		{#if available.length === 0}
			<p class="text-sm text-muted-foreground">
				{config.repositories.length === 0
					? 'No repositories added. Add one above to find extensions.'
					: 'All available extensions are already installed or none found.'}
			</p>
		{:else}
			<div class="grid gap-3 sm:grid-cols-2">
				{#each available as ext (ext.id)}
					{@const status = extensionStore.getStatus(ext.id)}
					<div
						class="relative overflow-hidden rounded-xl border border-border/60 bg-card/60 p-4 shadow-sm backdrop-blur-sm"
					>
						<div class="flex items-start gap-3">
							<!-- Icon -->
							<div
								class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg bg-muted/40 text-foreground/50"
							>
								{#if ext.iconUrl}
									<img
										src={ext.iconUrl}
										alt={ext.name}
										class="h-7 w-7 rounded object-contain opacity-80"
										onerror={(e) => {
											(e.currentTarget as HTMLImageElement).style.display = 'none';
										}}
									/>
								{:else}
									<Icon icon={typeIcon(ext.type)} class="h-5 w-5" />
								{/if}
							</div>

							<!-- Info -->
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<span class="truncate font-medium">{ext.name}</span>
									<Badge variant="secondary" class="shrink-0 px-1.5 py-0 text-[10px]">
										{typeLabel(ext.type)}
									</Badge>
									{#if ext.requiresAuth}
										<span title="Requires authentication setup">
											<Icon
												icon="solar:lock-keyhole-bold"
												class="h-3.5 w-3.5 shrink-0 text-amber-500"
											/>
										</span>
									{/if}
								</div>
								<p class="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
									{ext.description}
								</p>
								<p class="mt-1 text-[10px] text-muted-foreground/60">
									v{ext.latestVersion} · {ext.author}
								</p>
							</div>

							<!-- Install button -->
							<div class="shrink-0">
								{#if status.kind === 'downloading'}
									<div class="flex h-8 items-center gap-1.5 text-xs text-muted-foreground">
										<Icon icon="solar:refresh-bold" class="h-3.5 w-3.5 animate-spin" />
										Downloading…
									</div>
								{:else}
									<Button
										size="sm"
										class="h-8 gap-1.5 text-xs"
										disabled={extensionStore.isBusy(ext.id)}
										onclick={() => extensionStore.install(ext.id, ext.downloadUrl)}
									>
										{#if extensionStore.isBusy(ext.id)}
											<Icon icon="solar:refresh-bold" class="h-3.5 w-3.5 animate-spin" />
										{:else}
											<Icon icon="solar:download-minimalistic-bold" class="h-3.5 w-3.5" />
											Install
										{/if}
									</Button>
								{/if}
							</div>
						</div>

						<!-- Tags -->
						{#if ext.tags && ext.tags.length > 0}
							<div class="mt-3 flex flex-wrap gap-1">
								{#each ext.tags as tag (tag)}
									<span
										class="rounded-full bg-muted/60 px-2 py-0.5 text-[10px] text-muted-foreground"
									>
										{tag}
									</span>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Footer notice -->
	<p class="text-xs text-muted-foreground/50">
		Extensions are downloaded and stored locally in
		<code class="rounded bg-muted px-1 py-0.5 text-[10px]">~/.config/zafkiel/extensions/</code>.
		They are only loaded when needed and run in the app's WebView context.
	</p>
</div>
{#if settingsOpen}
	{@const ext = extensionStore.catalog.find((e) => e.id === settingsOpen)}
	{#if ext}
		<ExtensionSettingsDialog
			open={true}
			onOpenChange={(val) => {
				if (!val) settingsOpen = null;
			}}
			extension={ext}
		/>
	{/if}
{/if}
