import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';
import { sections } from '#lib/puzzles/index.ts';

export type Cell = {
	value: number;
	notes: number[];
	colors: string[];
	fixed: boolean; // one of the original/base digits
	/** notes shown centered (true) or in their corners (false); unset follows the setting */
	centered?: boolean;
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
	/** 81 digits; ready-made puzzles can ship it so the app doesn't have to solve them */
	solution?: string;
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

/** The generated game, kept in its own slot so playing a ready-made puzzle doesn't lose it. */
export type GeneratedGame = {
	puzzle: Puzzle;
	sudoku: Sudoku;
	/** time played so far, in ms */
	elapsed: number;
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

/**
 * A saved board laid over the puzzle's givens. The givens always come from the puzzle,
 * so a save can't add, move or drop one; it only brings back what the player added:
 * digits and notes in the other cells, and colors anywhere.
 */
export function withGivens(givens: string, saved: Sudoku): Sudoku {
	return givenCells(givens).map((row, i) =>
		row.map((cell, j) => {
			const old = saved[i]?.[j];
			if (!old) return cell;
			const colors = [...old.colors];
			const centered = old.centered;
			// a digit the save had as a given isn't one the player entered
			if (cell.fixed || old.fixed) return { ...cell, colors, centered };
			return { ...cell, value: old.value, notes: [...old.notes], colors, centered };
		})
	);
}

/** 81 digits as 9 rows of 9. */
function toRows(digits: number[]): number[][] {
	return Array.from({ length: 9 }, (_, i) => digits.slice(i * 9, i * 9 + 9));
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
	/** the generated game in progress, if any; there's only room for one */
	generated = $state<GeneratedGame | null>(null);
	/** whether the saved game has been read, so the launch splash can wait for it */
	loaded = $state(false);

	get isSolved(): boolean {
		const { sudoku, solved } = this;
		if (!sudoku || !solved) return false;
		return sudoku.every((row, i) => row.every((cell, j) => cell.value === solved[i][j]));
	}

	/** A generated game in progress, which is lost when another one is generated. */
	get hasUnsavedGame(): boolean {
		return this.generated !== null;
	}

	/** The game on the board is the generated one, not a ready-made puzzle. */
	get isGenerated(): boolean {
		return this.sudoku !== null && !this.puzzle?.id;
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
	async play({ id, givens, constraints, solution }: Puzzle & { id: string }) {
		// put the current game away with its time up to now, it's picked up again later
		if (this.sudoku) await this.save();
		this.reset();
		const puzzle = { id, givens, constraints, solution };
		const saved = this.progress[id];
		if (!saved || saved.completed) return this.start(puzzle);
		return this.resume(puzzle, saved);
	}

	/** Carry on with the generated game after playing ready-made puzzles. */
	async continueGenerated() {
		const saved = this.generated;
		if (!saved) return null;
		if (this.isGenerated) return this.sudoku;
		if (this.sudoku) await this.save();
		this.reset();
		return this.resume(saved.puzzle, saved);
	}

	private async resume(puzzle: Puzzle, saved: { sudoku: Sudoku; elapsed: number }) {
		// another game was opened while this one loaded, don't save over it
		if (!(await this.prepare(puzzle))) return null;
		this.sudoku = withGivens(puzzle.givens, $state.snapshot(saved.sudoku));
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
		// another game was opened while this one loaded, don't save over it
		if (!(await this.prepare(puzzle))) return null;
		this.sudoku = givenCells(puzzle.givens);
		this.save();
		return this.sudoku;
	}

	/** Counts calls to `prepare`, so a load that was overtaken by a newer one can tell. */
	#loads = 0;

	/**
	 * Fetch the puzzle's peer table, and its solution if it ships one. Otherwise the
	 * solution is worked out in the background, which can take minutes on hard
	 * variants, so the board doesn't wait for it; `solved` stays null until then.
	 * Resolves to false if another puzzle was prepared in the meantime.
	 */
	async prepare(puzzle: Puzzle): Promise<boolean> {
		const load = ++this.#loads;
		const current = () => load === this.#loads;
		this.puzzle = puzzle;
		if (puzzle.solution) {
			this.solved = toRows(puzzle.solution.split('').map(Number));
		} else {
			invoke<number[]>('solve_sudoku', { puzzle })
				.then((res) => {
					if (current()) this.solved = toRows(res);
				})
				.catch((e) => console.warn('Could not solve the puzzle:', e));
		}
		const peers = await invoke<number[][]>('get_peers', { constraints: puzzle.constraints });
		if (!current()) return false;
		this.peerMap = peers;
		return true;
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
		} else if (puzzle && sudoku && startTime) {
			// a finished generated game has nothing left to come back to
			this.generated = this.isSolved
				? null
				: {
						puzzle: $state.snapshot(puzzle),
						sudoku: $state.snapshot(sudoku),
						elapsed: (endTime ?? Date.now()) - startTime
					};
			writes.push(this.#store.set('generated', this.generated));
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
		this.generated = (await this.#store.get<GeneratedGame>('generated')) ?? null;
		const sudoku = (await this.#store.get<Sudoku>('sudoku')) ?? null;
		this.sudoku = sudoku;
		this.startTime = (await this.#store.get<number>('startTime')) ?? null;
		this.endTime = (await this.#store.get<number>('endTime')) ?? null;

		if (!sudoku) return;

		// saves from before variants only stored the grid; rebuild from its fixed cells
		const saved = (await this.#store.get<Puzzle>('puzzle')) ?? {
			givens: sudoku
				.map((row) => row.map((cell) => (cell.fixed ? cell.value : '.')).join(''))
				.join(''),
			constraints: []
		};
		// a ready-made puzzle is taken from the app, which may know its solution by now
		const ready = saved.id
			? sections.flatMap((s) => s.puzzles).find((p) => p.id === saved.id)
			: undefined;
		const puzzle: Puzzle = ready
			? {
					id: ready.id,
					givens: ready.givens,
					constraints: ready.constraints,
					solution: ready.solution
				}
			: saved;
		this.sudoku = withGivens(puzzle.givens, sudoku);

		// solving can take a while on hard puzzles, so it doesn't hold up the launch
		this.prepare(puzzle)
			.then(() => {
				// saves from before the generated game had its own slot
				if (this.isGenerated && !this.generated && !this.isSolved) this.save();
			})
			.catch((e) => console.warn('Could not solve the saved puzzle:', e));
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

	/** Start the puzzle over from its givens, with the clock back at zero. */
	restart() {
		if (!this.puzzle) return;
		this.sudoku = givenCells(this.puzzle.givens);
		this.startTime = Date.now();
		this.endTime = null;
		this.save();
	}
}

export const sudokuStore = new SudokuStore();
