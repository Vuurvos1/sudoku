import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';

export type Cell = {
	value: number;
	notes: number[];
	colors: string[];
	fixed: boolean; // one of the original/base digits
};

export type Sudoku = Cell[][];

export type LineKind = 'germanWhisper' | 'dutchWhisper' | 'thermo';
/** `chevron`: the first cell is greater, the chevron points at the second */
export type MarkerKind = 'white' | 'black' | 'x' | 'chevron';

/**
 * Variant rules, mirrors `ConstraintSpec` in the solver crate.
 * Cells are indices 0..81 (row * 9 + col).
 */
export type ConstraintSpec =
	| { type: 'diagonals' }
	| { type: 'antiKnight' }
	| { type: 'antiKing' }
	| { type: 'cage'; cells: number[]; sum: number | null }
	| { type: 'line'; kind: LineKind; cells: number[] }
	| { type: 'marker'; kind: MarkerKind; cells: [number, number] };

export type Puzzle = {
	/** 81 chars, digits for givens and `.` for empty cells */
	givens: string;
	constraints: ConstraintSpec[];
};

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

	puzzle = $state<Puzzle | null>(null);
	sudoku = $state<Sudoku | null>(null);
	solved = $state<number[][] | null>(null);
	lastHint = $state<HintResponse | null>(null);
	peerMap = $state<number[][] | null>(null);
	startTime = $state<number | null>(null);
	endTime = $state<number | null>(null);

	get isSolved(): boolean {
		if (!this.sudoku || !this.solved) return false;

		for (let i = 0; i < 9; i++) {
			for (let j = 0; j < 9; j++) {
				if (this.sudoku[i][j].value === 0) return false;
				if (this.sudoku[i][j].value !== this.solved[i][j]) return false;
			}
		}

		return true;
	}

	constructor() {
		this.load();
	}

	async generate(difficulty: number, constraints: ConstraintSpec[] = []) {
		console.info('generate', difficulty, constraints);
		this.reset();

		const puzzle = await invoke<Puzzle>('generate_sudoku', { difficulty, constraints });
		console.info('puzzle', puzzle);
		return this.start(puzzle);
	}

	/** Start the hand-made test puzzle that uses every variant marker. */
	async showcase() {
		this.reset();
		return this.start(await invoke<Puzzle>('showcase_puzzle'));
	}

	private reset() {
		this.puzzle = null;
		this.sudoku = null;
		this.solved = null;
		this.startTime = Date.now();
		this.endTime = null;
	}

	private async start(puzzle: Puzzle) {
		await this.prepare(puzzle);

		const grid: Sudoku = Array.from({ length: 9 }, () =>
			Array(9)
				.fill(0)
				.map(() => {
					return {
						value: 0,
						notes: [],
						colors: [],
						fixed: false
					};
				})
		);

		const digits = puzzle.givens.split('');

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

	/** Solve the puzzle and fetch its peer table. */
	async prepare(puzzle: Puzzle) {
		this.puzzle = puzzle;
		const [solvedRes, peers] = await Promise.all([
			invoke<number[]>('solve_sudoku', { puzzle }),
			invoke<number[][]>('get_peers', { constraints: puzzle.constraints })
		]);
		this.solved = Array.from({ length: 9 }, (_, i) => solvedRes.slice(i * 9, i * 9 + 9));
		this.peerMap = peers;
	}

	async save() {
		await this.#store.set('puzzle', this.puzzle);
		await this.#store.set('sudoku', this.sudoku);
		await this.#store.set('startTime', this.startTime);
		await this.#store.set('endTime', this.endTime);
		await this.#store.save();
	}

	async load() {
		this.sudoku = (await this.#store.get<Sudoku>('sudoku')) ?? null;
		this.startTime = (await this.#store.get<number>('startTime')) ?? null;
		this.endTime = (await this.#store.get<number>('endTime')) ?? null;

		if (!this.sudoku) return;

		// saves from before variants only stored the grid; rebuild from its fixed cells
		const puzzle = (await this.#store.get<Puzzle>('puzzle')) ?? {
			givens: this.sudoku
				.map((row) => row.map((cell) => (cell.fixed ? cell.value : '.')).join(''))
				.join(''),
			constraints: []
		};
		await this.prepare(puzzle);
	}

	clear() {
		this.puzzle = null;
		this.sudoku = null;
	}

	async hint() {
		if (!this.sudoku || !this.puzzle) return;

		const givens = this.sudoku
			.map((row) => row.map((cell) => (cell.value > 0 ? cell.value : '.')).join(''))
			.join('');

		try {
			this.lastHint = await invoke<HintResponse>('hint_sudoku', {
				puzzle: { givens, constraints: this.puzzle.constraints }
			});
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
