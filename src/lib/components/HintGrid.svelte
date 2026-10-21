<script lang="ts">
	import type { Sudoku, HintResponse } from '$lib/stores/sudoku.svelte';

	let { sudoku, hint }: { sudoku: Sudoku; hint: HintResponse } = $props();

	// key → role, keeping the first role seen per key unless a later one is "focus".
	// Map keeps the last entry per key, so the entries are reversed and focus entries appended.
	function rolesByKey<K>(entries: [K, string][]): Map<K, string> {
		const focus = entries.filter(([, role]) => role === 'focus');
		return new Map([...entries].reverse().concat(focus));
	}

	// cell index → highlight role
	const cellRoleMap = $derived(
		rolesByKey(
			hint.highlights.flatMap((group) =>
				group.cells.map(({ cell, role }): [number, string] => [cell, role])
			)
		)
	);

	// "cell:digit" → candidate role
	const candidateRoleMap = $derived(
		rolesByKey(
			hint.highlights.flatMap((group) =>
				group.candidates.map(({ cell, digit, role }): [string, string] => [
					`${cell}:${digit}`,
					role
				])
			)
		)
	);

	// cell index → digit to place
	const placementMap = $derived(new Map(hint.placements.map(({ cell, digit }) => [cell, digit])));

	// "cell:digit" keys for eliminations
	const eliminationSet = $derived(
		new Set(hint.eliminations.map(({ cell, digit }) => `${cell}:${digit}`))
	);

	// union of all cells referenced by any map above
	const relevantCells = $derived(
		new Set([
			...cellRoleMap.keys(),
			...[...candidateRoleMap.keys()].map((key) => parseInt(key.split(':')[0])),
			...placementMap.keys(),
			...[...eliminationSet].map((key) => parseInt(key.split(':')[0]))
		])
	);
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
						<div
							class={[
								'flex h-full w-full items-center justify-center',
								!isRelevant && 'opacity-20'
							]}
						>
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
