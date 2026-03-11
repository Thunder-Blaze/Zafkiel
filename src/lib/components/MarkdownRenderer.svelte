<script lang="ts">
	/**
	 * Renders AniList-flavored markdown.
	 *
	 * Architecture: two-phase processing.
	 *
	 * Phase 1 — preprocessAnilist():
	 *   Converts AniList constructs that conflict with GFM rules into HTML
	 *   BEFORE marked ever sees the source. This avoids the GFM fenced-code-block
	 *   rule grabbing ~~~...~~~ and the img/youtube/webm tokens being misinterpreted
	 *   when they appear inside link text or headings.
	 *
	 *   • [ img###(url) ](link) — linked image with surrounding spaces
	 *   • ~~~...~~~             — single-line and multi-line center blocks
	 *     Inner content is passed through parseInline so bold/italic/img/etc. work.
	 *
	 * Phase 2 — markedInstance.parse():
	 *   Standard GFM markdown.  AniList-specific INLINE syntax that doesn't
	 *   conflict at block level is handled by registered inline extensions:
	 *   • ~!spoiler!~
	 *   • img###(url)  (standalone, not inside [ ] link)
	 *   • youtube(url)
	 *   • webm(url)
	 */

	import { Marked } from 'marked';
	import type { TokenizerThis, RendererThis, Tokens, Token } from 'marked';

	// ─── Inline Extension: Spoiler ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	const spoilerExt = {
		name: 'spoiler',
		level: 'inline' as const,
		start(src: string) {
			return src.indexOf('~!');
		},
		tokenizer(this: TokenizerThis, src: string) {
			const match = /^~!([\s\S]*?)!~/.exec(src);
			if (match) {
				const token: Tokens.Generic = {
					type: 'spoiler',
					raw: match[0],
					text: match[1],
					tokens: [] as Token[],
				};
				this.lexer.inline(match[1], token.tokens as Token[]);
				return token;
			}
		},
		renderer(this: RendererThis, token: Tokens.Generic) {
			const inner = this.parser.parseInline(token.tokens as Token[]);
			return `<span class="al-spoiler"><button type="button" class="al-spoiler-btn" onclick="this.closest('.al-spoiler').classList.toggle('open')" aria-expanded="false"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="11" height="11" fill="currentColor" aria-hidden="true"><path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/></svg>Spoiler</button><span class="al-spoiler-text">${inner}</span></span>`;
		},
	};

	// ─── Inline Extension: AniList Image ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// img(url) or img420(url) — case-insensitive, matches standalone occurrences.
	// Linked-image form [ img###(url) ](link) is handled by the preprocessor.
	const alImageExt = {
		name: 'alImage',
		level: 'inline' as const,
		start(src: string) {
			const idx = src.search(/img\d*\(/i);
			return idx >= 0 ? idx : src.length;
		},
		tokenizer(src: string) {
			const match = /^img(\d+)?\(([^)\s]+)\)/i.exec(src);
			if (match) {
				return { type: 'alImage', raw: match[0], width: match[1] ?? null, url: match[2] };
			}
		},
		renderer(token: Tokens.Generic) {
			const w = token.width ? ` width="${Number(token.width)}"` : '';
			return `<img src="${String(token.url)}"${w} alt="" class="al-img" loading="lazy">`;
		},
	};

	// ─── Inline Extension: YouTube embed ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	const youtubeExt = {
		name: 'youtube',
		level: 'inline' as const,
		start(src: string) {
			const idx = src.search(/youtube\(/i);
			return idx >= 0 ? idx : src.length;
		},
		tokenizer(src: string) {
			const match = /^youtube\(([^)]+)\)/i.exec(src);
			if (match) {
				return { type: 'youtube', raw: match[0], ref: match[1].trim() };
			}
		},
		renderer(token: Tokens.Generic) {
			const ref = String(token.ref);
			const m = ref.match(/(?:v=|youtu\.be\/)([A-Za-z0-9_-]{11})/);
			const id = m ? m[1] : ref;
			return `<div class="al-yt-wrap"><iframe class="al-yt" src="https://www.youtube.com/embed/${id}" allowfullscreen title="YouTube video" loading="lazy" frameborder="0"></iframe></div>`;
		},
	};

	// ─── Inline Extension: Video (webm / mp4) ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	const videoExt = {
		name: 'alVideo',
		level: 'inline' as const,
		start(src: string) {
			const idx = src.search(/webm\(/i);
			return idx >= 0 ? idx : src.length;
		},
		tokenizer(src: string) {
			const match = /^webm\(([^)]+)\)/i.exec(src);
			if (match) {
				return { type: 'alVideo', raw: match[0], url: match[1].trim() };
			}
		},
		renderer(token: Tokens.Generic) {
			return `<video src="${String(token.url)}" class="al-video" autoplay muted loop playsinline controls></video>`;
		},
	};

	// ─── Marked instance ──────────────────────────────────────────────────────
	const markedInstance = new Marked({ async: false, gfm: true, breaks: true });

	markedInstance.use({
		renderer: {
			link({ href, title, text }: { href: string; title?: string | null; text: string }) {
				const safeHref = href ?? '';
				// Empty href links (e.g. [text]()) are AniList accent-color decorators
				if (!safeHref || safeHref === '#') {
					return `<span class="al-accent">${text}</span>`;
				}
				const t = title ? ` title="${escAttr(title)}"` : '';
				if (text.trimStart().startsWith('<img')) {
					return `<a href="${safeHref}"${t} target="_blank" rel="noopener noreferrer" class="al-img-link">${text}</a>`;
				}
				return `<a href="${safeHref}"${t} target="_blank" rel="noopener noreferrer">${text}</a>`;
			},
			image({ href, title, text }: { href: string; title?: string | null; text: string }) {
				const t = title ? ` title="${escAttr(title)}"` : '';
				const alt = text ? ` alt="${escAttr(text)}"` : '';
				return `<img src="${href ?? ''}"${alt}${t} class="al-img" loading="lazy">`;
			},
		},
		extensions: [spoilerExt, alImageExt, youtubeExt, videoExt],
	});

	/** Minimal HTML attribute escaping */
	function escAttr(s: string): string {
		return s
			.replace(/&/g, '&amp;')
			.replace(/"/g, '&quot;')
			.replace(/</g, '&lt;')
			.replace(/>/g, '&gt;');
	}

	/**
	 * Phase 1: pre-process AniList constructs before GFM gets to them.
	 *
	 * Must be defined AFTER markedInstance so parseInline is available.
	 */
	/** Convert AniList bold syntax __text__ → <strong>text</strong> (same-line only) */
	function applyBold(s: string): string {
		return s.replace(/__([^_\n]+?)__/g, '<strong>$1</strong>');
	}

	/** Convert raw bare <a> and <a href=""> tags to accent-color spans (AniList decoration) */
	function applyAccentTags(s: string): string {
		// <a href="">text</a> or <a href=''>text</a> or bare <a>text</a>
		return s.replace(
			/<a(?:\s+href=['"]['"])?\s*>([/\s\S]*?)<\/a>/g,
			'<span class="al-accent">$1</span>'
		);
	}
	/**
	 * AniList heading rule: # followed immediately by non-space starts a heading
	 * (no space required, unlike standard GFM). Multiple headings can appear on
	 * the same line — each ends where the next # marker begins.
	 *
	 * e.g. `#Foo #Bar` → `<h1>Foo </h1><h1>Bar</h1>`
	 *
	 * Must be called AFTER applyBold/applyAccentTags so inline markers are already
	 * resolved before being passed into parseInline.
	 */
	function applyAnilistHeadings(s: string): string {
		return s.replace(/^.+$/gm, (line) => {
			// Skip lines with no #
			if (!line.includes('#')) return line;
			// Leave standard GFM headings (# followed by space at line start) for marked
			if (/^#{1,6} /.test(line)) return line;
			// Only act if there's a # directly followed by a non-space, non-# character
			if (!/#{1,6}[^\s#]/.test(line)) return line;

			// Split at each heading boundary: one or more # followed by non-space/non-#
			const parts = line.split(/(?=#{1,6}[^\s#])/);
			return parts
				.map((part) => {
					const m = part.match(/^(#{1,6})(.+)$/);
					if (!m) return part; // pre-heading text, keep as-is
					const level = Math.min(m[1].length, 6);
					// Use parseInline so img/youtube/accent/spoiler extensions still fire
					const content = markedInstance.parseInline(m[2].trim(), { async: false }) as string;
					return `<h${level}>${content}</h${level}>`;
				})
				.join('\n');
		});
	}
	function preprocessAnilist(src: string): string {
		// ── 1. Linked images  [ img###(url) ](href)  (spaces inside brackets) ──
		// Must run before the center substitution to avoid ~~~ inside links.
		src = src.replace(
			/\[\s*img(\d+)?\(([^)\s]+)\)\s*\]\(([^)]+)\)/gi,
			(_full, width, imgUrl, linkUrl) => {
				const w = width ? ` width="${Number(width)}"` : '';
				return `<a href="${linkUrl.trim()}" target="_blank" rel="noopener noreferrer" class="al-img-link"><img src="${imgUrl}"${w} alt="" class="al-img" loading="lazy"></a>`;
			}
		);

		// ── 2. ~~~...~~~ center blocks (single-line or multi-line) ───────────
		// Uses full block parsing so # headings work inside center blocks.
		// We apply bold + accent-tag preprocessing to the inner content first
		// so __text__ and <a> tags inside ~~~ render correctly.
		// Newlines around each div ensure the later global __ pass (which uses
		// [^_\n]) won't accidentally span across center block boundaries.
		src = src.replace(/~~~([\s\S]*?)~~~/g, (_full, inner) => {
			const trimmed = inner.trim();
			if (!trimmed) return '';
			// Apply bold → accent tags → AniList headings, then full block parse.
			// applyAnilistHeadings uses parseInline internally for heading content.
			// Remaining non-heading lines go through markedInstance.parse normally.
			const innerSrc = applyAnilistHeadings(applyBold(applyAccentTags(trimmed)));
			let rendered = markedInstance.parse(innerSrc, { async: false }) as string;
			// Strip empty paragraphs and unwrap <p> around lone <img> tags.
			rendered = rendered
				.replace(/<p>(\s|<br\s*\/?>)*<\/p>/g, '')
				.replace(/<p>\s*(<img[^>]*>)\s*<\/p>/g, '$1')
				.trim();
			return `\n<div class="al-center">${rendered}</div>\n`;
		});

		// ── 3. AniList bold  __text__  → <strong>text</strong> ─────────────────
		// Only matches on the SAME line (no \n between __) so it never swallows
		// the newlines injected around center divs above.
		src = applyBold(src);

		// Strip any lone __ markers left over from cross-block pairs.
		src = src.replace(/(?<!_)__(?!_)/g, '');

		// ── 4. Raw <a> accent tags (bare HTML in markdown) ───────────────────
		src = applyAccentTags(src);

		// ── 5. AniList no-space headings ────────────────────────────────────────
		// Must run after applyBold/applyAccentTags so inline markers are resolved.
		src = applyAnilistHeadings(src);

		// Strip leftover empty headings ( "# " or "# __" remnants ).
		src = src.replace(/^#+\s*$/gm, '');

		// Collapse 3+ consecutive newlines to prevent visible <br> gaps.
		src = src.replace(/\n{3,}/g, '\n\n');

		return src;
	}

	function parse(text: string): string {
		if (!text) return '';
		let html = markedInstance.parse(preprocessAnilist(text), { async: false }) as string;
		// Remove paragraphs that contain only whitespace and/or <br> tags
		// These are emitted by breaks:true around block-level HTML elements.
		html = html.replace(/<p>(\s|<br\s*\/?>)*<\/p>/g, '');
		return html;
	}

	let { body, class: className = '' }: { body?: string | null; class?: string } = $props();

	const html = $derived(body ? parse(body) : '');
</script>

{#if html}
	<div class="anilist-markdown {className}">
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html html}
	</div>
{/if}

<style>
	/* ── Base typography ─────────────────────────────────────────────────── */
	.anilist-markdown {
		font-size: 0.875rem;
		line-height: 1.6;
		color: var(--foreground);
		word-break: break-word;
	}
	.anilist-markdown :global(a) {
		color: var(--primary) !important;
		text-decoration: none;
		cursor: pointer;
	}
	.anilist-markdown :global(a:hover) {
		text-decoration: underline;
		text-underline-offset: 2px;
		opacity: 0.85;
	}
	.anilist-markdown :global(strong) {
		font-weight: 600;
	}
	.anilist-markdown :global(em) {
		font-style: italic;
	}
	.anilist-markdown :global(.al-accent) {
		color: var(--primary);
		font-weight: inherit;
	}
	/* Headings inside al-center should inherit accent if wrapped in al-accent */
	.anilist-markdown :global(.al-center .al-accent) {
		color: var(--primary);
	}
	.anilist-markdown :global(del) {
		text-decoration: line-through;
		opacity: 0.6;
	}
	.anilist-markdown :global(ul),
	.anilist-markdown :global(ol) {
		padding-left: 1.25rem;
		margin: 0.25rem 0;
	}
	.anilist-markdown :global(li) {
		margin: 0.1rem 0;
	}
	.anilist-markdown :global(h1),
	.anilist-markdown :global(h2),
	.anilist-markdown :global(h3),
	.anilist-markdown :global(h4),
	.anilist-markdown :global(h5),
	.anilist-markdown :global(h6) {
		font-weight: 600;
		line-height: 1.3;
		margin: 0.3rem 0 0.1rem;
	}
	.anilist-markdown :global(h1) {
		font-size: 1.4em;
	}
	.anilist-markdown :global(h2) {
		font-size: 1.25em;
	}
	.anilist-markdown :global(h3) {
		font-size: 1.1em;
	}

	/* ── Spoiler ──────────────────────────────────────────────────────────── */
	.anilist-markdown :global(.al-spoiler) {
		display: inline;
	}
	.anilist-markdown :global(.al-spoiler-btn) {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		background: var(--muted);
		border: 1px solid var(--border);
		border-radius: 999px;
		padding: 1px 8px 1px 6px;
		font-size: 0.78em;
		font-weight: 500;
		color: var(--muted-foreground);
		cursor: pointer;
		transition:
			background 0.15s,
			color 0.15s;
		vertical-align: middle;
		line-height: 1.6;
	}
	.anilist-markdown :global(.al-spoiler-btn:hover) {
		background: var(--accent);
		color: var(--foreground);
	}
	.anilist-markdown :global(.al-spoiler-text) {
		display: none;
		margin-left: 4px;
	}
	.anilist-markdown :global(.al-spoiler.open .al-spoiler-btn) {
		background: hsl(var(--primary) / 0.15);
		color: var(--primary);
		border-color: hsl(var(--primary) / 0.4);
	}
	.anilist-markdown :global(.al-spoiler.open .al-spoiler-text) {
		display: inline;
	}

	/* ── Center ──────────────────────────────────────────────────────────── */
	.anilist-markdown :global(.al-center) {
		text-align: center;
		display: block;
		margin: 0;
		padding: 0;
	}
	/* Zero out inner block margins so headings/paragraphs inside ~~~ are tight */
	.anilist-markdown :global(.al-center p),
	.anilist-markdown :global(.al-center h1),
	.anilist-markdown :global(.al-center h2),
	.anilist-markdown :global(.al-center h3),
	.anilist-markdown :global(.al-center h4),
	.anilist-markdown :global(.al-center h5),
	.anilist-markdown :global(.al-center h6) {
		margin: 0;
	}

	/* ── Images ──────────────────────────────────────────────────────────── */
	.anilist-markdown :global(img) {
		max-width: 100%;
		display: inline-block;
		vertical-align: middle;
		border-radius: 6px;
	}
	.anilist-markdown :global(.al-img-link) {
		display: inline-block;
	}
	.anilist-markdown :global(.al-img-link img) {
		display: block;
		transition: opacity 0.15s;
	}
	.anilist-markdown :global(.al-img-link:hover img) {
		opacity: 0.85;
	}

	/* ── YouTube embed ───────────────────────────────────────────────────── */
	.anilist-markdown :global(.al-yt-wrap) {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 */
		margin: 0.5rem 0;
		border-radius: 8px;
		overflow: hidden;
	}
	.anilist-markdown :global(.al-yt) {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		border: none;
	}

	/* ── Video ───────────────────────────────────────────────────────────── */
	.anilist-markdown :global(.al-video) {
		max-width: 100%;
		border-radius: 6px;
		display: block;
		margin: 4px 0;
	}

	/* ── Blockquotes ─────────────────────────────────────────────────────── */
	.anilist-markdown :global(blockquote) {
		border-left: 3px solid var(--border);
		padding-left: 1rem;
		color: var(--muted-foreground);
		font-style: italic;
		margin: 0.5rem 0;
	}

	/* ── Code ────────────────────────────────────────────────────────────── */
	.anilist-markdown :global(code) {
		background: var(--muted);
		border-radius: 4px;
		padding: 1px 5px;
		font-size: 0.85em;
	}
	.anilist-markdown :global(pre) {
		background: var(--muted);
		border-radius: 8px;
		padding: 0.75rem 1rem;
		overflow-x: auto;
	}
	.anilist-markdown :global(pre code) {
		background: transparent;
		padding: 0;
	}

	/* ── Misc ────────────────────────────────────────────────────────────── */
	.anilist-markdown :global(hr) {
		border: none;
		border-top: 1px solid var(--border);
		margin: 0.1rem 0;
	}
	.anilist-markdown :global(p:empty) {
		display: none;
	}
	.anilist-markdown :global(p) {
		display: block;
		margin: 0 0 0.2rem;
	}
	/* Remove stray <br> tags generated by breaks:true adjacent to block elements */
	.anilist-markdown :global(.al-center + br),
	.anilist-markdown :global(br + .al-center),
	.anilist-markdown :global(hr + br),
	.anilist-markdown :global(br + hr) {
		display: none;
	}
</style>
