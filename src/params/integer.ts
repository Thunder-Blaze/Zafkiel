import type { ParamMatcher } from '@sveltejs/kit';

/** Matches route segments that are purely numeric (e.g. /user/12345) */
export const match: ParamMatcher = (param) => /^\d+$/.test(param);
