<script lang="ts">
	import {
		Dialog,
		DialogContent,
		DialogHeader,
		DialogTitle,
		DialogDescription,
		DialogFooter,
	} from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { invoke } from '@tauri-apps/api/core';
	import type { CatalogExtension, ExtensionSetting } from '$lib/types/extensions';
	import { toast } from 'svelte-sonner';

	interface Props {
		open: boolean;
		onOpenChange: (open: boolean) => void;
		extension: CatalogExtension;
	}

	let { open, onOpenChange, extension }: Props = $props();

	let settingsValues = $state<Record<string, any>>({});
	let isSaving = $state(false);
	let isLoading = $state(false);

	async function loadSettings() {
		if (!extension.settings || !open) return;
		isLoading = true;
		try {
			const newValues: Record<string, any> = {};
			for (const setting of extension.settings) {
				const val = await invoke<string | null>('ext_storage_get', {
					extId: extension.id,
					key: `setting:${setting.id}`,
				});
				if (val !== null) {
					try {
						newValues[setting.id] = JSON.parse(val);
					} catch {
						newValues[setting.id] = val;
					}
				} else {
					newValues[setting.id] = setting.default;
				}
			}
			settingsValues = newValues;
		} catch (e) {
			console.error("Failed to load settings:", e);
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		if (open) {
			loadSettings();
		}
	});

	async function saveSettings() {
		isSaving = true;
		try {
			for (const setting of extension.settings || []) {
				const value = settingsValues[setting.id];
				await invoke('ext_storage_set', {
					extId: extension.id,
					key: `setting:${setting.id}`,
					value: typeof value === 'string' ? value : JSON.stringify(value),
				});
			}
			toast.success('Settings saved');
			onOpenChange(false);
		} catch (e) {
			toast.error('Failed to save settings');
			console.error(e);
		} finally {
			isSaving = false;
		}
	}
</script>

<Dialog {open} {onOpenChange}>
	<DialogContent class="sm:max-w-[425px]">
		<DialogHeader>
			<DialogTitle>{extension.name} Settings</DialogTitle>
			<DialogDescription>
				Configure preferences for the {extension.name} extension.
			</DialogDescription>
		</DialogHeader>

		<div class="grid gap-4 py-4">
			{#if isLoading}
				<div class="text-center text-sm text-muted-foreground">Loading...</div>
			{:else if !extension.settings || extension.settings.length === 0}
				<div class="text-center text-sm text-muted-foreground">No settings available.</div>
			{:else}
				{#each extension.settings as setting}
					<div class="grid gap-2">
						<Label for={setting.id}>{setting.label}</Label>
						{#if setting.type === 'select'}
							<select
								id={setting.id}
								bind:value={settingsValues[setting.id]}
								class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
							>
								{#each setting.options || [] as opt}
									<option value={opt.value}>{opt.label}</option>
								{/each}
							</select>
						{:else if setting.type === 'boolean'}
							<div class="flex items-center gap-2">
								<input
									type="checkbox"
									id={setting.id}
									bind:checked={settingsValues[setting.id]}
								/>
								<span class="text-sm text-muted-foreground">{setting.description || ''}</span>
							</div>
						{:else}
							<Input
								id={setting.id}
								type={setting.type === 'number' ? 'number' : 'text'}
								bind:value={settingsValues[setting.id]}
								placeholder={String(setting.default)}
							/>
							{#if setting.description}
								<p class="text-[10px] text-muted-foreground">{setting.description}</p>
							{/if}
						{/if}
					</div>
				{/each}
			{/if}
		</div>

		<DialogFooter>
			<Button variant="outline" onclick={() => onOpenChange(false)}>Cancel</Button>
			<Button onclick={saveSettings} disabled={isSaving || isLoading}>Save changes</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
