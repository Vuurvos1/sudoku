/** Grid units: cell (r, c) spans (c, r) to (c + 1, r + 1). */
export type Point = { x: number; y: number };

export function cellIndex(row: number, col: number): number {
	return row * 9 + col;
}

export function cellPosition(index: number): { row: number; col: number } {
	return { row: Math.floor(index / 9), col: index % 9 };
}

export function cellCenter(cell: number): Point {
	return { x: (cell % 9) + 0.5, y: Math.floor(cell / 9) + 0.5 };
}

/** Midpoint of the border between two orthogonally adjacent cells. */
export function borderMidpoint([a, b]: [number, number]): Point {
	const pa = cellCenter(a);
	const pb = cellCenter(b);
	return { x: (pa.x + pb.x) / 2, y: (pa.y + pb.y) / 2 };
}

/**
 * Inset outline around a group of cells, as an SVG path (one segment per cell side);
 * an inset of 0 traces the cell borders.
 * `cutout` clears a rectangle (measured from the cell corner) at the top-left of the
 * lowest-index cell; the two sides it crosses start there, so dashes resume whole.
 */
export function regionOutline(
	cells: number[],
	inset: number,
	cutout?: { width: number; height: number }
): string {
	const set = new Set(cells);
	const first = Math.min(...cells);
	const has = (r: number, c: number) => r >= 0 && r < 9 && c >= 0 && c < 9 && set.has(r * 9 + c);
	// outward normal and direction along the side, as [dr, dc]
	const sides = [
		{ out: [-1, 0], along: [0, 1] },
		{ out: [0, 1], along: [1, 0] },
		{ out: [1, 0], along: [0, -1] },
		{ out: [0, -1], along: [-1, 0] }
	];

	const segments: string[] = [];
	for (const cell of cells) {
		const r = Math.floor(cell / 9);
		const c = cell % 9;
		for (const { out, along } of sides) {
			if (has(r + out[0], c + out[1])) continue;
			// how far the side reaches towards each end: short of the corner when
			// the cage turns there, past it to meet an inner corner
			const reach = (s: number) => {
				const nr = r + s * along[0];
				const nc = c + s * along[1];
				if (!has(nr, nc)) return 0.5 - inset;
				return has(nr + out[0], nc + out[1]) ? 0.5 + inset : 0.5;
			};
			const point = (s: number) => {
				const t = s * reach(s);
				const x = c + 0.5 + out[1] * (0.5 - inset) + along[1] * t;
				const y = r + 0.5 + out[0] * (0.5 - inset) + along[0] * t;
				return { x, y };
			};
			let from = point(-1);
			let to = point(1);
			if (cutout && cell === first && out[0] === -1) {
				// top side, left to right: start past the cutout
				from = { x: Math.max(from.x, c + cutout.width), y: from.y };
				if (from.x >= to.x) continue;
			} else if (cutout && cell === first && out[1] === -1) {
				// left side runs upwards; flip it to start below the cutout
				[from, to] = [{ x: to.x, y: Math.max(to.y, r + cutout.height) }, from];
				if (from.y >= to.y) continue;
			}
			segments.push(`M${from.x},${from.y}L${to.x},${to.y}`);
		}
	}
	return segments.join('');
}
