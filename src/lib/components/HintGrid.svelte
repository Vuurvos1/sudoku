<script lang="ts">
	import type { Sudoku, HintResponse } from '$lib/stores/sudoku.svelte';

	let { sudoku, hint }: { sudoku: Sudoku; hint: HintResponse } = $props();

	// cell index → highlight role ("focus" takes priority over "supporting")
	const cellRoleMap = $derived.by(() => {
		const map = new Map<number, string>();
		for (const group of hint.highlights) {
			for (const { cell, role } of group.cells) {
				const existing = map.get(cell);
				if (!existing || role === 'focus') {
					map.set(cell, role);
				}
			}
		}
		return map;
	});

	// "cell:digit" → candidate role ("focus" takes priority over "eliminated")
	const candidateRoleMap = $derived.by(() => {
		const map = new Map<string, string>();
		for (const group of hint.highlights) {
			for (const { cell, digit, role } of group.candidates) {
				const key = `${cell}:${digit}`;
				const existing = map.get(key);
				if (!existing || role === 'focus') {
					map.set(key, role);
				}
			}
		}
		return map;
	});

	// cell index → digit to place
	const placementMap = $derived.by(() => {
		const map = new Map<number, number>();
		for (const { cell, digit } of hint.placements) {
			map.set(cell, digit);
		}
		return map;
	});

	// "cell:digit" keys for eliminations
	const eliminationSet = $derived.by(() => {
		const set = new Set<string>();
		for (const { cell, digit } of hint.eliminations) {
			set.add(`${cell}:${digit}`);
		}
		return set;
	});

	// union of all cells referenced by any map above
	const relevantCells = $derived.by(() => {
		const set = new Set<number>();
		for (const cell of cellRoleMap.keys()) {
			set.add(cell);
		}
		for (const key of candidateRoleMap.keys()) {
			set.add(parseInt(key.split(':')[0]));
		}
		for (const cell of placementMap.keys()) {
			set.add(cell);
		}
		for (const key of eliminationSet) {
			set.add(parseInt(key.split(':')[0]));
		}
		return set;
	});
</script>

<div class="mx-auto mt-4 aspect-square w-full max-w-72">
	<div class="relative h-full w-full">
		<svg
			class="pointer-events-none absolute inset-0 z-10 h-full w-full"
			viewBox="0 0 9 9"
			preserveAspectRatio="none"
			stroke-linecap="round"
			overflow="visible"
		>
			<!-- thin lines -->
			{#each [1, 2, 4, 5, 7, 8] as i (i)}
				<line x1={i} y1={0} x2={i} y2={9} class="grid-line-thin" stroke-width="0.02" />
				<line x1={0} y1={i} x2={9} y2={i} class="grid-line-thin" stroke-width="0.02" />
			{/each}
			<!-- thick lines -->
			{#each [3, 6] as i (i)}
				<line x1={i} y1={0} x2={i} y2={9} class="stroke-accent" stroke-width="0.06" />
				<line x1={0} y1={i} x2={9} y2={i} class="stroke-accent" stroke-width="0.06" />
			{/each}
		</svg>

		<div class="grid h-full w-full grid-cols-9 grid-rows-9">
			{#each sudoku as row, i (i)}
				{#each row as cell, j (j)}
					{@const index = i * 9 + j}
					{@const cellRole = cellRoleMap.get(index)}
					{@const placement = placementMap.get(index)}
					{@const isRelevant = relevantCells.has(index)}
					<div
						class={[
							'grid place-items-center',
							cellRole === 'focus' && 'bg-accent/40',
							cellRole === 'supporting' && 'bg-accent/20'
						]}
					>
						<div class={['flex h-full w-full items-center justify-center', !isRelevant && 'opacity-20']}>
							{#if cell.value > 0}
								<span class="text-sm font-medium">{cell.value}</span>
							{:else if placement !== undefined}
								<span class="text-sm font-bold text-accent">{placement}</span>
							{:else}
								<!-- notes -->
								<div class="grid h-full w-full grid-cols-3 grid-rows-3 place-items-center">
									{#each { length: 9 } as _, x (x)}
										{@const digit = x + 1}
										{@const candidateKey = `${index}:${digit}`}
										{@const candidateRole = candidateRoleMap.get(candidateKey)}
										{@const isEliminated = eliminationSet.has(candidateKey)}
										{#if cell.notes.includes(digit)}
											<div
												class={[
													'text-[0.45rem] leading-none',
													candidateRole === 'focus' && 'font-bold text-accent',
													candidateRole !== 'focus' &&
														(candidateRole === 'eliminated' || isEliminated) &&
														'text-red-400 line-through'
												]}
											>
												{digit}
											</div>
										{:else}
											<div></div>
										{/if}
									{/each}
								</div>
							{/if}
						</div>
					</div>
				{/each}
			{/each}
		</div>
	</div>
</div>

<style>
	.grid-line-thin {
		stroke: hsl(var(--color-accent-hsl) / 0.4);
	}
</style>
