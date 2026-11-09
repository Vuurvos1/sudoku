<script lang="ts">
	import { CircleCheckIcon } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import FilterChips from '#lib/components/FilterChips.svelte';
	import Sudoku from '#lib/components/Sudoku.svelte';
	import { givenCells, sudokuStore, withGivens } from '#lib/stores/index.ts';
	import { difficulties, sections, type ReadyPuzzle } from '#lib/puzzles/index.ts';
	import RuleIcon from '#lib/components/RuleIcon.svelte';
	import { puzzleRules, ruleIcons } from '#lib/variants/index.ts';

	// every game keeps its own progress, so nothing is lost by switching
	function play(puzzle: ReadyPuzzle) {
		sudokuStore.play(puzzle);
		goto(resolve('game'));
	}

	// e.g. "Medium · by Someone"; the rules are shown as icons
	function details({ difficulty, author }: ReadyPuzzle): string {
		return [difficulties[difficulty], author && `by ${author}`].filter(Boolean).join(' · ');
	}

	const rules = (puzzle: ReadyPuzzle) => puzzleRules(puzzle.constraints);

	// `?section=variants` shows a single section, e.g. from the home screen
	const only = $derived(sections.find((s) => s.id === page.url.searchParams.get('section')));
	const visible = $derived(only ? [only] : sections);

	// the rules and difficulties of the puzzles on show, classic first and easiest first
	const ruleOptions = $derived(
		[...new Set(visible.flatMap((section) => section.puzzles.flatMap(rules)))]
			.sort((a, b) => Number(b === 'Classic') - Number(a === 'Classic') || a.localeCompare(b))
			.map((rule) => ({ value: rule, label: rule, glyph: glyphs.get(rule) }))
	);
	// each rule's icon, by name; classic has none
	const glyphs = $derived(
		new Map(
			ruleIcons(visible.flatMap((s) => s.puzzles.flatMap((p) => p.constraints))).map(
				({ label, glyph }) => [label, glyph]
			)
		)
	);

	const difficultyOptions = $derived(
		Object.entries(difficulties)
			.filter(([value]) => visible.some((s) => s.puzzles.some((p) => p.difficulty === value)))
			.map(([value, label]) => ({ value, label }))
	);

	// what's picked, nothing means every puzzle; switching views starts over,
	// since a rule picked on one may not be on another
	let pickedRules = $derived.by((): string[] => {
		void only;
		return [];
	});
	let pickedDifficulties = $derived.by((): string[] => {
		void only;
		return [];
	});

	// a puzzle shows when it matches a picked rule and a picked difficulty
	const matches = (puzzle: ReadyPuzzle) =>
		(pickedRules.length === 0 || rules(puzzle).some((rule) => pickedRules.includes(rule))) &&
		(pickedDifficulties.length === 0 || pickedDifficulties.includes(puzzle.difficulty));

	// numbered by their place in the section, so a filter doesn't renumber them
	const shown = $derived(
		visible
			.map((section) => ({
				...section,
				entries: section.puzzles
					.map((puzzle, i) => ({ puzzle, n: i + 1 }))
					.filter(({ puzzle }) => matches(puzzle))
			}))
			.filter((section) => section.entries.length > 0)
	);
</script>

<h1 class="text-3xl font-bold">{only?.name ?? 'Puzzles'}</h1>
{#if only}
	<p class="mt-1 opacity-70">{only.description}</p>
{/if}

<div class="mb-6 mt-4 flex flex-col gap-2">
	<!-- a classic-only view has no rules to choose between -->
	{#if ruleOptions.length > 1}
		<FilterChips label="Filter by rule" options={ruleOptions} bind:selected={pickedRules} />
	{/if}
	<FilterChips
		label="Filter by difficulty"
		options={difficultyOptions}
		bind:selected={pickedDifficulties}
	/>
</div>

{#if shown.length === 0}
	<p class="text-sm opacity-50">No puzzles match these filters.</p>
{/if}

{#each shown as section (section.id)}
	<section class="mb-8">
		{#if !only}
			<h2 class="text-xl font-semibold">{section.name}</h2>
			<p class="text-sm opacity-70">{section.description}</p>
		{/if}

		<ul class="mt-4 grid grid-cols-2 gap-x-4 gap-y-6">
			{#each section.entries as { puzzle, n } (puzzle.id)}
				{@const progress = sudokuStore.progress[puzzle.id]}
				<li>
					<button
						class="block w-full text-left transition hover:opacity-80"
						onclick={() => play(puzzle)}
					>
						<!-- the board as it was left, or as it starts, drawn with the game's own renderer -->
						<span class="block aspect-square rounded-lg bg-text/5 p-1.5" aria-hidden="true">
							<Sudoku
								cells={progress
									? withGivens(puzzle.givens, progress.sudoku)
									: givenCells(puzzle.givens)}
								constraints={puzzle.constraints}
								showNotes={false}
							/>
						</span>
						<span class="mt-2 flex items-center gap-1.5 font-medium">
							{#if progress?.solved}
								<CircleCheckIcon size={16} class="shrink-0 text-accent" aria-label="Solved" />
							{/if}
							<span class="truncate">{puzzle.title || `Puzzle ${n}`}</span>
						</span>
						{#if progress && !progress.solved}
							<span class="block text-sm font-medium text-accent">In progress</span>
						{/if}
						{#if puzzle.constraints.length > 0}
							<span class="mt-1 flex flex-wrap gap-1.5 opacity-70">
								{#each ruleIcons(puzzle.constraints) as { glyph, label } (label)}
									<span role="img" aria-label={label} title={label}
										><RuleIcon {glyph} size={16} /></span
									>
								{/each}
							</span>
						{/if}
						<span class="line-clamp-2 block text-sm opacity-60">{details(puzzle)}</span>
					</button>
				</li>
			{/each}
		</ul>
	</section>
{/each}
