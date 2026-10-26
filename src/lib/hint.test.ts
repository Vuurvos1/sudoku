import { describe, expect, it } from 'vitest';
import { hintLayer } from './hint';
import type { HintResponse } from './stores/sudoku.svelte';

const hint: HintResponse = {
	strategy: 'Naked single',
	difficulty: 1,
	summary: '',
	detail: '',
	placements: [{ cell: 0, digit: 5 }],
	eliminations: [{ cell: 2, digit: 3 }],
	highlights: [
		{
			cells: [
				{ cell: 0, role: 'focus' },
				{ cell: 1, role: 'supporting' }
			],
			candidates: [
				{ cell: 1, digit: 4, role: 'supporting' },
				{ cell: 1, digit: 4, role: 'focus' }
			]
		}
	]
};

describe('hintLayer', () => {
	const layer = hintLayer(hint);

	it('highlights cells by role', () => {
		expect(layer.cell!(0)).toMatchObject({ 'bg-accent/40': true });
		expect(layer.cell!(1)).toMatchObject({ 'bg-accent/20': true, 'bg-accent/40': false });
	});

	it('dims cells the hint does not mention', () => {
		expect(layer.content!(2)).toEqual({ 'opacity-20': false });
		expect(layer.content!(40)).toEqual({ 'opacity-20': true });
	});

	it('shows placements as accented digits', () => {
		expect(layer.placement!(0)).toBe(5);
		expect(layer.placement!(1)).toBeUndefined();
		expect(layer.value!(0)).toEqual({ 'font-bold text-accent': true });
	});

	it('prefers focus over other roles and strikes eliminations', () => {
		expect(layer.note!(1, 4)).toBe('font-bold text-accent');
		expect(layer.note!(2, 3)).toBe('text-red-400 line-through');
		expect(layer.note!(2, 4)).toBeUndefined();
	});
});
