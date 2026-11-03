import { describe, it, expect } from 'vitest';
import { cellColor, cellGradient, CELL_COLORS } from './color';

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

describe('cellColor', () => {
	it('uses the palette slot, so the palette can be swapped', () => {
		expect(cellColor('#4ade80')).toBe('var(--cell-color-4)');
		expect(cellColor('#4ade80', 0.4)).toBe(
			'color-mix(in srgb, var(--cell-color-4) 40%, transparent)'
		);
	});

	it('keeps colors outside the palette as they are', () => {
		expect(cellColor('#123456')).toBe('#123456');
	});
});

describe('cellGradient', () => {
	it('returns empty string for no colors', () => {
		expect(cellGradient([])).toBe('');
	});

	it('returns solid background for one color', () => {
		const result = cellGradient(['#f87171']);
		expect(result).toBe('color-mix(in srgb, var(--cell-color-1) 40%, transparent)');
	});

	it('returns conic-gradient for two colors', () => {
		const result = cellGradient(['#f87171', '#60a5fa']);
		expect(result).toContain('conic-gradient');
		expect(result).toContain('var(--cell-color-1) 40%');
		expect(result).toContain('var(--cell-color-6) 40%');
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
