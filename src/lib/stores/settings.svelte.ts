import { Store } from '@tauri-apps/plugin-store';

export enum NoteStyles {
	Grid,
	Row
}

export type Settings = {
	noteStyle: NoteStyles;
	showTimer: boolean;
	showRemainingDigits: boolean;
};

function createSettingsStore() {
	const store = new Store('oneSudokuSettings.bin');

	// TODO: add theme
	const settings = $state<Settings>({
		noteStyle: NoteStyles.Grid,
		showTimer: false,
		showRemainingDigits: true
	});

	store.get<Settings>('settings').then((val) => {
		if (val) {
			// TODO: improve setting of values with extra checks (some sort of merge)
			for (const key in val) {
				// @ts-expect-error - this is fine
				settings[key] = val[key];
			}
		}
	});

	$effect.root(() => {
		$effect(() => {
			console.log('saving settings');
			settings;
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
