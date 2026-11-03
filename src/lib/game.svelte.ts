import type { BoardLayer } from '$lib/board';
import { CELL_COLORS } from '$lib/color';
import type { CellSelection } from '$lib/selection.svelte';
import { HistoryStore } from '$lib/stores/history.svelte';
import type { Settings } from '$lib/stores/settings.svelte';
import type { Sudoku, SudokuStore } from '$lib/stores/sudoku.svelte';
import { cellPosition } from '$lib/variants/geometry';

export type InputMode = 'fill' | 'note' | 'color';

/** The settings a game follows. */
export type GameOptions = Pick<
	Settings,
	'mistakes' | 'autoCleanNotes' | 'highlightSameDigit' | 'highlightHouses'
>;

const DEFAULT_OPTIONS: GameOptions = {
	mistakes: 'rules',
	autoCleanNotes: true,
	highlightSameDigit: true,
	highlightHouses: true
};

type Store = Pick<
	SudokuStore,
	'sudoku' | 'solved' | 'peerMap' | 'copy' | 'restore' | 'save' | 'conflicts'
>;

/** Toggles `item` in every list: removes it if all have it, adds it to the rest otherwise. */
function toggleAll<T>(lists: T[][], item: T): T[][] {
	const allHave = lists.every((list) => list.includes(item));
	return lists.map((list) =>
		allHave ? list.filter((x) => x !== item) : list.includes(item) ? list : [...list, item]
	);
}

/** What the player does to the board: entering digits, notes and colors, with undo. */
export class Game {
	mode = $state<InputMode>('fill');
	readonly history = new HistoryStore<Sudoku>();

	#store: Store;
	#selection: CellSelection;
	#options: () => GameOptions;
	// cells breaking a rule, as of the last `checkRules`
	#conflicts = $state<number[]>([]);
	#checks = 0;

	constructor(
		store: Store,
		selection: CellSelection,
		options: () => GameOptions = () => DEFAULT_OPTIONS
	) {
		this.#store = store;
		this.#selection = selection;
		this.#options = options;
	}

	/**
	 * Finds the digits that break a rule, for the `rules` mistake mode. Reads the board
	 * before awaiting, so an effect calling this reruns whenever a digit changes.
	 */
	async checkRules() {
		const grid = this.#store.sudoku?.flat().map((cell) => cell.value);
		const check = ++this.#checks;
		try {
			const conflicts = grid ? await this.#store.conflicts(grid) : [];
			// a newer check may have finished first
			if (check === this.#checks) this.#conflicts = conflicts;
		} catch (e) {
			// keep the last marks rather than failing on every digit
			console.warn('Checking the rules failed:', e);
		}
	}

	/** Switches to `mode`, or back to filling in digits if it's already on. */
	toggleMode(mode: Exclude<InputMode, 'fill'>) {
		this.mode = this.mode === mode ? 'fill' : mode;
	}

	#cell(index: number) {
		const { row, col } = cellPosition(index);
		return this.#store.sudoku![row][col];
	}

	/** Runs an edit to the selected cells, or the given ones, as one undo step. */
	#edit(apply: (cells: number[]) => void, indices = [...this.#selection.cells]) {
		if (!this.#store.sudoku || indices.length === 0) return;
		if (this.history.current === undefined) this.history.pushState(this.#store.copy());
		apply(indices);
		this.history.pushState(this.#store.copy());
		this.#store.save();
	}

	/**
	 * Enters a digit in the selected cells, as the current mode says. With several cells
	 * selected, fill mode toggles notes instead.
	 */
	enter(digit: number) {
		this.#edit((indices) => {
			const color = CELL_COLORS[digit - 1];
			if (this.mode === 'color') {
				const cells = indices.map((i) => this.#cell(i));
				const colors = toggleAll(
					cells.map((cell) => cell.colors),
					color
				);
				cells.forEach((cell, i) => {
					cell.colors = colors[i];
				});
				return;
			}

			const cells = indices.map((i) => this.#cell(i)).filter((cell) => !cell.fixed);
			if (this.mode === 'note' || indices.length > 1) {
				const notes = toggleAll(
					cells.map((cell) => cell.notes),
					digit
				);
				cells.forEach((cell, i) => {
					cell.notes = notes[i];
				});
				return;
			}

			const [index] = indices;
			const [cell] = cells;
			if (!cell) return;
			cell.value = digit;

			// placing a digit, right or wrong, clears its notes and the same note from its peers
			if (this.#options().autoCleanNotes) {
				cell.notes = [];
				for (const peer of this.#store.peerMap?.[index] ?? []) {
					const peerCell = this.#cell(peer);
					peerCell.notes = peerCell.notes.filter((n) => n !== digit);
				}
			}
		});
	}

	/** Clears the selected cells' colors in color mode, their digits and notes otherwise. */
	clear() {
		this.#edit((indices) => {
			for (const cell of indices.map((i) => this.#cell(i))) {
				if (this.mode === 'color') {
					cell.colors = [];
				} else if (!cell.fixed) {
					cell.value = 0;
					cell.notes = [];
				}
			}
		});
	}

	/**
	 * Notes every digit each empty cell could still take: those none of its peers holds.
	 * Replaces the notes already there, as one undo step.
	 */
	fillNotes() {
		const { peerMap } = this.#store;
		if (!peerMap) return;
		const all = Array.from({ length: 81 }, (_, i) => i);
		this.#edit((indices) => {
			for (const index of indices) {
				const cell = this.#cell(index);
				if (cell.value) continue;
				const seen = peerMap[index].map((peer) => this.#cell(peer).value);
				cell.notes = [1, 2, 3, 4, 5, 6, 7, 8, 9].filter((digit) => !seen.includes(digit));
			}
		}, all);
	}

	undo() {
		const state = this.history.undo();
		if (state) this.#store.restore(state);
	}

	redo() {
		const state = this.history.redo();
		if (state) this.#store.restore(state);
	}

	/** Whether any empty cell has notes, which `fillNotes` would replace. */
	get hasNotes(): boolean {
		return !!this.#store.sudoku?.flat().some((cell) => !cell.value && cell.notes.length > 0);
	}

	/** How many of a digit are still to be placed. */
	remaining(digit: number): number {
		const placed = this.#store.sudoku?.flat().filter((cell) => cell.value === digit).length ?? 0;
		return 9 - placed;
	}

	/** Dev shortcut: fills in the solution except for one cell. */
	almostSolve() {
		const { sudoku, solved } = this.#store;
		if (!sudoku || !solved) return;
		let skipped = false;
		for (let i = 0; i < 9; i++) {
			for (let j = 0; j < 9; j++) {
				if (sudoku[i][j].fixed) continue;
				sudoku[i][j].value = skipped ? solved[i][j] : 0;
				skipped = true;
			}
		}
	}

	// value of the only selected cell, to highlight the same digit elsewhere
	#primaryValue = $derived.by(() => {
		const primary = this.#selection.primary;
		return primary !== null && this.#store.sudoku ? this.#cell(primary).value : 0;
	});

	// cells that see every selected cell
	#peers = $derived.by(() => {
		const { peerMap } = this.#store;
		const selected = this.#selection.cells;
		const [first, ...rest] = selected;
		if (first === undefined || !peerMap) return [];
		return peerMap[first].filter(
			(index) => !selected.has(index) && rest.every((other) => peerMap[other].includes(index))
		);
	});

	#isMistake(index: number): boolean {
		const cell = this.#cell(index);
		switch (this.#options().mistakes) {
			case 'off':
				return false;
			case 'rules':
				return !cell.fixed && this.#conflicts.includes(index);
			case 'solution': {
				const { row, col } = cellPosition(index);
				const solved = this.#store.solved;
				return !!solved && cell.value !== solved[row][col];
			}
		}
	}

	/** Board decorations: the selection, cells it sees, matching digits and mistakes. */
	readonly layer: BoardLayer = {
		cell: (index) => {
			const cell = this.#cell(index);
			const { highlightSameDigit, highlightHouses } = this.#options();
			const value = highlightSameDigit ? this.#primaryValue : 0;
			const selected = this.#selection.cells.has(index);
			const peer = highlightHouses && this.#peers.includes(index);
			const sameDigit = !selected && value > 0 && cell.value === value;
			const matchingNote = !selected && value > 0 && cell.notes.includes(value);
			return {
				'bg-accent/60': selected,
				'bg-accent/20': !selected && (peer || sameDigit),
				'bg-accent/10': !selected && !peer && !sameDigit && matchingNote
			};
		},
		value: (index) => ({ 'mistake text-red-400': this.#isMistake(index) })
	};
}
