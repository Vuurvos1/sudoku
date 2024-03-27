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
