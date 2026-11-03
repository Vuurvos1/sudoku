import type { ClassValue } from 'svelte/elements';

type Classes = ClassValue | null | undefined;

/**
 * Decorations drawn on top of a board's cells, such as the selection highlights or a hint.
 * A board takes a list of layers and applies every one of them, in order.
 */
export type BoardLayer = {
	/** background of a cell, below the variant shapes */
	cell?: (index: number) => Classes;
	/** everything inside a cell (digit or notes), e.g. to dim it */
	content?: (index: number) => Classes;
	/** the cell's digit, including one from `placement` */
	value?: (index: number) => Classes;
	/** a single note */
	note?: (index: number, digit: number) => Classes;
	/** digit to show in an empty cell, e.g. the one a hint places */
	placement?: (index: number) => number | undefined;
};
