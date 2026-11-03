<script lang="ts">
	import { ChevronLeftIcon, ChevronRightIcon } from '@lucide/svelte';
	import RuleDetail from '$lib/components/RuleDetail.svelte';
	import RuleIcon from '$lib/components/RuleIcon.svelte';
	import Sheet from '$lib/components/Sheet.svelte';
	import { findRule, type Rule } from '$lib/learn';
	import type { ConstraintSpec } from '$lib/stores';
	import { ruleLabels } from '$lib/variants';

	let { open = $bindable(false), constraints }: { open?: boolean; constraints: ConstraintSpec[] } =
		$props();

	const puzzleRules = $derived(
		ruleLabels(constraints)
			.map(findRule)
			.filter((rule) => rule !== undefined)
	);

	// the rule being read; with a single rule there is no list to go back to
	let chosen = $state<Rule | null>(null);
	const shown = $derived(puzzleRules.length === 1 ? puzzleRules[0] : chosen);

	$effect(() => {
		if (!open) chosen = null;
	});
</script>

<!-- the rules of the puzzle being played, explained without leaving the game -->
<Sheet bind:open title="Rules">
	{#if shown}
		{#if puzzleRules.length > 1}
			<button
				class="-ml-1 mb-4 inline-flex items-center gap-1 text-sm opacity-70 transition hover:opacity-100"
				onclick={() => (chosen = null)}
			>
				<ChevronLeftIcon size={18} />
				All rules
			</button>
		{/if}
		<RuleDetail rule={shown} />
	{:else}
		<p class="mb-2 opacity-70">On top of the usual sudoku rules:</p>
		<ul>
			{#each puzzleRules as rule (rule.slug)}
				<li class="border-b border-text/10 last:border-0">
					<button
						class="flex w-full items-center gap-4 py-3 text-left transition hover:opacity-80"
						onclick={() => (chosen = rule)}
					>
						<span
							class="grid h-11 w-11 shrink-0 place-items-center rounded-xl bg-text/10 text-accent"
						>
							<RuleIcon glyph={rule.glyph} />
						</span>
						<span class="min-w-0 flex-1">
							<span class="block font-medium">{rule.name}</span>
							<span class="block text-sm opacity-70">{rule.summary}</span>
						</span>
						<ChevronRightIcon size={18} class="shrink-0 opacity-50" />
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</Sheet>
