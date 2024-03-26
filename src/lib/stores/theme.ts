import { derived, writable } from 'svelte/store';
import { hexToHSL } from '$lib/color';

export const theme = writable({
	background: '#1E1E1E',
	accent: '#F7B155'
});

export const hslTheme = derived(theme, ($theme) => ({
	background: hexToHSL($theme.background),
	accent: hexToHSL($theme.accent)
}));
