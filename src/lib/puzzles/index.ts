import type { Puzzle } from '#lib/stores/index.ts';
import classic from './classic.json';
import variants from './variants.json';

export const difficulties = {
	easy: 'Easy',
	medium: 'Medium',
	hard: 'Hard',
	expert: 'Expert'
} as const;

export type Difficulty = keyof typeof difficulties;

export type ReadyPuzzle = Puzzle & {
	/** stable across releases, e.g. "classic-3" */
	id: string;
	title: string;
	difficulty: Difficulty;
	/** who set the puzzle; generated puzzles have none */
	author?: string;
};

export type PuzzleSection = {
	id: string;
	name: string;
	description: string;
	puzzles: ReadyPuzzle[];
};

/** The ready-made puzzles: classic ones first, then variants. */
export const sections = [classic, variants] as PuzzleSection[];
