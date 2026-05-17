import { describe, it, expect } from 'vitest';
import { cellGradient, CELL_COLORS } from './color';

describe('CELL_COLORS', () => {
	it('has exactly 9 colors', () => {
		expect(CELL_COLORS).toHaveLength(9);
	});

	it('each color is a valid 6-digit hex string', () => {
		for (const color of CELL_COLORS) {
			expect(color).toMatch(/^#[0-9a-f]{6}$/i);
		}
	});
});

describe('cellGradient', () => {
	it('returns empty string for no colors', () => {
		expect(cellGradient([])).toBe('');
	});

	it('returns solid background for one color', () => {
		const result = cellGradient(['#f87171']);
		expect(result).toBe('#f8717166');
	});

	it('returns conic-gradient for two colors', () => {
		const result = cellGradient(['#f87171', '#60a5fa']);
		expect(result).toContain('conic-gradient');
		expect(result).toContain('#f8717166');
		expect(result).toContain('#60a5fa66');
		expect(result).toContain('180deg');
	});

	it('returns conic-gradient with equal slices for three colors', () => {
		const result = cellGradient(['#f87171', '#60a5fa', '#4ade80']);
		expect(result).toContain('conic-gradient');
		expect(result).toContain('120deg');
		expect(result).toContain('240deg');
	});

	it('handles all 9 colors', () => {
		const result = cellGradient(CELL_COLORS);
		expect(result).toContain('conic-gradient');
		expect(result).toContain('40deg');
	});
});
