<script lang="ts">
	import { useSaveTextActivity } from '$lib/hooks/useAnilist.svelte';
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { tick } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { gsap } from 'gsap';
	import MarkdownRenderer from '$lib/components/MarkdownRenderer.svelte';

	let {
		onPublished,
		userAvatar,
		userName,
	}: {
		onPublished?: () => void;
		userAvatar?: string | null;
		userName?: string | null;
	} = $props();

	let expanded = $state(false);
	let text = $state('');
	let preview = $state(false);
	let uploading = $state(false);
	let uploadError = $state<string | null>(null);

	let textarea: HTMLTextAreaElement | undefined = $state();
	let composerEl: HTMLDivElement | undefined = $state();
	let editorAreaEl: HTMLDivElement | undefined = $state();

	const saveMutation = useSaveTextActivity();

	const charLimit = 2000;
	const remaining = $derived(charLimit - text.length);
	const canPublish = $derived(text.trim().length > 0 && text.length <= charLimit);

	function openComposer() {
		expanded = true;
		tick().then(() => {
			if (editorAreaEl) {
				gsap.to(editorAreaEl, { height: 'auto', duration: 0.25, ease: 'power2.out', clearProps: 'height' });
			}
			setTimeout(() => textarea?.focus(), 200);
		});
	}

	function resetState() {
		expanded = false;
		text = '';
		preview = false;
		uploadError = null;
	}

	function cancel() {
		const el = editorAreaEl;
		if (!el) {
			resetState();
			return;
		}
		gsap.set(el, { height: el.offsetHeight });
		gsap.to(el, {
			height: 0,
			duration: 0.18,
			ease: 'power2.in',
			onComplete: resetState,
		});
	}

	async function publish() {
		if (!canPublish) return;
		try {
			await saveMutation.mutateAsync({ text });
			cancel();
			onPublished?.();
		} catch (_e) {
			// error is surfaced from mutation state
		}
	}

	/** Insert markdown wrapper at cursor / around selection */
	function wrap(before: string, after: string = before) {
		if (!textarea) return;
		const start = textarea.selectionStart;
		const end = textarea.selectionEnd;
		const selected = text.slice(start, end);
		const replacement = `${before}${selected || 'text'}${after}`;
		text = text.slice(0, start) + replacement + text.slice(end);
		// Restore cursor
		setTimeout(() => {
			textarea?.focus();
			const cursorPos = start + before.length + (selected || 'text').length + after.length;
			textarea?.setSelectionRange(cursorPos, cursorPos);
		}, 0);
	}

	/** Insert a prefix at the start of the current line */
	function linePrefix(prefix: string) {
		if (!textarea) return;
		const start = textarea.selectionStart;
		const lineStart = text.lastIndexOf('\n', start - 1) + 1;
		const alreadyHas = text.slice(lineStart).startsWith(prefix);
		if (alreadyHas) {
			text = text.slice(0, lineStart) + text.slice(lineStart + prefix.length);
		} else {
			text = text.slice(0, lineStart) + prefix + text.slice(lineStart);
		}
		setTimeout(() => textarea?.focus(), 0);
	}

	/**
	 * Insert a template string at the cursor position and select a sub-range
	 * within it so the user can immediately type the value (e.g. a URL).
	 * @param template    The text to insert
	 * @param selectStart Offset from insert position where selection begins
	 * @param selectEnd   Offset from insert position where selection ends
	 */
	function insertTemplate(template: string, selectStart: number, selectEnd: number) {
		const pos = textarea ? textarea.selectionStart : text.length;
		text = text.slice(0, pos) + template + text.slice(pos);
		setTimeout(() => {
			textarea?.focus();
			textarea?.setSelectionRange(pos + selectStart, pos + selectEnd);
		}, 0);
	}

	/** Upload image to catbox.moe and insert link */
	async function handleImageUpload(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;

		if (file.size > 10 * 1024 * 1024) {
			uploadError = 'File must be under 10 MB';
			input.value = '';
			return;
		}

		uploading = true;
		uploadError = null;
		try {
			// Convert to regular array so Tauri serializes as JSON [1,2,3] (Vec<u8>)
			// Passing Uint8Array directly would serialize as {"0":1,"1":2,...} and break deserialization
			const arrayBuffer = await file.arrayBuffer();
			const bytes = Array.from(new Uint8Array(arrayBuffer));
			const url = await invoke<string>('upload_to_catbox', {
				bytes,
				filename: file.name,
				mimeType: file.type || 'application/octet-stream',
			});

			// Insert image using AniList markdown syntax
			const imgMd = `img(${url})`;
			if (!textarea) {
				text += imgMd;
			} else {
				const pos = textarea.selectionStart;
				text = text.slice(0, pos) + imgMd + text.slice(pos);
				setTimeout(() => {
					textarea?.focus();
					const newPos = pos + imgMd.length;
					textarea?.setSelectionRange(newPos, newPos);
				}, 0);
			}
		} catch (err) {
			// Tauri invoke errors are thrown as plain strings, not Error objects
			uploadError = err instanceof Error ? err.message : String(err);
		} finally {
			uploading = false;
			input.value = '';
		}
	}

	const toolbarButtons = [
		{ icon: 'solar:text-bold-square-bold', label: 'Bold', action: () => wrap('**') },
		{ icon: 'solar:text-italic-bold', label: 'Italic', action: () => wrap('_') },
		{ icon: 'solar:text-cross-bold', label: 'Strikethrough', action: () => wrap('~~') },
		// Heading — inserts '# ' at line start
		{ icon: 'solar:text-bold-square-linear', label: 'Heading', action: () => linePrefix('# ') },
		{
			icon: 'solar:eye-closed-bold',
			label: 'Spoiler',
			action: () => wrap('~!', '!~'),
		},
		{
			icon: 'solar:align-horizonal-center-bold',
			label: 'Center',
			action: () => wrap('~~~', '~~~'),
		},
		{ icon: 'solar:link-bold', label: 'Link', action: () => wrap('[', '](url)') },
		// AniList image embed from URL — inserts img(url) with url pre-selected
		{
			icon: 'solar:gallery-wide-bold',
			label: 'Embed image URL',
			action: () => insertTemplate('img(url)', 4, 7),
		},
		// YouTube embed
		{
			icon: 'solar:videocamera-record-bold',
			label: 'YouTube embed',
			action: () => insertTemplate('youtube(url)', 8, 11),
		},
		// webm / video
		{
			icon: 'solar:play-circle-bold',
			label: 'Video embed',
			action: () => insertTemplate('webm(url)', 5, 8),
		},
		{
			icon: 'solar:list-bold',
			label: 'Bullet list',
			action: () => linePrefix('- '),
		},
		{
			icon: 'solar:sort-by-alphabet-bold',
			label: 'Numbered list',
			action: () => linePrefix('1. '),
		},
		{ icon: 'solar:chat-square-code-bold', label: 'Code', action: () => wrap('`') },
		{
			icon: 'solar:chat-line-bold-duotone',
			label: 'Quote',
			action: () => linePrefix('> '),
		},
	] as const;
</script>

<!-- Unified composer — one element that morphs between collapsed and expanded -->
<div bind:this={composerEl} class="rounded-xl border bg-card {expanded ? 'shadow-md' : ''}">
	<!-- Header: button trigger when collapsed, identity bar when expanded -->
	{#if !expanded}
		<button
			class="flex w-full items-center gap-3 px-4 py-3 transition-colors hover:bg-muted/40"
			onclick={openComposer}
			aria-label="Write a status update"
		>
			<div class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-muted">
				{#if userAvatar}
					<img src={userAvatar} alt={userName ?? 'You'} class="h-full w-full rounded-full object-cover" />
				{:else}
					<Icon icon="solar:user-bold" class="size-5 text-muted-foreground" />
				{/if}
			</div>
			<span class="flex-1 text-left text-sm text-muted-foreground">Write a status update…</span>
			<Icon icon="solar:pen-new-square-linear" class="size-4 shrink-0 text-muted-foreground" />
		</button>
	{:else}
		<div class="flex items-center gap-3 px-4 py-3">
			<div class="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-muted">
				{#if userAvatar}
					<img src={userAvatar} alt={userName ?? 'You'} class="h-full w-full rounded-full object-cover" />
				{:else}
					<Icon icon="solar:user-bold" class="size-5 text-muted-foreground" />
				{/if}
			</div>
			<span class="flex-1 text-sm font-medium text-foreground">{userName ?? 'Status Update'}</span>
		</div>
	{/if}

	<!-- Editor body — height animated by GSAP (starts at 0, expands when opened) -->
	<div bind:this={editorAreaEl} class="overflow-hidden" style="height: 0">
		<!-- Toolbar -->
		<div class="flex flex-wrap items-center gap-0.5 border-b px-3 py-2">
			{#each toolbarButtons as btn (btn.label)}
				<button
					type="button"
					title={btn.label}
					aria-label={btn.label}
					onclick={btn.action}
					class="rounded p-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
				>
					<Icon icon={btn.icon} class="size-4" />
				</button>
			{/each}

			<div class="mx-1 h-5 w-px bg-border"></div>

			<!-- Image upload -->
			<label
				class="relative cursor-pointer rounded p-1.5 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
				title="Upload image (max 10 MB)"
			>
				{#if uploading}
					<Icon icon="solar:spinner-bold" class="size-4 animate-spin" />
				{:else}
					<Icon icon="solar:gallery-add-bold" class="size-4" />
				{/if}
				<input
					type="file"
					accept="image/*,video/webm"
					class="sr-only"
					onchange={handleImageUpload}
					disabled={uploading}
				/>
			</label>

			<div class="ml-auto flex items-center gap-1.5">
				<!-- Preview toggle -->
				<button
					type="button"
					onclick={() => (preview = !preview)}
					class="rounded px-2 py-1 text-xs font-medium transition-colors {preview
						? 'bg-primary text-primary-foreground'
						: 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
				>
					{preview ? 'Edit' : 'Preview'}
				</button>
			</div>
		</div>

		<!-- Editor / Preview area -->
		<div class="min-h-[120px] p-3">
			{#if preview}
				<div class="prose prose-sm dark:prose-invert max-w-none" transition:fade={{ duration: 100 }}>
					{#if text.trim()}
						<MarkdownRenderer body={text} />
					{:else}
						<p class="text-sm italic text-muted-foreground">Nothing to preview yet…</p>
					{/if}
				</div>
			{:else}
				<textarea
					bind:this={textarea}
					bind:value={text}
					placeholder="Write a status update… Markdown is supported"
					rows={5}
					class="w-full resize-none border-0 bg-transparent text-sm text-foreground shadow-none outline-none ring-0 placeholder:text-muted-foreground focus:border-0 focus:outline-none focus:ring-0"
					transition:fade={{ duration: 100 }}
				></textarea>
			{/if}
		</div>

		<!-- Upload error -->
		{#if uploadError}
			<div class="mx-3 mb-2 flex items-center gap-2 rounded-md bg-destructive/10 px-3 py-2 text-xs text-destructive">
				<Icon icon="solar:danger-triangle-bold" class="size-3.5 shrink-0" />
				{uploadError}
			</div>
		{/if}

		<!-- Save error -->
		{#if saveMutation.error}
			<div class="mx-3 mb-2 flex items-center gap-2 rounded-md bg-destructive/10 px-3 py-2 text-xs text-destructive">
				<Icon icon="solar:danger-triangle-bold" class="size-3.5 shrink-0" />
				{(saveMutation.error as Error).message ?? 'Failed to post'}
			</div>
		{/if}

		<!-- Footer: char counter + actions -->
		<div class="flex items-center justify-between border-t px-3 py-2">
			<span
				class="text-xs {remaining < 50
					? remaining < 0
						? 'text-destructive font-medium'
						: 'text-amber-500'
					: 'text-muted-foreground'}"
			>
				{remaining} / {charLimit}
			</span>

			<div class="flex items-center gap-2">
				<button
					type="button"
					onclick={cancel}
					class="rounded-lg px-3 py-1.5 text-sm text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={publish}
					disabled={!canPublish || saveMutation.isPending}
					class="flex items-center gap-1.5 rounded-lg bg-primary px-4 py-1.5 text-sm font-medium text-primary-foreground transition-all hover:bg-primary/90 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{#if saveMutation.isPending}
						<Icon icon="solar:spinner-bold" class="size-3.5 animate-spin" />
					{/if}
					Publish
				</button>
			</div>
		</div>
	</div>
</div>
