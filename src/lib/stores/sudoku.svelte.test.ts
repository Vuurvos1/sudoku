import { describe, expect, it, vi } from 'vitest';

// the module creates the app's store on import, keep it away from Tauri
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => null) }));
vi.mock('@tauri-apps/plugin-store', () => ({
	LazyStore: class {
		entries = async () => [];
		get = async () => undefined;
		set = async () => {};
	}
}));

const { givenCells, withGivens } = await import('./sudoku.svelte');

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
