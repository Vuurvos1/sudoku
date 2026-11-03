import type { BoardLayer } from '$lib/board';
import type { HintResponse } from '$lib/stores';

// key → role, keeping the first role seen per key unless a later one is "focus".
// Map keeps the last entry per key, so the entries are reversed and focus entries appended.
function rolesByKey<K>(entries: [K, string][]): Map<K, string> {
	const focus = entries.filter(([, role]) => role === 'focus');
	return new Map([...entries].reverse().concat(focus));
}

/** Shows a hint on a board: highlights its cells and candidates and dims everything else. */
export function hintLayer(hint: HintResponse): BoardLayer {
	// cell index → highlight role
	const cellRoles = rolesByKey(
		hint.highlights.flatMap((group) =>
			group.cells.map(({ cell, role }): [number, string] => [cell, role])
		)
	);
	// "cell:digit" → candidate role
	const candidateRoles = rolesByKey(
		hint.highlights.flatMap((group) =>
			group.candidates.map(({ cell, digit, role }): [string, string] => [`${cell}:${digit}`, role])
		)
	);
	// cell index → digit to place
	const placements = new Map(hint.placements.map(({ cell, digit }) => [cell, digit]));
	const eliminations = new Set(hint.eliminations.map(({ cell, digit }) => `${cell}:${digit}`));

	// every cell the hint refers to
	const relevant = new Set([
		...cellRoles.keys(),
		...[...candidateRoles.keys()].map((key) => parseInt(key.split(':')[0])),
		...placements.keys(),
		...hint.eliminations.map(({ cell }) => cell)
	]);

	return {
		cell: (index) => {
			const role = cellRoles.get(index);
			return { 'bg-accent/40': role === 'focus', 'bg-accent/20': role === 'supporting' };
		},
		content: (index) => ({ 'opacity-20': !relevant.has(index) }),
		value: (index) => ({ 'font-bold text-accent': placements.has(index) }),
		note: (index, digit) => {
			const key = `${index}:${digit}`;
			const role = candidateRoles.get(key);
			if (role === 'focus') return 'font-bold text-accent';
			if (role === 'eliminated' || eliminations.has(key)) return 'text-red-400 line-through';
		},
		placement: (index) => placements.get(index)
	};
}
