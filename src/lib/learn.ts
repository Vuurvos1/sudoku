import type { BoardLayer } from '#lib/board.ts';
import type { ConstraintSpec, Sudoku } from '#lib/stores/index.ts';

export type RuleGlyph =
	| 'diagonals'
	| 'antiKnight'
	| 'antiKing'
	| 'cage'
	| 'thermo'
	| 'arrow'
	| 'whisper'
	| 'kropki'
	| 'xv'
	| 'chevron';

export type Rule = {
	slug: string;
	glyph: RuleGlyph;
	/** matches `ruleLabel` in #lib/variants */
	name: string;
	summary: string;
	tips: string[];
	example: {
		constraints: ConstraintSpec[];
		/** cell index → digit */
		digits: Record<number, number>;
		/** cells to highlight, e.g. the ones a rule rules out */
		highlight?: number[];
		/** rows of the board to show, from the top */
		rows: number;
		caption: string;
	};
};

export const rules: Rule[] = [
	{
		slug: 'x-sudoku',
		glyph: 'diagonals',
		name: 'X-sudoku',
		summary: "Digits can't repeat on either long diagonal.",
		tips: [
			'Each diagonal holds 1 to 9 exactly once, so treat it like an extra row.',
			'The centre cell lies on both diagonals.'
		],
		example: {
			constraints: [{ type: 'diagonals' }],
			digits: { 0: 3, 10: 7, 20: 1, 30: 9, 16: 4, 24: 6 },
			rows: 4,
			caption: 'No digit appears twice along a diagonal.'
		}
	},
	{
		slug: 'knights-move',
		glyph: 'antiKnight',
		name: "Knight's move",
		summary: "Cells a chess knight's move apart can't hold the same digit.",
		tips: [
			'A placed digit rules out up to eight more cells.',
			"Inside a box, a digit's knight moves often leave it only one or two cells."
		],
		example: {
			constraints: [{ type: 'antiKnight' }],
			digits: { 21: 5 },
			highlight: [2, 4, 10, 14, 28, 32, 38, 40],
			rows: 5,
			caption: "None of the marked cells can be 5: they're a knight's move away."
		}
	},
	{
		slug: 'kings-move',
		glyph: 'antiKing',
		name: "King's move",
		summary: "Cells that touch, even diagonally, can't hold the same digit.",
		tips: [
			'Within a box this adds nothing new; it matters across box borders.',
			'Two equal digits in neighbouring boxes are at least two columns apart.'
		],
		example: {
			constraints: [{ type: 'antiKing' }],
			digits: { 21: 5 },
			highlight: [11, 12, 13, 20, 22, 29, 30, 31],
			rows: 4,
			caption: 'None of the touching cells can be 5.'
		}
	},
	{
		slug: 'cages',
		glyph: 'cage',
		name: 'Cages',
		summary: "Digits in a cage add up to its small number and don't repeat.",
		tips: [
			'Some sums have one combination: 3 is 1+2, 4 is 1+3, 16 is 7+9, 17 is 8+9.',
			'Every row, column and box adds up to 45. Subtract the cages inside one to find the rest.'
		],
		example: {
			constraints: [
				{ type: 'cage', cells: [0, 1, 2], sum: 15 },
				{ type: 'cage', cells: [12, 13], sum: 17 }
			],
			digits: { 0: 4, 1: 5, 2: 6, 12: 8, 13: 9 },
			rows: 3,
			caption: '4 + 5 + 6 = 15, and 17 in two cells can only be 8 + 9.'
		}
	},
	{
		slug: 'thermo',
		glyph: 'thermo',
		name: 'Thermo',
		summary: 'Digits increase from the bulb to the end of the line.',
		tips: [
			"The bulb is the smallest digit, so it can't be 9.",
			'On a thermo of n cells, the k-th cell is at least k and at most 9 − (n − k).'
		],
		example: {
			constraints: [{ type: 'line', kind: 'thermo', cells: [9, 10, 11, 12] }],
			digits: { 9: 2, 10: 4, 11: 5, 12: 8 },
			rows: 3,
			caption: 'Each digit is bigger than the one before it.'
		}
	},
	{
		slug: 'arrow',
		glyph: 'arrow',
		name: 'Arrow',
		summary: 'The digit in the circle is the sum of the digits along its arrow.',
		tips: [
			'Long arrows force a big circle and small digits on the shaft.',
			'Digits may repeat on the arrow, unless the cells share a row, column or box.'
		],
		example: {
			constraints: [{ type: 'arrow', cells: [10, 11, 12] }],
			digits: { 10: 9, 11: 4, 12: 5 },
			rows: 3,
			caption: 'The circle, 9, is 4 + 5.'
		}
	},
	{
		slug: 'german-whispers',
		glyph: 'whisper',
		name: 'German whispers',
		summary: 'Neighbouring digits on the line differ by at least 5.',
		tips: [
			'5 can never be on the line: no digit is 5 or more away from it.',
			'Digits alternate between low (1–4) and high (6–9).',
			'4 needs a 9 on both sides, 6 a 1.'
		],
		example: {
			constraints: [{ type: 'line', kind: 'germanWhisper', cells: [9, 10, 11, 12] }],
			digits: { 9: 1, 10: 7, 11: 2, 12: 8 },
			rows: 3,
			caption: 'Each step along the line jumps by 5 or more.'
		}
	},
	{
		slug: 'dutch-whispers',
		glyph: 'whisper',
		name: 'Dutch whispers',
		summary: 'Neighbouring digits on the line differ by at least 4.',
		tips: [
			'5 can only sit next to a 1 or a 9.',
			'Lines still tend to alternate between low and high digits.'
		],
		example: {
			constraints: [{ type: 'line', kind: 'dutchWhisper', cells: [9, 10, 11, 12] }],
			digits: { 9: 1, 10: 5, 11: 9, 12: 4 },
			rows: 3,
			caption: 'Each step along the line jumps by 4 or more.'
		}
	},
	{
		slug: 'kropki',
		glyph: 'kropki',
		name: 'Kropki',
		summary: 'A hollow dot joins consecutive digits, a filled dot a digit and its double.',
		tips: ['Filled dots only fit 1–2, 2–4, 3–6 and 4–8.', '1 and 2 can sit on either kind of dot.'],
		example: {
			constraints: [
				{ type: 'marker', kind: 'white', cells: [10, 11] },
				{ type: 'marker', kind: 'black', cells: [13, 14] }
			],
			digits: { 10: 4, 11: 5, 13: 3, 14: 6 },
			rows: 3,
			caption: '4 and 5 are consecutive; 6 is double 3.'
		}
	},
	{
		slug: 'xv',
		glyph: 'xv',
		name: 'XV',
		summary: 'Digits joined by an X add up to 10.',
		tips: ['An X only fits 1–9, 2–8, 3–7 and 4–6.', "5 can't touch an X."],
		example: {
			constraints: [{ type: 'marker', kind: 'x', cells: [10, 11] }],
			digits: { 10: 3, 11: 7 },
			rows: 3,
			caption: '3 + 7 = 10.'
		}
	},
	{
		slug: 'greater-than',
		glyph: 'chevron',
		name: 'Greater than',
		summary: 'The arrowhead points at the smaller digit.',
		tips: [
			'Chains of arrows put several cells in order.',
			"A cell with arrows pointing away on all sides is a big digit; it can't be 1."
		],
		example: {
			constraints: [{ type: 'marker', kind: 'chevron', cells: [10, 11] }],
			digits: { 10: 7, 11: 2 },
			rows: 3,
			caption: '7 is greater than 2, so the arrow points at the 2.'
		}
	}
];

export type Technique = {
	slug: string;
	name: string;
	/** the solver's rating, 1 (easiest) to 7 */
	difficulty: number;
	summary: string;
	steps: string[];
};

// names and ratings match the solver's strategies
export const techniques: Technique[] = [
	{
		slug: 'naked-single',
		name: 'Naked single',
		difficulty: 1,
		summary: 'A cell where only one digit is still possible.',
		steps: [
			'Look at a cell and cross off every digit its row, column and box already have.',
			'If one digit is left, it goes there.'
		]
	},
	{
		slug: 'hidden-single',
		name: 'Hidden single',
		difficulty: 2,
		summary: 'A digit with only one possible cell in a row, column or box.',
		steps: [
			'Pick a digit and a box, row or column that still needs it.',
			'If every cell but one is blocked, the digit goes in that one, whatever else the cell could hold.'
		]
	},
	{
		slug: 'pointing-pair',
		name: 'Pointing pair',
		difficulty: 3,
		summary: "A digit's spots in a box all lie on one line, so it leaves the rest of that line.",
		steps: [
			"Find a box where a digit's possible cells all share a row or column.",
			'The digit must be in that box, so remove it from the rest of that row or column.'
		]
	},
	{
		slug: 'box-line-reduction',
		name: 'Box/line reduction',
		difficulty: 3,
		summary:
			"A digit's spots in a row or column all lie in one box, so it leaves the rest of that box.",
		steps: [
			"Find a row or column where a digit's possible cells all sit in one box.",
			'The digit must be on that line, so remove it from the rest of the box.'
		]
	},
	{
		slug: 'naked-pair',
		name: 'Naked pair',
		difficulty: 4,
		summary: 'Two cells in a house that can only hold the same two digits.',
		steps: [
			'Find two cells in a row, column or box with the same two candidates.',
			'Those digits must go in those cells, so remove them from the rest of the house.'
		]
	},
	{
		slug: 'hidden-pair',
		name: 'Hidden pair',
		difficulty: 4,
		summary: 'Two digits that can only go in the same two cells of a house.',
		steps: [
			'Find two digits that each have only the same two spots in a house.',
			'Those cells must hold those digits, so remove every other candidate from them.'
		]
	},
	{
		slug: 'naked-triple',
		name: 'Naked triple',
		difficulty: 5,
		summary: 'Three cells in a house that between them can only hold three digits.',
		steps: [
			'Find three cells in a house whose candidates together are just three digits.',
			'Remove those digits from the rest of the house.'
		]
	},
	{
		slug: 'hidden-triple',
		name: 'Hidden triple',
		difficulty: 5,
		summary: 'Three digits that can only go in the same three cells of a house.',
		steps: [
			'Find three digits whose spots in a house are the same three cells.',
			'Remove every other candidate from those cells.'
		]
	},
	{
		slug: 'x-wing',
		name: 'X-wing',
		difficulty: 6,
		summary: 'A digit locked into the same two columns in two rows.',
		steps: [
			'Find two rows where a digit can only go in the same two columns.',
			'Either way it fills both columns, so remove it from the rest of those columns.',
			'The same works with rows and columns swapped.'
		]
	},
	{
		slug: 'y-wing',
		name: 'Y-wing',
		difficulty: 6,
		summary: 'Three two-candidate cells that together rule out a digit.',
		steps: [
			'Find a pivot cell with candidates XY that sees a cell with XZ and a cell with YZ.',
			'Whatever the pivot is, one of the wings is Z.',
			'So any cell that sees both wings cannot be Z.'
		]
	},
	{
		slug: 'swordfish',
		name: 'Swordfish',
		difficulty: 7,
		summary: 'An X-wing over three rows and three columns.',
		steps: [
			'Find three rows where a digit can only go in the same three columns.',
			'Remove the digit from the rest of those columns.'
		]
	}
];

/** The rule with a `ruleLabel` from $lib/variants, e.g. "German whispers". */
export function findRule(label: string): Rule | undefined {
	return rules.find(({ name }) => name === label);
}

/** The technique behind a solver strategy, e.g. "hidden single"; variant rules have none. */
export function findTechnique(strategy: string): Technique | undefined {
	return techniques.find(({ name }) => name.toLowerCase() === strategy.toLowerCase());
}

/** Easy to Expert, from the solver's 1–7 rating. */
export function techniqueLevel(difficulty: number): string {
	if (difficulty <= 2) return 'Easy';
	if (difficulty <= 3) return 'Medium';
	if (difficulty <= 5) return 'Hard';
	return 'Expert';
}

/** A board with just the given digits, for examples. */
export function exampleCells(digits: Record<number, number>): Sudoku {
	return Array.from({ length: 9 }, (_, row) =>
		Array.from({ length: 9 }, (_, col) => {
			const value = digits[row * 9 + col] ?? 0;
			return { value, notes: [], colors: [], fixed: value > 0 };
		})
	);
}

/** Highlights cells of an example. */
export function highlightLayer(cells: number[] = []): BoardLayer {
	return { cell: (index) => (cells.includes(index) ? 'bg-accent/25' : undefined) };
}
