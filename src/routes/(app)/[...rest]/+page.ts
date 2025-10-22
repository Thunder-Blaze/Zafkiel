// src/routes/(app)/[...rest]/+page.ts
import { error } from '@sveltejs/kit';

export function load() {
    error(404, 'Not Found');
}
