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

export class SudokuStore {
	#store = new LazyStore('oneSudoku.bin');

	sudoku = $state<Sudoku | null>(null);
	solved = $state<number[][] | null>(null);
	lastHint = $state<HintResponse | null>(null);
	peerMap = $state<number[][] | null>(null);

	constructor() {
		invoke<number[][]>('get_peers').then((res) => {
			this.peerMap = res;
		});

		this.load();
	}

	async generate(difficulty: number) {
		console.info('generate', difficulty);

		const sudokuString = await invoke<string>('generate_sudoku');
		console.info('sudokuString', sudokuString);

		const solvedRes = await invoke<number[]>('solve_sudoku', { sudokuLine: sudokuString });
		this.solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
		console.info('solved', this.solved);

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

		this.sudoku = grid;
		this.save();
		return grid;
	}

	async save() {
		await this.#store.set('sudoku', this.sudoku);
		await this.#store.save();
	}

	async load() {
		this.sudoku = (await this.#store.get<Sudoku>('sudoku')) ?? null;

		if (!this.sudoku) return;

		const sudokuLine = this.sudoku
			.map((row) => row.map((cell) => (cell.value > 0 ? cell.value : '.')).join(''))
			.join('');

		const solvedRes = await invoke<number[]>('solve_sudoku', { sudokuLine });
		this.solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
	}

	clear() {
		this.sudoku = null;
	}

	async hint() {
		if (!this.sudoku) return;

		const sudokuLine = this.sudoku
			.map((row) => row.map((cell) => (cell.value > 0 ? cell.value : '.')).join(''))
			.join('');

		try {
			this.lastHint = await invoke<HintResponse>('hint_sudoku', { sudokuLine });
		} catch (e) {
			console.warn('No hint available:', e);
			this.lastHint = null;
		}
	}

	copy(): Sudoku {
		return structuredClone($state.snapshot(this.sudoku)) as Sudoku;
	}

	restore(sudoku: Sudoku) {
		this.sudoku = sudoku;
		this.save();
	}
}

export const sudokuStore = new SudokuStore();
