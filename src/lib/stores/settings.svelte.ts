import { LazyStore } from '@tauri-apps/plugin-store';
import { themeStore, themes } from '$lib/stores/theme.svelte';

export enum NoteStyles {
	Grid,
	Row
}

/**
 * Which digits are marked as mistakes: none, ones that break a rule given the digits
 * around them, or any that differ from the solution.
 */
export type MistakeMode = 'off' | 'rules' | 'solution';

export type Settings = {
	noteStyle: NoteStyles;
	showTimer: boolean;
	showRemainingDigits: boolean;
	mistakes: MistakeMode;
	/** placing a digit removes that note from the cells it sees */
	autoCleanNotes: boolean;
	/** highlight cells with the selected digit, as a value or a note */
	highlightSameDigit: boolean;
	/** shade the row, column, box and other cells the selection sees */
	highlightHouses: boolean;
	/** colors that stay apart with color blindness, plus shape cues; opt-in */
	colorBlindFriendly: boolean;
	/** skip animations and transitions; opt-in */
	reduceMotion: boolean;
	theme: keyof typeof themes;
};

function createSettingsStore() {
	const store = new LazyStore('oneSudokuSettings.bin');

	// TODO: add theme
	const settings = $state<Settings>({
		noteStyle: NoteStyles.Grid,
		showTimer: false,
		showRemainingDigits: true,
		mistakes: 'rules',
		autoCleanNotes: true,
		highlightSameDigit: true,
		highlightHouses: true,
		colorBlindFriendly: false,
		reduceMotion: false,
		theme: 'dark'
	});

	let loaded = $state(false);

	store
		.get<Settings>('settings')
		.then((val) => {
			if (val) {
				// TODO: improve setting of values with extra checks (some sort of merge)
				for (const key in val) {
					// @ts-expect-error - this is fine
					settings[key] = val[key];
				}

				themeStore.setTheme(val.theme);
			}
		})
		.finally(() => (loaded = true));

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
		/** whether the saved settings have been read */
		get loaded() {
			return loaded;
		},
		save
	};
}

export const settingsStore = createSettingsStore();
