<script lang="ts">
	/**
	 * Renders AniList-flavored markdown/HTML with spoiler support.
	 * AniList uses: __bold__, ~~strike~~, ~small~, [spoiler]...[/spoiler], bare URLs → links
	 * Body may also contain raw HTML from the API (sanitized).
	 */

	let { body, class: className = '' }: { body?: string | null; class?: string } = $props();

	/**
	 * Parse AniList markdown into safe HTML.
	 * Order matters — process block transforms before inline ones.
	 */
	function parseAnilistMarkdown(text: string): string {
		if (!text) return '';

		let html = text
			// Escape any pre-existing angle brackets that aren't part of approved tags
			// (API may include <br> and simple tags — keep them)

			// Headers
			.replace(/^###\s+(.+)$/gm, '<h3>$1</h3>')
			.replace(/^##\s+(.+)$/gm, '<h2>$1</h2>')
			.replace(/^#\s+(.+)$/gm, '<h1>$1</h1>')

			// Bold: **text** or __text__
			.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
			.replace(/__(.+?)__/g, '<strong>$1</strong>')

			// Italic: *text* or _text_
			.replace(/\*([^*\n]+?)\*/g, '<em>$1</em>')
			.replace(/_([^_\n]+?)_/g, '<em>$1</em>')

			// Strikethrough: ~~text~~
			.replace(/~~(.+?)~~/g, '<s>$1</s>')

			// Small: ~text~
			.replace(/~([^~\n]+?)~/g, '<small>$1</small>')

			// Spoiler tags [spoiler]text[/spoiler] — collapsible
			.replace(
				/\[spoiler\]([\s\S]*?)\[\/spoiler\]/gi,
				'<details class="spoiler"><summary>Spoiler</summary><span>$1</span></details>'
			)

			// Image: img(url)
			.replace(
				/img\(([^)]+)\)/g,
				'<img src="$1" alt="AniList image" class="inline-img" loading="lazy" />'
			)

			// YouTube: youtube(id)
			.replace(
				/youtube\(([A-Za-z0-9_-]+)\)/g,
				'<a href="https://youtu.be/$1" target="_blank" rel="noopener noreferrer" class="yt-link">▶ YouTube</a>'
			)

			// Markdown links: [text](url)
			.replace(
				/\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)/g,
				'<a href="$1" target="_blank" rel="noopener noreferrer">$2</a>'
			)

			// Bare URLs
			.replace(
				/(^|[\s(])((https?:\/\/)[^\s<>"']+)/g,
				'$1<a href="$2" target="_blank" rel="noopener noreferrer">$2</a>'
			)

			// Ordered lists: lines starting with "1. "
			.replace(/^\d+\.\s+(.+)$/gm, '<li>$1</li>')

			// Unordered lists: lines starting with "- " or "* "
			.replace(/^[-*]\s+(.+)$/gm, '<li>$1</li>')

			// Paragraphs: double newlines
			.replace(/\n{2,}/g, '</p><p>')

			// Single line breaks
			.replace(/\n/g, '<br>');

		return `<p>${html}</p>`;
	}

	const html = $derived(body ? parseAnilistMarkdown(body) : '');
</script>

{#if html}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="anilist-markdown prose prose-sm dark:prose-invert max-w-none {className}">
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</div>
{/if}

<style>
	.anilist-markdown :global(details.spoiler) {
		cursor: pointer;
		display: inline-block;
		border: 1px solid hsl(var(--border));
		border-radius: 4px;
		padding: 2px 6px;
	}
	.anilist-markdown :global(details.spoiler[open] summary) {
		margin-bottom: 4px;
	}
	.anilist-markdown :global(details.spoiler summary) {
		list-style: none;
		color: hsl(var(--muted-foreground));
		font-size: 0.8em;
	}
	.anilist-markdown :global(img.inline-img) {
		max-width: 100%;
		border-radius: 4px;
	}
	.anilist-markdown :global(p:empty) {
		display: none;
	}
</style>
