import { invoke } from '@tauri-apps/api/core';
import { Store } from '@tauri-apps/plugin-store';

export type Cell = {
	value: number;
	notes: number[];
	color: string[]; // TODO: enumify?
	fixed: boolean; // one of the original/base digits
};

export type Sudoku = Cell[][];

function createSudokuStore() {
	const store = new Store('oneSudoku.bin');
	let sudoku = $state<Sudoku | null>(null);
	let solved = $state<number[][] | null>(null);

	async function generate(difficulty: number) {
		console.info('generate', difficulty);

		const sudokuString = await invoke<string>('generate_sudoku');
		console.info('sudokuString', sudokuString);

		const solvedRes = await invoke<number[]>('solve_sudoku', { sudokuLine: sudokuString });
		solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
		console.info('solved', solved);

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
		sudoku = await store.get<Sudoku>('sudoku');

		const solvedRes = await invoke<number[]>('solve_sudoku', { sudokuLine: sudoku });
		solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
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
		get solved() {
			return solved;
		},
		generate,
		save,
		load,
		clear,
		hint
	};
}

export const sudokuStore = createSudokuStore();
