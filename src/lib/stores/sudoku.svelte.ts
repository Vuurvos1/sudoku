import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';

export type Cell = {
	value: number;
	notes: number[];
	color: string[]; // TODO: enumify?
	fixed: boolean; // one of the original/base digits
};

export type Sudoku = Cell[][];

export type HintResponse = {
	strategy: string;
	difficulty: number;
	summary: string;
	detail: string;
	placements: { cell: number; digit: number }[];
	eliminations: { cell: number; digit: number }[];
	highlights: {
		cells: { cell: number; role: string }[];
		candidates: { cell: number; digit: number; role: string }[];
	}[];
};

function createSudokuStore() {
	const store = new LazyStore('oneSudoku.bin');

	let sudoku = $state<Sudoku | null>(null);
	let solved = $state<number[][] | null>(null);
	let lastHint = $state<HintResponse | null>(null);
	let peerMap = $state<number[][] | null>(null);

	// Fetch the peer map from the constraint system (cached, only needs to be called once per puzzle type)
	invoke<number[][]>('get_peers').then((res) => {
		peerMap = res;
	});

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
		await store.save();
	}

	async function load() {
		sudoku = (await store.get<Sudoku>('sudoku')) ?? null;

		if (!sudoku) return;

		const sudokuLine = sudoku
			.map((row) => row.map((cell) => (cell.value > 0 ? cell.value : '.')).join(''))
			.join('');

		const solvedRes = await invoke<number[]>('solve_sudoku', { sudokuLine });
		solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
	}

	function clear() {
		sudoku = null;
	}

	async function hint() {
		if (!sudoku) return;

		const sudokuLine = sudoku
			.map((row) => row.map((cell) => (cell.value > 0 ? cell.value : '.')).join(''))
			.join('');

		try {
			lastHint = await invoke<HintResponse>('hint_sudoku', { sudokuLine });
		} catch (e) {
			console.warn('No hint available:', e);
			lastHint = null;
		}
	}

	load();

	return {
		get sudoku() {
			return sudoku;
		},
		get solved() {
			return solved;
		},
		get lastHint() {
			return lastHint;
		},
		get peerMap() {
			return peerMap;
		},
		generate,
		save,
		load,
		clear,
		hint
	};
}

export const sudokuStore = createSudokuStore();
