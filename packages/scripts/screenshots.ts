// Takes the README screenshots: Home, a puzzle in progress and the puzzles overview.
//
// The app runs in a plain browser against the Vite dev server, with Tauri's IPC mocked:
// the store plugin is kept in memory and seeded with a game in progress, and the solver
// commands are answered by a small backtracking solver below (classic rules + diagonals).
//
//   pnpm --filter @one-sudoku/scripts exec playwright install chromium   # once
//   pnpm screenshots

import { mkdir } from 'node:fs/promises';
import { join, resolve } from 'node:path';
import { chromium, type Page } from 'playwright';
import { createServer } from 'vite';
import classic from '../../src/lib/puzzles/classic.json' with { type: 'json' };

// the app lives at the repo root; this runs from its own package
const ROOT = resolve(import.meta.dirname, '../..');
const OUT_DIR = join(ROOT, 'docs/screenshots');
// Tailwind and PostCSS look for their config in the working directory
process.chdir(ROOT);
const VIEWPORT = { width: 390, height: 844 };

type Constraint = { type: string };
type Cell = { value: number; notes: number[]; colors: string[]; fixed: boolean };

/** Rows, columns, boxes, and the diagonals when the puzzle has them, as lists of cell indices. */
function houses(constraints: Constraint[]): number[][] {
	const range = [...Array(9).keys()];
	const result = [
		...range.map((r) => range.map((c) => r * 9 + c)),
		...range.map((c) => range.map((r) => r * 9 + c)),
		...range.map((b) =>
			range.map((i) => (Math.floor(b / 3) * 3 + Math.floor(i / 3)) * 9 + (b % 3) * 3 + (i % 3))
		)
	];
	if (constraints.some((c) => c.type === 'diagonals')) {
		result.push(range.map((i) => i * 10));
		result.push(range.map((i) => i * 8 + 8));
	}
	return result;
}

function peers(constraints: Constraint[]): number[][] {
	const sets = Array.from({ length: 81 }, () => new Set<number>());
	for (const house of houses(constraints)) {
		for (const a of house) for (const b of house) if (a !== b) sets[a].add(b);
	}
	return sets.map((set) => [...set].sort((a, b) => a - b));
}

function candidates(grid: number[], peerMap: number[][], cell: number): number[] {
	const seen = new Set(peerMap[cell].map((p) => grid[p]));
	return [1, 2, 3, 4, 5, 6, 7, 8, 9].filter((d) => !seen.has(d));
}

/** Backtracking with the most constrained cell first; fast enough for these puzzles. */
function solve(givens: string, constraints: Constraint[]): number[] {
	const grid = [...givens].map((ch) => Number(ch) || 0);
	const peerMap = peers(constraints);

	const search = (): boolean => {
		let best = -1;
		let bestOptions: number[] = [];
		for (let i = 0; i < 81; i++) {
			if (grid[i]) continue;
			const options = candidates(grid, peerMap, i);
			if (best === -1 || options.length < bestOptions.length) {
				best = i;
				bestOptions = options;
				if (options.length <= 1) break;
			}
		}
		if (best === -1) return true;
		for (const digit of bestOptions) {
			grid[best] = digit;
			if (search()) return true;
		}
		grid[best] = 0;
		return false;
	};

	if (!search()) throw new Error(`no solution for ${givens}`);
	return grid;
}

type Puzzle = { id: string; givens: string; constraints: Constraint[] };

/**
 * A board partway through: some of the empty cells filled in from the solution, and
 * pencil marks with the real candidates in a few of the others.
 */
function inProgress(puzzle: Puzzle, solution: number[]): Cell[][] {
	const peerMap = peers(puzzle.constraints);
	const grid = [...puzzle.givens].map((ch) => Number(ch) || 0);
	const empty = grid.flatMap((v, i) => (v ? [] : [i]));
	empty.filter((_, n) => n % 3 === 0).forEach((i) => (grid[i] = solution[i]));
	const noted = new Set(empty.filter((i) => !grid[i] && i < 27));

	return Array.from({ length: 9 }, (_, row) =>
		Array.from({ length: 9 }, (_, col) => {
			const i = row * 9 + col;
			return {
				value: grid[i],
				notes: noted.has(i) ? candidates(grid, peerMap, i) : [],
				colors: [],
				fixed: puzzle.givens[i] !== '.'
			};
		})
	);
}

function solvedBoard(puzzle: Puzzle, solution: number[]): Cell[][] {
	return Array.from({ length: 9 }, (_, row) =>
		Array.from({ length: 9 }, (_, col) => {
			const i = row * 9 + col;
			return { value: solution[i], notes: [], colors: [], fixed: puzzle.givens[i] !== '.' };
		})
	);
}

/** What the app's stores read at launch: a classic game in progress and a solved one. */
function seed() {
	const current = classic.puzzles[2] as Puzzle;
	const done = classic.puzzles[0] as Puzzle;
	const currentSolution = solve(current.givens, current.constraints);
	const doneSolution = solve(done.givens, done.constraints);
	const elapsed = (7 * 60 + 23) * 1000;
	const sudoku = inProgress(current, currentSolution);

	return {
		'oneSudoku.bin': {
			puzzle: { id: current.id, givens: current.givens, constraints: current.constraints },
			sudoku,
			// shifted to the browser's clock in the init script
			startTime: -elapsed,
			endTime: null,
			[`progress:${current.id}`]: { sudoku, elapsed, completed: false, solved: false },
			[`progress:${done.id}`]: {
				sudoku: solvedBoard(done, doneSolution),
				elapsed: (4 * 60 + 51) * 1000,
				completed: true,
				solved: true
			}
		},
		'oneSudokuSettings.bin': {}
	};
}

/**
 * Runs in the page before the app: stands in for `window.__TAURI_INTERNALS__`.
 * `solver` holds the functions above, passed as source since they run in the browser.
 */
function mockTauri({
	stores,
	solver
}: {
	stores: Record<string, Record<string, unknown>>;
	solver: string;
}) {
	const { solve, peers } = new Function(`${solver}; return { solve, peers };`)();

	const byRid: Map<string, unknown>[] = [];
	const ridByPath: Record<string, number> = {};
	for (const [path, entries] of Object.entries(stores)) {
		if (entries.startTime !== undefined && entries.startTime !== null) {
			entries.startTime = Date.now() + (entries.startTime as number);
		}
		ridByPath[path] = byRid.push(new Map(Object.entries(entries))) - 1;
	}

	let callbackId = 0;
	/** what the mocked commands take, each only some of it */
	type Args = {
		path: string;
		rid: number;
		key: string;
		value: unknown;
		puzzle: Puzzle;
		constraints: Constraint[];
	};
	const commands: Record<string, (args: Args) => unknown> = {
		'plugin:store|load': ({ path }) => (ridByPath[path] ??= byRid.push(new Map()) - 1),
		'plugin:store|get_store': ({ path }) => ridByPath[path] ?? null,
		'plugin:store|get': ({ rid, key }) => [byRid[rid].get(key) ?? null, byRid[rid].has(key)],
		'plugin:store|set': ({ rid, key, value }) => void byRid[rid].set(key, value),
		'plugin:store|has': ({ rid, key }) => byRid[rid].has(key),
		'plugin:store|delete': ({ rid, key }) => byRid[rid].delete(key),
		'plugin:store|entries': ({ rid }) => [...byRid[rid].entries()],
		'plugin:store|keys': ({ rid }) => [...byRid[rid].keys()],
		'plugin:store|values': ({ rid }) => [...byRid[rid].values()],
		'plugin:store|length': ({ rid }) => byRid[rid].size,
		'plugin:store|save': () => {},
		'plugin:event|listen': () => callbackId++,
		'plugin:event|unlisten': () => {},
		solve_sudoku: ({ puzzle }) => solve(puzzle.givens, puzzle.constraints),
		get_peers: ({ constraints }) => peers(constraints),
		find_conflicts: () => [],
		hint_sudoku: () => {
			throw 'no hints in screenshots';
		}
	};

	(window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {
		metadata: { currentWindow: { label: 'main' }, currentWebview: { label: 'main' } },
		transformCallback: () => callbackId++,
		invoke: async (cmd: string, args: Partial<Args> = {}) => {
			const command = commands[cmd];
			if (!command) throw new Error(`screenshots: unmocked command ${cmd}`);
			return command(args as Args);
		}
	};
}

async function open(page: Page, url: string) {
	await page.goto(url);
	await page.locator('#splash').waitFor({ state: 'detached' });
	await page.evaluate(() => document.fonts.ready);
}

async function main() {
	const server = await createServer({
		root: ROOT,
		configFile: join(ROOT, 'vite.config.js'),
		server: { port: 5199, strictPort: false },
		logLevel: 'warn'
	});
	await server.listen();
	const base = server.resolvedUrls!.local[0].replace(/\/$/, '');

	const browser = await chromium.launch().catch(async (err) => {
		await server.close();
		throw err;
	});
	try {
		const context = await browser.newContext({
			viewport: VIEWPORT,
			deviceScaleFactor: 2,
			reducedMotion: 'reduce',
			isMobile: true,
			hasTouch: true
		});
		const solver = [houses, peers, candidates, solve].map(String).join('\n');
		await context.addInitScript(mockTauri, { stores: seed(), solver });
		const page = await context.newPage();
		page.on('pageerror', (err) => console.error('page error:', err.message));

		await mkdir(OUT_DIR, { recursive: true });

		await open(page, `${base}/`);
		await page.screenshot({ path: `${OUT_DIR}/home.png` });

		await open(page, `${base}/game`);
		await page.screenshot({ path: `${OUT_DIR}/puzzle.png` });

		await open(page, `${base}/puzzles`);
		await page.screenshot({ path: `${OUT_DIR}/puzzles.png` });

		console.log(`screenshots saved to ${OUT_DIR}/`);
	} finally {
		await browser.close();
		await server.close();
	}
}

await main();
