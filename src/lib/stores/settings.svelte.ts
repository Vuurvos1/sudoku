import { LazyStore } from '@tauri-apps/plugin-store';
import { themeStore, themes } from '$lib/stores/theme.svelte';

export enum NoteStyles {
	Grid,
	Row
}

export type Settings = {
	noteStyle: NoteStyles;
	showTimer: boolean;
	showRemainingDigits: boolean;
	theme: keyof typeof themes;
};

function createSettingsStore() {
	const store = new LazyStore('oneSudokuSettings.bin');

	// TODO: add theme
	const settings = $state<Settings>({
		noteStyle: NoteStyles.Grid,
		showTimer: false,
		showRemainingDigits: true,
		theme: 'dark'
	});

	store.get<Settings>('settings').then((val) => {
		if (val) {
			// TODO: improve setting of values with extra checks (some sort of merge)
			for (const key in val) {
				// @ts-expect-error - this is fine
				settings[key] = val[key];
			}

			themeStore.setTheme(val.theme);
		}
	});

	$effect.root(() => {
		$effect(() => {
			console.log('saving settings');
			void settings;
			save();
		});
	});

	function save() {
		store.set('settings', settings);
		store.save();
	}

	return {
		settings,
		save
	};
}

export const settingsStore = createSettingsStore();
