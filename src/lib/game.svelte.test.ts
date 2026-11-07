import { describe, expect, it } from 'vitest';
import { CELL_COLORS } from './color';
import { Game, type GameOptions } from './game.svelte';
import { CellSelection } from './selection.svelte';
import type { Sudoku } from './stores/sudoku.svelte';

function setup(overrides: Partial<GameOptions> = {}) {
	const options: GameOptions = {
		mistakes: 'solution',
		autoCleanNotes: true,
		highlightSameDigit: true,
		highlightHouses: true,
		noteStyle: 'corner',
		...overrides
	};
	const sudoku: Sudoku = Array.from({ length: 9 }, () =>
		Array.from({ length: 9 }, () => ({ value: 0, notes: [], colors: [], fixed: false }))
	);
	// R1C1 is a given 1, the solution has 2 in R1C2
	sudoku[0][0] = { value: 1, notes: [], colors: [], fixed: true };
	const solved = Array.from({ length: 9 }, () => Array<number>(9).fill(9));
	solved[0][1] = 2;
	const store = {
		sudoku,
		solved,
		// R1C2 sees R1C3 and R2C2
		peerMap: Object.assign(
			Array.from({ length: 81 }, (): number[] => []),
			{ 1: [2, 10] }
		),
		copy: () => structuredClone(store.sudoku),
		restore: (s: Sudoku) => (store.sudoku = s),
		save: async () => {},
		// pretend R1C2 breaks a rule
		conflicts: async () => [1]
	};
	const selection = new CellSelection(() => null);
	const game = new Game(store, selection, () => options);
	const select = (...cells: number[]) => {
		selection.clear();
		cells.forEach((c) => selection.cells.add(c));
	};
	return { store, game, select };
}

describe('Game', () => {
	it('fills a digit and clears that note from its peers', () => {
		const { store, game, select } = setup();
		store.sudoku[0][1].notes = [2, 4];
		store.sudoku[0][2].notes = [2, 3];
		store.sudoku[1][1].notes = [2];
		select(1);
		game.enter(2);
		expect(store.sudoku[0][1].value).toBe(2);
		expect(store.sudoku[0][1].notes).toEqual([]);
		expect(store.sudoku[0][2].notes).toEqual([3]);
		expect(store.sudoku[1][1].notes).toEqual([]);
	});

	it('cleans notes for a wrong digit too, so it gives nothing away', () => {
		const { store, game, select } = setup({ mistakes: 'rules' });
		store.sudoku[0][2].notes = [3];
		select(1);
		game.enter(3);
		expect(store.sudoku[0][1].value).toBe(3);
		expect(store.sudoku[0][2].notes).toEqual([]);
	});

	it('leaves notes alone when auto-clean is off', () => {
		const { store, game, select } = setup({ autoCleanNotes: false });
		store.sudoku[0][1].notes = [4];
		store.sudoku[0][2].notes = [2];
		select(1);
		game.enter(2);
		expect(store.sudoku[0][1].notes).toEqual([4]);
		expect(store.sudoku[0][2].notes).toEqual([2]);
	});

	it('does not change givens', () => {
		const { store, game, select } = setup();
		select(0);
		game.enter(5);
		game.clear();
		expect(store.sudoku[0][0].value).toBe(1);
	});

	it('toggles notes on several cells in fill mode, skipping givens', () => {
		const { store, game, select } = setup();
		store.sudoku[0][1].notes = [4];
		select(0, 1, 2);
		game.enter(4);
		expect(store.sudoku[0][2].notes).toEqual([4]);
		expect(store.sudoku[0][0].notes).toEqual([]);
		game.enter(4);
		expect(store.sudoku[0][1].notes).toEqual([]);
		expect(store.sudoku[0][2].notes).toEqual([]);
	});

	it('colors and clears colors, givens included', () => {
		const { store, game, select } = setup();
		game.mode = 'color';
		select(0, 1);
		game.enter(3);
		expect(store.sudoku[0][0].colors).toEqual([CELL_COLORS[2]]);
		expect(store.sudoku[0][1].colors).toEqual([CELL_COLORS[2]]);
		game.clear();
		expect(store.sudoku[0][0].colors).toEqual([]);
		expect(store.sudoku[0][1].colors).toEqual([]);
	});

	it('undoes and redoes edits', () => {
		const { store, game, select } = setup();
		select(1);
		game.enter(5);
		game.enter(6);
		game.undo();
		expect(store.sudoku[0][1].value).toBe(5);
		game.undo();
		expect(store.sudoku[0][1].value).toBe(0);
		game.redo();
		expect(store.sudoku[0][1].value).toBe(5);
	});

	it('toggles input modes, falling back to fill', () => {
		const { game } = setup();
		game.toggleMode('note');
		expect(game.mode).toBe('note');
		game.toggleMode('color');
		expect(game.mode).toBe('color');
		game.toggleMode('color');
		expect(game.mode).toBe('fill');
	});

	it('fills the notes of empty cells with what their peers allow, as one undo step', () => {
		const { store, game } = setup();
		expect(game.hasNotes).toBe(false);
		// R1C2 sees R1C3 and R2C2
		store.sudoku[0][2].value = 3;
		store.sudoku[1][1].value = 7;
		store.sudoku[0][3].notes = [5];
		expect(game.hasNotes).toBe(true);
		game.fillNotes();
		expect(store.sudoku[0][1].notes).toEqual([1, 2, 4, 5, 6, 8, 9]);
		expect(store.sudoku[0][3].notes).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9]);
		// filled and given cells keep no notes
		expect(store.sudoku[0][0].notes).toEqual([]);
		expect(store.sudoku[0][2].notes).toEqual([]);
		game.undo();
		expect(store.sudoku[0][1].notes).toEqual([]);
		expect(store.sudoku[0][3].notes).toEqual([5]);
	});

	it('switches notes to the other style and back, as one undo step', () => {
		const { store, game, select } = setup();
		store.sudoku[0][2].centered = true;
		select(1, 2);
		game.toggleNoteStyle();
		expect(store.sudoku[0][1].centered).toBe(true);
		expect(store.sudoku[0][2].centered).toBe(true);
		expect(game.selectionNoteStyle).toBe('center');
		// back to the default, so they follow the setting again
		game.toggleNoteStyle();
		expect(store.sudoku[0][1].centered).toBeUndefined();
		expect(store.sudoku[0][2].centered).toBeUndefined();
		game.undo();
		expect(store.sudoku[0][1].centered).toBe(true);
	});

	it('switches to corner notes when center is the default', () => {
		const { store, game, select } = setup({ noteStyle: 'center' });
		select(1);
		expect(game.otherNoteStyle).toBe('corner');
		game.toggleNoteStyle();
		expect(store.sudoku[0][1].centered).toBe(false);
	});

	it('counts the remaining digits', () => {
		const { game } = setup();
		expect(game.remaining(1)).toBe(8);
		expect(game.remaining(2)).toBe(9);
	});

	it('highlights the selection, its peers and wrong digits', () => {
		const { store, game, select } = setup();
		store.sudoku[0][1].value = 3;
		select(1);
		expect(game.layer.cell!(1)).toMatchObject({ 'bg-accent/60': true });
		expect(game.layer.cell!(10)).toMatchObject({ 'bg-accent/20': true });
		expect(game.layer.cell!(40)).toMatchObject({ 'bg-accent/20': false });
		expect(game.layer.value!(1)).toEqual({ 'mistake text-red-400': true });
	});

	it('can turn off the peer and same-digit highlights', () => {
		const { store, game, select } = setup({ highlightHouses: false, highlightSameDigit: false });
		store.sudoku[0][1].value = 3;
		store.sudoku[4][4].value = 3;
		select(1);
		expect(game.layer.cell!(1)).toMatchObject({ 'bg-accent/60': true });
		expect(game.layer.cell!(10)).toMatchObject({ 'bg-accent/20': false });
		expect(game.layer.cell!(40)).toMatchObject({ 'bg-accent/20': false });
	});

	it('marks mistakes by rules, by the solution, or not at all', async () => {
		const rules = setup({ mistakes: 'rules' });
		rules.store.sudoku[0][1].value = 2;
		rules.store.sudoku[0][2].value = 5;
		expect(rules.game.layer.value!(1)).toEqual({ 'mistake text-red-400': false });
		await rules.game.checkRules();
		expect(rules.game.layer.value!(1)).toEqual({ 'mistake text-red-400': true });
		// wrong by the solution, but no rule says so
		expect(rules.game.layer.value!(2)).toEqual({ 'mistake text-red-400': false });

		const solution = setup({ mistakes: 'solution' });
		solution.store.sudoku[0][2].value = 5;
		expect(solution.game.layer.value!(2)).toEqual({ 'mistake text-red-400': true });

		const off = setup({ mistakes: 'off' });
		off.store.sudoku[0][2].value = 5;
		expect(off.game.layer.value!(2)).toEqual({ 'mistake text-red-400': false });
	});
});
