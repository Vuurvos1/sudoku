// import { createEmptyGrid } from '$lib/sudoku';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@tauri-apps/plugin-store';

// TODO: change sudoku where each cell has an object with value and notes?
// [[{value: 0, notes: [], color: [], filled, false}]]
// static, filled, base, fixed??

export type Cell = {
	value: number;
	notes: number[];
	color: string[]; // TODO: enumify?
	fixed: boolean; // one of the original numbers
};

export type Sudoku = Cell[][];

function createSudokuStore() {
	const store = new Store('oneSudoku.bin');
	let sudoku = $state<Sudoku | null>(null);

	async function generate(difficulty: number) {
		console.log('generate', difficulty);

		const sudokuString: string = await invoke('generate_sudoku');
		console.log('sudokuString', sudokuString);

		const grid: Sudoku = Array.from({ length: 9 }, () =>
			Array(9)
				.fill(0)
				.map(() => {
					return {
						value: 0,
						notes: [],
						color: [],
						fixed: false
					};
				})
		);

		const digits = sudokuString.split('');

		for (let i = 0; i < digits.length; i++) {
			if (digits[i] === '.') continue;

			const row = Math.floor(i / 9);
			const col = i % 9;
			grid[row][col].value = parseInt(digits[i]);
			grid[row][col].fixed = true;
		}

		sudoku = grid;
		save();
		return grid;
	}

	async function save() {
		await store.set('sudoku', sudoku);

		// You can manually save the store after making changes.
		// Otherwise, it will save upon graceful exit.
		await store.save();
	}

	async function load() {
		const val = await store.get<Sudoku>('sudoku');
		console.log(val);
		sudoku = val;
	}

	function clear() {
		sudoku = null;
	}

	async function hint() {
		console.info('hint');
	}

	load();

	return {
		get sudoku() {
			return sudoku;
		},
		generate,
		save,
		load,
		clear,
		hint
	};
}

export const sudokuStore = createSudokuStore();
