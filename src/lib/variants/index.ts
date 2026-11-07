import type { Component } from 'svelte';
import type { ConstraintSpec } from '$lib/stores';
import type { RuleGlyph } from '$lib/learn';
import Arrow from './Arrow.svelte';
import Cage from './Cage.svelte';
import Diagonals from './Diagonals.svelte';
import Line from './Line.svelte';
import Marker from './Marker.svelte';
import { borderMidpoint } from './geometry';

/**
 * Board layers a renderer can draw into:
 * - `under`: above cell highlights, below digits (lines, cages, arrows)
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
	arrow: { layer: 'under', component: Arrow },
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
		x: 0.13
	};
/** Extra space between a marker and where the lines around it resume, in cells. */
const CUTOUT_PADDING = 0.03;

/** Circles to cut out of the grid, cages and lines so markers sitting on a border read cleanly. */
export function gridCutouts(constraints: ConstraintSpec[]): { x: number; y: number; r: number }[] {
	return constraints.flatMap((c) => {
		const r = c.type === 'marker' ? CUTOUT_RADIUS[c.kind] : undefined;
		return c.type === 'marker' && r ? [{ ...borderMidpoint(c.cells), r: r + CUTOUT_PADDING }] : [];
	});
}

/** Short rule name for labels, e.g. "German whispers". */
export function ruleLabel(constraint: ConstraintSpec): string {
	switch (constraint.type) {
		case 'diagonals':
			return 'X-sudoku';
		case 'antiKnight':
			return "Knight's move";
		case 'antiKing':
			return "King's move";
		case 'cage':
			return 'Cages';
		case 'line':
			return {
				germanWhisper: 'German whispers',
				dutchWhisper: 'Dutch whispers',
				thermo: 'Thermo'
			}[constraint.kind];
		case 'arrow':
			return 'Arrow';
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

/** The rules of a puzzle, "Classic" when it has no variant rules. */
export function puzzleRules(constraints: ConstraintSpec[]): string[] {
	return constraints.length > 0 ? ruleLabels(constraints) : ['Classic'];
}

/** The icon drawn for a rule, see `RuleIcon`. */
export function ruleGlyph(constraint: ConstraintSpec): RuleGlyph {
	switch (constraint.type) {
		case 'line':
			return constraint.kind === 'thermo' ? 'thermo' : 'whisper';
		case 'marker':
			return { white: 'kropki', black: 'kropki', x: 'xv', chevron: 'chevron' }[
				constraint.kind
			] as RuleGlyph;
		default:
			return constraint.type;
	}
}

/** One icon per distinct rule, with its name for screen readers, in first-seen order. */
export function ruleIcons(constraints: ConstraintSpec[]): { glyph: RuleGlyph; label: string }[] {
	const seen = new Map<string, RuleGlyph>();
	for (const c of constraints) if (!seen.has(ruleLabel(c))) seen.set(ruleLabel(c), ruleGlyph(c));
	return [...seen].map(([label, glyph]) => ({ glyph, label }));
}
