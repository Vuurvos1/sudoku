import { describe, expect, it } from 'vitest';
import { findRule, findTechnique, rules } from './learn';
import { ruleLabels } from './variants';

describe('findTechnique', () => {
	it('matches the solver strategy names', () => {
		expect(findTechnique('hidden single')?.slug).toBe('hidden-single');
		expect(findTechnique('box/line reduction')?.slug).toBe('box-line-reduction');
		expect(findTechnique('x-wing')?.slug).toBe('x-wing');
	});

	it('has nothing for strategies without a page', () => {
		expect(findTechnique('fish')).toBeUndefined();
	});
});

describe('findRule', () => {
	it('finds a rule by its label', () => {
		expect(findRule("Knight's move")?.slug).toBe('knights-move');
		expect(findRule('Renban')?.slug).toBe('renban');
		expect(findRule('Nonsense')).toBeUndefined();
	});

	it("finds each rule from its example's labels, as the game shows them", () => {
		for (const rule of rules) {
			const found = ruleLabels(rule.example.constraints).map(findRule);
			expect(found, rule.name).toContain(rule);
		}
	});

	it('has a unique name for every rule', () => {
		expect(new Set(rules.map(({ name }) => name)).size).toBe(rules.length);
	});
});
