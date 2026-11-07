<script lang="ts">
	import RuleIcon from '#lib/components/RuleIcon.svelte';
	import Sudoku from '#lib/components/Sudoku.svelte';
	import { exampleCells, highlightLayer, type Rule } from '#lib/learn.ts';

	let { rule }: { rule: Rule } = $props();
</script>

<div class="flex items-center gap-4">
	<span class="grid h-12 w-12 shrink-0 place-items-center rounded-xl bg-text/10 text-accent">
		<RuleIcon glyph={rule.glyph} />
	</span>
	<h1 class="text-3xl font-bold">{rule.name}</h1>
</div>

<p class="mt-4 text-lg">{rule.summary}</p>

<figure class="mt-6">
	<!-- the top rows of a board, drawn with the game's own renderer -->
	<div
		class="relative w-full overflow-hidden rounded-lg"
		style:aspect-ratio="9 / {rule.example.rows}"
	>
		<div class="absolute inset-x-0 top-0 aspect-square">
			<Sudoku
				cells={exampleCells(rule.example.digits)}
				constraints={rule.example.constraints}
				layers={[highlightLayer(rule.example.highlight)]}
			/>
		</div>
	</div>
	<figcaption class="mt-2 text-sm opacity-70">{rule.example.caption}</figcaption>
</figure>

<h2 class="mb-2 mt-8 text-xs font-semibold uppercase tracking-wider opacity-60">Tips</h2>
<ul class="list-disc space-y-2 pl-5 opacity-80">
	{#each rule.tips as tip (tip)}
		<li>{tip}</li>
	{/each}
</ul>
