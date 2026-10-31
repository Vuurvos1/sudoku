<script lang="ts">
	import { ChevronRightIcon } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import Sheet from '$lib/components/Sheet.svelte';
	import Sudoku from '$lib/components/Sudoku.svelte';
	import { hintLayer } from '$lib/hint';
	import { findTechnique, techniqueLevel } from '$lib/learn';
	import type { HintResponse } from '$lib/stores/sudoku.svelte';
	import { sudokuStore } from '$lib/stores/sudoku.svelte';

	let { open = $bindable(false), hint = null }: { open?: boolean; hint?: HintResponse | null } =
		$props();

	const technique = $derived(hint ? findTechnique(hint.strategy) : undefined);
	// the solver's names are lower case, e.g. "naked pair"
	const strategyName = $derived(
		technique?.name ?? (hint ? hint.strategy[0].toUpperCase() + hint.strategy.slice(1) : '')
	);
</script>

<Sheet bind:open title="Hint">
	{#if hint}
		<div class="flex flex-wrap items-center gap-2">
			<span class="rounded-full bg-accent/15 px-3 py-1 text-sm font-semibold text-accent">
				{strategyName}
			</span>
			<span class="rounded-full bg-text/10 px-3 py-1 text-sm opacity-80">
				{techniqueLevel(hint.difficulty)}
			</span>
		</div>

		<p class="mt-4 text-lg font-semibold leading-snug">{hint.summary}</p>
		<p class="mt-1 text-sm opacity-70">{hint.detail}</p>

		{#if sudokuStore.sudoku}
			<div class="mt-5 rounded-2xl bg-text/5 p-3">
				<div class="mx-auto aspect-square w-full max-w-72">
					<Sudoku
						cells={sudokuStore.sudoku}
						constraints={sudokuStore.puzzle?.constraints}
						layers={[hintLayer(hint)]}
					/>
				</div>
			</div>
		{/if}

		<div class="mt-5 flex items-center gap-3">
			{#if technique}
				<a
					href={resolve('/(tabs)/learn/techniques/[slug]', { slug: technique.slug })}
					onclick={() => (open = false)}
					class="flex flex-1 items-center gap-1 text-sm font-medium text-accent"
				>
					How it works
					<ChevronRightIcon size={16} />
				</a>
			{/if}
			<button
				class="ml-auto rounded-full bg-accent px-6 py-2.5 font-semibold text-background transition hover:opacity-90"
				onclick={() => (open = false)}
			>
				Got it
			</button>
		</div>
	{:else}
		<p class="opacity-70">
			No hint found. The board may have a mistake, or need a technique the solver doesn't know yet.
		</p>
	{/if}
</Sheet>
