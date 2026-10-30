import { describe, expect, it } from 'vitest';
import { formatTime } from './time';

describe('formatTime', () => {
	it('shows minutes and seconds, adding hours when needed', () => {
		expect(formatTime(0)).toBe('0:00');
		expect(formatTime(65_400)).toBe('1:05');
		expect(formatTime(3_725_000)).toBe('1:02:05');
	});
});
