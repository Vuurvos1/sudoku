import { describe, expect, it } from 'vitest';
import { borderMidpoint, regionOutline, cellCenter } from './geometry';

/** Segments of an outline path as [[x1, y1], [x2, y2]], rounded to avoid float noise. */
function segments(path: string): number[][][] {
	return path
		.split('M')
		.filter(Boolean)
		.map((seg) =>
			seg.split('L').map((p) => p.split(',').map((n) => Math.round(Number(n) * 100) / 100))
		);
}

describe('cellCenter', () => {
	it('maps an index to the middle of its cell', () => {
		expect(cellCenter(0)).toEqual({ x: 0.5, y: 0.5 });
		expect(cellCenter(80)).toEqual({ x: 8.5, y: 8.5 });
		expect(cellCenter(10)).toEqual({ x: 1.5, y: 1.5 });
	});
});

describe('borderMidpoint', () => {
	it('sits on the shared border of two neighbours', () => {
		expect(borderMidpoint([0, 1])).toEqual({ x: 1, y: 0.5 });
		expect(borderMidpoint([9, 0])).toEqual({ x: 0.5, y: 1 });
	});
});

describe('regionOutline', () => {
	it('draws a single cell as an inset square', () => {
		expect(segments(regionOutline([0], 0.1))).toEqual([
			[
				[0.1, 0.1],
				[0.9, 0.1]
			],
			[
				[0.9, 0.1],
				[0.9, 0.9]
			],
			[
				[0.9, 0.9],
				[0.1, 0.9]
			],
			[
				[0.1, 0.9],
				[0.1, 0.1]
			]
		]);
	});

	it('only outlines the outside of a two-cell cage', () => {
		// R1C1 + R1C2: no line on the shared side
		const segs = segments(regionOutline([0, 1], 0.1));
		expect(segs).toHaveLength(6);
		expect(segs.some((s) => s.every(([x]) => x === 1))).toBe(false);
	});

	it('meets at the inner corner of an L-shaped cage', () => {
		// R1C1, R1C2, R2C1: inner corner sits inset at (0.9, 0.9)
		const segs = segments(regionOutline([0, 1, 9], 0.1));
		const ends = segs.flat().filter(([x, y]) => x === 0.9 && y === 0.9);
		expect(ends).toHaveLength(2);
	});

	it('traces the cell borders without an inset', () => {
		// R1C1 + R1C2, as for a selection outline
		const segs = segments(regionOutline([0, 1], 0));
		expect(segs).toHaveLength(6);
		expect(segs).toContainEqual([
			[0, 0],
			[1, 0]
		]);
		expect(segs).toContainEqual([
			[2, 1],
			[1, 1]
		]);
	});

	it('starts the sides crossing the cutout at its edges', () => {
		// R2C3 is the first cell; both trimmed sides run away from the cutout
		const segs = segments(regionOutline([11, 12], 0.1, { width: 0.3, height: 0.25 }));
		expect(segs).toContainEqual([
			[2.3, 1.1],
			[3, 1.1]
		]);
		expect(segs).toContainEqual([
			[2.1, 1.25],
			[2.1, 1.9]
		]);
	});
});
