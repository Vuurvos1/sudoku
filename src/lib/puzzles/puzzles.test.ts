import { describe, expect, it } from 'vitest';
import { difficulties, sections } from '.';

const [classic, variants] = sections;
const all = sections.flatMap((section) => section.puzzles);

describe('ready-made puzzles', () => {
	it('have unique ids', () => {
		const ids = all.map((puzzle) => puzzle.id);
		expect(new Set(ids).size).toBe(ids.length);
	});

	it('have 81 cells and a difficulty', () => {
		for (const puzzle of all) {
			expect(puzzle.givens, puzzle.id).toMatch(/^[1-9.]{81}$/);
			expect(Object.keys(difficulties), puzzle.id).toContain(puzzle.difficulty);
		}
	});

	it('ship a solution that fits their givens', () => {
		for (const { id, givens, solution } of all) {
			if (!solution) continue;
			expect(solution, id).toMatch(/^[1-9]{81}$/);
			for (let i = 0; i < 81; i++) if (givens[i] !== '.') expect(solution[i], id).toBe(givens[i]);
		}
	});

	it('are in the section matching their rules', () => {
		for (const puzzle of classic.puzzles) expect(puzzle.constraints, puzzle.id).toEqual([]);
		for (const puzzle of variants.puzzles) expect(puzzle.constraints, puzzle.id).not.toEqual([]);
	});
});
