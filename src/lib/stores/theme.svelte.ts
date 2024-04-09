import { hexToHSL } from '$lib/color';

export type Theme = {
	id: number;
	background: string;
	accent: string;
	text: string;
	colorScheme: 'light' | 'dark';
};

export const themes: Record<string, Theme> = {
	light: {
		id: 1,
		background: '#FFFFFF',
		accent: '#F7B155',
		text: '#000000',
		colorScheme: 'light'
	},
	dark: {
		id: 2,
		background: '#1E1E1E',
		accent: '#F7B155',
		text: '#FFFFFF',
		colorScheme: 'dark'
	},
	blue: {
		id: 3,
		background: '#FFFFFF',
		accent: '#38BDF8',
		text: '#111827',
		colorScheme: 'light'
	},
	purple: {
		id: 4,
		background: '#FFFFFF',
		accent: '#4F46E5',
		text: '#111827',
		colorScheme: 'light'
	}
};

export const theme = $state<Theme>({ ...themes.dark });

const hslThemeStore = $derived.by(() => {
	return {
		background: hexToHSL(theme.background),
		accent: hexToHSL(theme.accent),
		text: hexToHSL(theme.text)
	};
});

// export const hslTheme = () => hslThemeStore;

$effect.root(() => {
	$effect(() => {
		const doc = document.documentElement;

		if (!doc || !hslThemeStore.accent || !hslThemeStore.background || !hslThemeStore.text) return;

		doc.style.colorScheme = theme?.colorScheme ?? 'light';

		doc.style.setProperty('--accent-h', String(hslThemeStore.accent.h));
		doc.style.setProperty('--accent-s', String(hslThemeStore.accent.s) + '%');
		doc.style.setProperty('--accent-l', String(hslThemeStore.accent.l) + '%');

		doc.style.setProperty('--background-h', String(hslThemeStore.background.h));
		doc.style.setProperty('--background-s', String(hslThemeStore.background.s) + '%');
		doc.style.setProperty('--background-l', String(hslThemeStore.background.l) + '%');

		doc.style.setProperty('--text-h', String(hslThemeStore.text.h));
		doc.style.setProperty('--text-s', String(hslThemeStore.text.s) + '%');
		doc.style.setProperty('--text-l', String(hslThemeStore.text.l) + '%');
	});

	return () => {
		// effect cleanup
	};
});
