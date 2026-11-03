<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import ConfirmSheet from '$lib/components/ConfirmSheet.svelte';
	import Segmented from '$lib/components/Segmented.svelte';
	import Sheet from '$lib/components/Sheet.svelte';
	import { difficulties as labels } from '$lib/puzzles';
	import { sudokuStore } from '$lib/stores';

	let { open = $bindable(false) }: { open?: boolean } = $props();

	// the generator numbers them from 1, easiest first
	const difficulties = Object.values(labels).map((label, i) => ({ value: i + 1, label }));
	let difficulty = $state(1);

	let confirmOpen = $state(false);

	function start() {
		open = false;
		if (sudokuStore.hasUnsavedGame) confirmOpen = true;
		else newGame();
	}

	function newGame() {
		sudokuStore.generate(difficulty);
		goto(resolve('/game'));
	}
</script>

<Sheet bind:open title="Classic sudoku">
	<div class="flex flex-col gap-6">
		<section>
			<h3 class="mb-2 text-xs font-semibold uppercase tracking-wider opacity-60">Difficulty</h3>
			<Segmented label="Difficulty" options={difficulties} bind:value={difficulty} />
		</section>

		<button
			class="rounded-full bg-accent py-3 font-semibold text-background transition hover:bg-accent-1"
			onclick={start}
		>
			Start
		</button>

		<a
			href="{resolve('/puzzles')}?section=classic"
			class="-mt-2 text-center text-sm font-medium text-accent transition hover:opacity-80"
			onclick={() => (open = false)}
		>
			Or pick a ready-made puzzle
		</a>
	</div>
</Sheet>

<ConfirmSheet
	bind:open={confirmOpen}
	title="Discard current game?"
	message="Starting a new puzzle throws away the game you're playing now."
	confirmLabel="Discard"
	onconfirm={newGame}
/>
