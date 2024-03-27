import { hexToHSL } from '$lib/color';

export type Theme = {
	id: number;
	background: string;
	accent: string;
};

export const themes: Record<string, Theme> = {
	light: {
		id: 1,
		background: '#FFFFFF',
		accent: '#F7B155'
	},
	dark: {
		id: 2,
		background: '#1E1E1E',
		accent: '#F7B155'
	}
};

export const theme = $state<Theme>({ ...themes.dark });

const hslThemeStore = $derived.by(() => {
	return {
		background: hexToHSL(theme.background),
		accent: hexToHSL(theme.accent)
	};
});

// export const hslTheme = () => hslThemeStore;

$effect.root(() => {
	$effect(() => {
		const doc = document.documentElement;

		if (!doc || !hslThemeStore.accent || !hslThemeStore.background) return;

		doc.style.setProperty('--accent-h', String(hslThemeStore.accent.h));
		doc.style.setProperty('--accent-s', String(hslThemeStore.accent.s) + '%');
		doc.style.setProperty('--accent-l', String(hslThemeStore.accent.l) + '%');

		doc.style.setProperty('--background-h', String(hslThemeStore.background.h));
		doc.style.setProperty('--background-s', String(hslThemeStore.background.s) + '%');
		doc.style.setProperty('--background-l', String(hslThemeStore.background.l) + '%');
	});

	return () => {
		// effect cleanup
	};
});
