import { describe, expect, it } from 'vitest';
import { CellSelection } from './selection.svelte';

// a board where the pointer's x coordinate is the cell index, and anything negative is off it
const at = (x: number) => ({ clientX: x * 100, clientY: 0, ctrlKey: false, metaKey: false });
const selection = () => new CellSelection((x) => (x < 0 ? null : x / 100));

describe('CellSelection', () => {
	it('selects a tapped cell and deselects it on a second tap', () => {
		const s = selection();
		s.pointerDown(at(3), 3);
		s.pointerUp();
		expect([...s.cells]).toEqual([3]);
		expect(s.primary).toBe(3);

		s.pointerDown(at(3), 3);
		s.pointerUp();
		expect(s.cells.size).toBe(0);
	});

	it('selects every cell a drag passes over', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerMove(at(2));
		s.pointerMove(at(4));
		s.pointerUp();
		expect([...s.cells]).toEqual([1, 2, 4]);
		expect(s.primary).toBeNull();
	});

	it('ignores movement below the drag threshold', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerMove({ ...at(1), clientX: 105 });
		s.pointerUp();
		expect([...s.cells]).toEqual([1]);
	});

	it('narrows a selection to the tapped cell', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerMove(at(2));
		s.pointerUp();
		s.pointerDown(at(2), 2);
		s.pointerUp();
		expect([...s.cells]).toEqual([2]);
	});

	it('starts a new drag from an already selected cell', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerMove(at(2));
		s.pointerUp();
		s.pointerDown(at(2), 2);
		s.pointerMove(at(5));
		s.pointerUp();
		expect([...s.cells]).toEqual([2, 5]);
	});

	it('keeps the selection when a drag from it leaves the board', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerUp();
		s.pointerDown(at(1), 1);
		s.pointerMove(at(-1));
		s.pointerUp();
		expect([...s.cells]).toEqual([1]);
	});

	it('toggles single cells with ctrl', () => {
		const s = selection();
		s.pointerDown(at(1), 1);
		s.pointerUp();
		s.pointerDown({ ...at(4), ctrlKey: true }, 4);
		expect([...s.cells]).toEqual([1, 4]);
		s.pointerDown({ ...at(1), ctrlKey: true }, 1);
		expect([...s.cells]).toEqual([4]);
	});
});
