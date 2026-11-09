import { describe, expect, it, vi } from 'vitest';

// the module creates the app's store on import, keep it away from Tauri
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => null) }));
vi.mock('@tauri-apps/plugin-store', () => ({
	LazyStore: class {
		entries = async () => [];
		get = async () => undefined;
		set = async () => {};
		save = async () => {};
	}
}));

const { givenCells, withGivens, SudokuStore } = await import('./sudoku.svelte');

const givens = '1' + '.'.repeat(80);

describe('withGivens', () => {
	it('keeps what the player added', () => {
		const saved = givenCells(givens);
		saved[0][1] = { value: 5, notes: [], colors: ['red'], fixed: false };
		saved[0][2] = { value: 0, notes: [2, 3], colors: [], fixed: false, centered: true };
		saved[0][0].colors = ['blue'];

		const board = withGivens(givens, saved);
		expect(board[0][0]).toMatchObject({ value: 1, notes: [], colors: ['blue'], fixed: true });
		expect(board[0][1]).toMatchObject({ value: 5, notes: [], colors: ['red'], fixed: false });
		expect(board[0][2]).toMatchObject({
			value: 0,
			notes: [2, 3],
			colors: [],
			fixed: false,
			centered: true
		});
	});

	it('takes the givens from the puzzle, not the save', () => {
		// a save of another board: no given where the puzzle has one, and one where it hasn't
		const saved = givenCells('.' + '2' + '.'.repeat(79));
		saved[0][0] = { value: 7, notes: [], colors: [], fixed: false };

		const board = withGivens(givens, saved);
		expect(board[0][0]).toMatchObject({ value: 1, notes: [], colors: [], fixed: true });
		expect(board[0][1]).toMatchObject({ value: 0, notes: [], colors: [], fixed: false });
	});
});

describe('the clock', () => {
	it("doesn't count the time the game was paused", () => {
		vi.useFakeTimers({ now: new Date(2026, 0, 1) });
		const store = new SudokuStore();
		store.startTime = Date.now();

		vi.advanceTimersByTime(60_000);
		store.pause();
		vi.advanceTimersByTime(3_600_000);
		expect(store.elapsed()).toBe(60_000);

		store.unpause();
		vi.advanceTimersByTime(30_000);
		expect(store.elapsed()).toBe(90_000);
		vi.useRealTimers();
	});

	it('stays stopped once the puzzle is solved', () => {
		vi.useFakeTimers({ now: new Date(2026, 0, 1) });
		const store = new SudokuStore();
		store.startTime = Date.now();
		vi.advanceTimersByTime(60_000);
		store.endTime = Date.now();

		store.pause();
		vi.advanceTimersByTime(60_000);
		store.unpause();
		expect(store.elapsed()).toBe(60_000);
		vi.useRealTimers();
	});
});
