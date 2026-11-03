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
	| { type: 'marker'; kind: MarkerKind; cells: [number, number] }
	/** circle first, then the shaft up to the arrowhead */
	| { type: 'arrow'; cells: number[] };

export type Puzzle = {
	/** 81 chars, digits for givens and `.` for empty cells */
	givens: string;
	constraints: ConstraintSpec[];
	/** set on ready-made puzzles, whose progress is kept */
	id?: string;
};

/** Where a ready-made puzzle was left. */
export type Progress = {
	sudoku: Sudoku;
	/** time played so far, in ms */
	elapsed: number;
	/** this board is solved, playing again starts over */
	completed: boolean;
	/** solved at some point, it stays solved when played again */
	solved: boolean;
};

/** The starting board of a puzzle: its givens fixed, every other cell empty. */
export function givenCells(givens: string): Sudoku {
	return Array.from({ length: 9 }, (_, row) =>
		Array.from({ length: 9 }, (_, col) => {
			const value = Number(givens[row * 9 + col]) || 0;
			return { value, notes: [], colors: [], fixed: value > 0 };
		})
	);
}

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
	/** ready-made puzzles by id, kept across games */
	progress = $state<Record<string, Progress>>({});
	/** whether the saved game has been read, so the launch splash can wait for it */
	loaded = $state(false);

	get isSolved(): boolean {
		const { sudoku, solved } = this;
		if (!sudoku || !solved) return false;
		return sudoku.every((row, i) => row.every((cell, j) => cell.value === solved[i][j]));
	}

	/** A generated game in progress, which is lost when another one starts. */
	get hasUnsavedGame(): boolean {
		return this.sudoku !== null && !this.puzzle?.id && !this.isSolved;
	}

	constructor() {
		this.load();
	}

	/** Start a new classic puzzle; variant ones are only ready-made. */
	async generate(difficulty: number) {
		this.reset();
		return this.start(await invoke<Puzzle>('generate_sudoku', { difficulty }));
	}

	/** Start a ready-made puzzle, carrying on where it was left unless it was finished. */
	async play({ id, givens, constraints }: Required<Puzzle>) {
		this.reset();
		const puzzle = { id, givens, constraints };
		const saved = this.progress[id];
		if (!saved || saved.completed) return this.start(puzzle);

		await this.prepare(puzzle);
		this.sudoku = $state.snapshot(saved.sudoku);
		// the clock carries on from where it stopped, not from when the puzzle was first opened
		this.startTime = Date.now() - saved.elapsed;
		this.save();
		return this.sudoku;
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

		this.sudoku = givenCells(puzzle.givens);
		this.save();
		return this.sudoku;
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
		const { puzzle, sudoku, startTime, endTime } = this;
		const writes = [
			this.#store.set('puzzle', puzzle),
			this.#store.set('sudoku', sudoku),
			this.#store.set('startTime', startTime),
			this.#store.set('endTime', endTime)
		];

		// a ready-made puzzle also keeps its own entry, so it survives starting another game
		if (puzzle?.id && sudoku && startTime) {
			const completed = this.isSolved;
			const progress: Progress = {
				sudoku: $state.snapshot(sudoku),
				elapsed: (endTime ?? Date.now()) - startTime,
				completed,
				solved: completed || this.progress[puzzle.id]?.solved === true
			};
			this.progress[puzzle.id] = progress;
			writes.push(this.#store.set(`progress:${puzzle.id}`, progress));
		}
		await Promise.all(writes);
		await this.#store.save();
	}

	async load() {
		try {
			await this.#load();
		} finally {
			this.loaded = true;
		}
	}

	async #load() {
		const entries = await this.#store.entries<Progress>();
		this.progress = Object.fromEntries(
			entries
				.filter(([key]) => key.startsWith('progress:'))
				.map(([key, progress]) => [key.slice('progress:'.length), progress])
		);
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

	/** Filled cells in `grid` (81 digits, 0 = empty) that break one of the puzzle's rules. */
	async conflicts(grid: number[]): Promise<number[]> {
		if (!this.puzzle) return [];
		return invoke<number[]>('find_conflicts', { constraints: this.puzzle.constraints, grid });
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
