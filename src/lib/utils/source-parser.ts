/**
 * Utilities for parsing stream source metadata from labels like "[SubsPlease] 1080p DUB".
 */

export interface ParsedSourceMeta {
	source: string | null;
	quality: string | null;
	qualityNumeric: number | null;
	language: 'sub' | 'dub' | null;
	raw: string;
}

const QUALITY_RE = /(\d{3,4})p/i;
const FANSUB_RE = /\[([^\]]+)\]/;
const DUB_RE = /\b(dub|eng(?:lish)?)\b/i;
const SUB_RE = /\b(sub|jpn|japanese)\b/i;

/**
 * Parse a stream source label or its constituent fields into structured metadata.
 *
 * Accepts either a single raw label string OR the individual fields from
 * a `StreamSource` (fansub, resolution, audio).
 */
export function parseSourceLabel(
	labelOrFields: string | { label?: string; fansub?: string; resolution?: string; audio?: string }
): ParsedSourceMeta {
	if (typeof labelOrFields === 'string') {
		return parseRawLabel(labelOrFields);
	}

	const { label, fansub, resolution, audio } = labelOrFields;

	const qualityNumeric = resolution ? parseInt(resolution, 10) || null : null;

	let language: 'sub' | 'dub' | null = null;
	if (audio === 'eng' || audio === 'dub') language = 'dub';
	else if (audio === 'jpn' || audio === 'sub') language = 'sub';

	return {
		source: fansub || null,
		quality: resolution ? `${resolution}p` : null,
		qualityNumeric,
		language,
		raw:
			label ||
			[fansub ? `[${fansub}]` : '', resolution ? `${resolution}p` : '', audio || '']
				.filter(Boolean)
				.join(' '),
	};
}

function parseRawLabel(raw: string): ParsedSourceMeta {
	const fansubMatch = raw.match(FANSUB_RE);
	const qualityMatch = raw.match(QUALITY_RE);
	const isDub = DUB_RE.test(raw);
	const isSub = SUB_RE.test(raw);

	return {
		source: fansubMatch?.[1] ?? null,
		quality: qualityMatch?.[0] ?? null,
		qualityNumeric: qualityMatch ? parseInt(qualityMatch[1], 10) : null,
		language: isDub ? 'dub' : isSub ? 'sub' : null,
		raw,
	};
}

/**
 * Score how well a candidate source matches a reference set of preferences.
 * Higher score = better match.
 *
 * Priority: language (4 pts) > quality (2 pts) > source (1 pt)
 */
export function scoreSourceMatch(
	candidate: ParsedSourceMeta,
	preferred: { language?: 'sub' | 'dub' | null; quality?: string | null; source?: string | null }
): number {
	let score = 0;
	if (preferred.language && candidate.language === preferred.language) score += 4;
	if (preferred.quality && candidate.quality === preferred.quality) score += 2;
	if (preferred.source && candidate.source?.toLowerCase() === preferred.source?.toLowerCase())
		score += 1;
	return score;
}

/**
 * Find the best matching source from a list of candidates based on preferred metadata.
 */
export function findBestSource(
	candidates: ParsedSourceMeta[],
	preferred: { language?: 'sub' | 'dub' | null; quality?: string | null; source?: string | null }
): number {
	if (candidates.length === 0) return -1;

	let bestIdx = 0;
	let bestScore = -1;

	for (let i = 0; i < candidates.length; i++) {
		const score = scoreSourceMatch(candidates[i], preferred);
		if (score > bestScore) {
			bestScore = score;
			bestIdx = i;
		}
	}

	return bestIdx;
}
