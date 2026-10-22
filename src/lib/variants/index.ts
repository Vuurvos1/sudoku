import type { Component } from 'svelte';
import type { ConstraintSpec } from '$lib/stores';
import Cage from './Cage.svelte';
import Diagonals from './Diagonals.svelte';
import Line from './Line.svelte';
import Marker from './Marker.svelte';
import { borderMidpoint } from './geometry';

/**
 * Board layers a renderer can draw into:
 * - `under`: above cell highlights, below digits (lines, cages)
 * - `over`: above digits and grid lines (dots, X, chevrons)
 */
export type Layer = 'under' | 'over';

type SpecOf<T extends ConstraintSpec['type']> = Extract<ConstraintSpec, { type: T }>;

type Renderer<T extends ConstraintSpec['type']> = {
	layer: Layer;
	/** draws in grid units: cell (r, c) spans (c, r) to (c + 1, r + 1) */
	component: Component<{ constraint: SpecOf<T> }>;
};

const renderers: { [T in ConstraintSpec['type']]?: Renderer<T> } = {
	diagonals: { layer: 'under', component: Diagonals },
	cage: { layer: 'under', component: Cage },
	line: { layer: 'under', component: Line },
	marker: { layer: 'over', component: Marker }
};

export function rendererFor(
	constraint: ConstraintSpec
): { layer: Layer; component: Component<{ constraint: ConstraintSpec }> } | undefined {
	return renderers[constraint.type] as ReturnType<typeof rendererFor>;
}

/** Radius of the gap cut into the grid lines behind each marker kind, in cells. */
const CUTOUT_RADIUS: Partial<Record<Extract<ConstraintSpec, { type: 'marker' }>['kind'], number>> =
	{
		chevron: 0.13,
		x: 0.17
	};

/** Circles to cut out of the grid lines so markers sitting on a border read cleanly. */
export function gridCutouts(constraints: ConstraintSpec[]): { x: number; y: number; r: number }[] {
	return constraints.flatMap((c) => {
		const r = c.type === 'marker' ? CUTOUT_RADIUS[c.kind] : undefined;
		return c.type === 'marker' && r ? [{ ...borderMidpoint(c.cells), r }] : [];
	});
}

/** Short rule name for labels, e.g. "German whispers". */
export function ruleLabel(constraint: ConstraintSpec): string {
	switch (constraint.type) {
		case 'diagonals':
			return 'X-sudoku';
		case 'antiKnight':
			return 'Anti-knight';
		case 'antiKing':
			return 'Anti-king';
		case 'cage':
			return 'Killer';
		case 'line':
			return {
				germanWhisper: 'German whispers',
				dutchWhisper: 'Dutch whispers',
				thermo: 'Thermo'
			}[constraint.kind];
		case 'marker':
			return { white: 'Kropki', black: 'Kropki', x: 'XV', chevron: 'Greater than' }[
				constraint.kind
			];
	}
}

/** Distinct rule names for a puzzle, in first-seen order. */
export function ruleLabels(constraints: ConstraintSpec[]): string[] {
	return [...new Set(constraints.map(ruleLabel))];
}
