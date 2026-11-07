<script lang="ts">
	import { TrophyIcon } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Sheet from '#lib/components/Sheet.svelte';
	import { sudokuStore } from '#lib/stores/index.ts';
	import { puzzleRules } from '#lib/variants/index.ts';
	import { formatTime } from '#lib/time.ts';

	let { open = $bindable(false) }: { open?: boolean } = $props();

	const elapsed = $derived(
		sudokuStore.startTime && sudokuStore.endTime
			? sudokuStore.endTime - sudokuStore.startTime
			: null
	);
	const kind = $derived(puzzleRules(sudokuStore.puzzle?.constraints ?? []).join(' · '));

	function handleNewGame() {
		open = false;
		sudokuStore.clear();
		goto(resolve('/(tabs)'));
	}
</script>

<Sheet bind:open title="Solved">
	<div class="flex flex-col items-center text-center">
		<span class="grid h-16 w-16 place-items-center rounded-full bg-accent/15 text-accent">
			<TrophyIcon size={30} />
		</span>

		{#if elapsed !== null}
			<p class="mt-4 text-5xl font-bold tabular-nums">{formatTime(elapsed)}</p>
		{/if}
		<p class="mt-2 text-sm opacity-70">{kind}</p>
	</div>

	<div class="mt-8 grid grid-cols-2 gap-3">
		<button
			class="rounded-full py-2.5 font-semibold ring-1 ring-inset ring-text/20 transition hover:bg-text/5"
			onclick={() => (open = false)}
		>
			View board
		</button>
		<button
			class="rounded-full bg-accent py-2.5 font-semibold text-background transition hover:opacity-90"
			onclick={handleNewGame}
		>
			New game
		</button>
	</div>
</Sheet>
