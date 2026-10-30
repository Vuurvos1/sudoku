<script lang="ts" module>
	export type GameKind = 'classic' | 'variants';
</script>

<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Segmented from '$lib/components/Segmented.svelte';
	import Sheet from '$lib/components/Sheet.svelte';
	import { sudokuStore, type ConstraintSpec } from '$lib/stores';
	import { ruleLabel } from '$lib/variants';

	let { open = $bindable(false), kind }: { open?: boolean; kind: GameKind } = $props();

	const difficulties = [
		{ value: 1, label: 'Easy' },
		{ value: 2, label: 'Medium' },
		{ value: 3, label: 'Hard' },
		{ value: 4, label: 'Expert' }
	];
	let difficulty = $state(1);

	// rules that apply to the whole grid, the only ones the generator can add for now
	type GlobalRule = Extract<ConstraintSpec['type'], 'diagonals' | 'antiKnight' | 'antiKing'>;
	const rules: GlobalRule[] = ['diagonals', 'antiKnight', 'antiKing'];
	let selectedRules = $state<GlobalRule[]>(['diagonals']);

	function toggleRule(rule: GlobalRule) {
		selectedRules = selectedRules.includes(rule)
			? selectedRules.filter((r) => r !== rule)
			: [...selectedRules, rule];
	}

	const canStart = $derived(kind === 'classic' || selectedRules.length > 0);

	function start() {
		const constraints = kind === 'variants' ? selectedRules.map((type) => ({ type })) : [];
		sudokuStore.generate(difficulty, constraints);
		open = false;
		goto(resolve('/game'));
	}
</script>

<Sheet bind:open title={kind === 'classic' ? 'Classic sudoku' : 'Variant sudoku'}>
	<div class="flex flex-col gap-6">
		<section>
			<h3 class="mb-2 text-xs font-semibold uppercase tracking-wider opacity-60">Difficulty</h3>
			<Segmented label="Difficulty" options={difficulties} bind:value={difficulty} />
		</section>

		{#if kind === 'variants'}
			<section>
				<h3 class="mb-2 text-xs font-semibold uppercase tracking-wider opacity-60">Rules</h3>
				<div class="flex flex-wrap gap-2">
					{#each rules as rule (rule)}
						{@const active = selectedRules.includes(rule)}
						<button
							aria-pressed={active}
							onclick={() => toggleRule(rule)}
							class={[
								'rounded-full px-4 py-2 text-sm font-medium ring-1 ring-inset transition',
								active ? 'bg-accent/15 text-accent ring-accent' : 'ring-text/25 hover:bg-text/5'
							]}
						>
							{ruleLabel({ type: rule })}
						</button>
					{/each}
				</div>
			</section>
		{/if}

		<button
			class="rounded-full bg-accent py-3 font-semibold text-background transition hover:bg-accent-1 disabled:opacity-40"
			disabled={!canStart}
			onclick={start}
		>
			Start
		</button>
	</div>
</Sheet>
