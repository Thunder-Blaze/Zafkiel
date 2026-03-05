<script lang="ts">
	/**
	 * Renders AniList-flavored markdown/HTML.
	 *
	 * AniList-specific syntax handled here before passing to marked:
	 *   ~!spoiler!~            → collapsible spoiler block
	 *   ~~~text~~~             → centered text
	 *   img420(url)            → inline image (optional pixel width)
	 *   youtube(url|id)        → YouTube link
	 *   webm(url)              → inline looping video
	 *
	 * Standard markdown is processed by `marked` (GFM + line-break mode).
	 */

	import { Marked } from 'marked';

	// Isolated marked instance created once at module level — avoids mutating the global
	// singleton and prevents the `parseInline` crash caused by losing the internal parser context.
	const markedInstance = new Marked({
		gfm: true,
		breaks: true,
		renderer: {
			link({ href, title, text }: { href: string; title?: string | null; text: string }) {
				const t = title ? ` title="${title}"` : '';
				return `<a href="${href ?? ''}"${t} target="_blank" rel="noopener noreferrer">${text}</a>`;
			},
		},
	});

	let { body, class: className = '' }: { body?: string | null; class?: string } = $props();

	function preprocess(text: string): string {
		return (
			text
				// Spoilers: ~!content!~ (must come before any ~ strikethrough or center handling)
				.replace(
					/~!([\s\S]*?)!~/g,
					'<details class="spoiler"><summary>Spoiler</summary><div>$1</div></details>'
				)

				// Center alignment: ~~~content~~~ (must come before ~~ strikethrough)
				.replace(/~~~([\s\S]*?)~~~/g, '<center>$1</center>')

				// AniList images: img420(url) or img(url)
				.replace(/img(\d+)?\(([^)\s]+)\)/g, (_, width, url) => {
					const w = width ? ` width="${Number(width)}"` : '';
					return `<img src="${url}"${w} alt="" class="al-img" loading="lazy">`;
				})

				// YouTube embed: youtube(https://youtu.be/ID) or youtube(ID)
				.replace(/youtube\(([^)]+)\)/g, (_, ref) => {
					const match = ref.match(/(?:v=|youtu\.be\/)([A-Za-z0-9_-]{11})/);
					const id = match ? match[1] : ref.trim();
					return `<a href="https://youtu.be/${id}" target="_blank" rel="noopener noreferrer" class="al-yt">▶ Watch on YouTube</a>`;
				})

				// webm / video embeds: webm(url)
				.replace(
					/webm\(([^)]+)\)/g,
					'<video src="$1" class="al-video" autoplay muted loop controls></video>'
				)
		);
	}

	function parseAnilistMarkdown(text: string): string {
		if (!text) return '';
		const preprocessed = preprocess(text);
		return markedInstance.parse(preprocessed) as string;
	}

	const html = $derived(body ? parseAnilistMarkdown(body) : '');
</script>

{#if html}
	<div class="anilist-markdown prose prose-sm dark:prose-invert max-w-none {className}">
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</div>
{/if}

<style>
	/* Spoiler block */
	.anilist-markdown :global(details.spoiler) {
		display: inline-block;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		padding: 2px 8px;
		cursor: pointer;
		margin: 2px 0;
	}
	.anilist-markdown :global(details.spoiler summary) {
		list-style: none;
		color: hsl(var(--muted-foreground));
		font-size: 0.8em;
		user-select: none;
	}
	.anilist-markdown :global(details.spoiler[open] summary) {
		margin-bottom: 4px;
	}
	.anilist-markdown :global(details.spoiler div) {
		padding: 4px 0;
	}

	/* All images render inline-block so they flow side-by-side */
	.anilist-markdown :global(img) {
		max-width: 100%;
		display: inline-block;
		vertical-align: middle;
	}
	.anilist-markdown :global(img.al-img) {
		border-radius: 6px;
	}

	/* YouTube link pill */
	.anilist-markdown :global(a.al-yt) {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		background: hsl(var(--muted));
		border-radius: 6px;
		padding: 2px 10px;
		font-size: 0.85em;
		text-decoration: none;
	}

	/* Inline/looping video */
	.anilist-markdown :global(video.al-video) {
		max-width: 100%;
		border-radius: 6px;
		display: block;
		margin: 4px 0;
	}

	/* Suppress empty paragraphs that marked sometimes emits */
	.anilist-markdown :global(p:empty) {
		display: none;
	}

	/* Paragraphs flow as block by default; collapse margin when only images inside */
	.anilist-markdown :global(p) {
		display: block;
		margin-bottom: 0.5rem;
	}

	/* Blockquotes */
	.anilist-markdown :global(blockquote) {
		border-left: 3px solid hsl(var(--border));
		padding-left: 1rem;
		color: hsl(var(--muted-foreground));
		font-style: italic;
		margin: 0.5rem 0;
	}

	/* Inline code */
	.anilist-markdown :global(code) {
		background: hsl(var(--muted));
		border-radius: 4px;
		padding: 1px 5px;
		font-size: 0.85em;
	}

	/* Code blocks */
	.anilist-markdown :global(pre) {
		background: hsl(var(--muted));
		border-radius: 8px;
		padding: 0.75rem 1rem;
		overflow-x: auto;
	}
	.anilist-markdown :global(pre code) {
		background: transparent;
		padding: 0;
	}

	/* Horizontal rule */
	.anilist-markdown :global(hr) {
		border: none;
		border-top: 1px solid hsl(var(--border));
		margin: 1rem 0;
	}
</style>
