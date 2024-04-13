import { hexToHSL } from '$lib/color';
import { settingsStore } from '$lib/stores';

export type Theme = {
	id: number;
	background: string;
	accent: string;
	text: string;
	colorScheme: 'light' | 'dark';
};

// save theme key to settings
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

function createThemeStore() {
	let theme = $state<Theme>({ ...themes.dark });

	function setTheme(key: string) {
		if (themes[key]) {
			theme = { ...themes[key] };
			settingsStore.settings.theme = key;
		} else {
			console.error(`Theme key ${key} not found`);
		}
	}

	return {
		get() {
			return theme;
		},
		setTheme
	};
}

export const themeStore = createThemeStore();

const hslThemeStore = $derived.by(() => {
	return {
		background: hexToHSL(themeStore.get().background),
		accent: hexToHSL(themeStore.get().accent),
		text: hexToHSL(themeStore.get().text)
	};
});

$effect.root(() => {
	$effect(() => {
		const doc = document.documentElement;

		if (!doc || !hslThemeStore.accent || !hslThemeStore.background || !hslThemeStore.text) return;

		doc.style.colorScheme = themeStore.get()?.colorScheme ?? 'light';

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
