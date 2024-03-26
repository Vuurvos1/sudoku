import { hexToHSL } from '$lib/color';

export const theme = $state({
	background: '#1E1E1E',
	accent: '#F7B155'
});

const hslThemeStore = $derived.by(() => {
	return {
		background: hexToHSL(theme.background),
		accent: hexToHSL(theme.accent)
	};
});

export const hslTheme = () => hslThemeStore;
