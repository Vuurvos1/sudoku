<script lang="ts">
	import Dialog from '$lib/components/Dialog.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { sudokuStore } from '$lib/stores';

	let { open = $bindable(false) }: { open?: boolean } = $props();

	function formatTime(ms: number): string {
		const totalSeconds = Math.floor(ms / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}m ${seconds}s`;
	}

	const elapsed = $derived(
		sudokuStore.startTime && sudokuStore.endTime
			? sudokuStore.endTime - sudokuStore.startTime
			: null
	);

	function handleNewGame() {
		open = false;
		sudokuStore.clear();
		goto(resolve('/'));
	}
</script>

<Dialog bind:open>
	<div class="flex flex-col items-center gap-6 py-4">
		<h2 class="text-2xl font-semibold">Puzzle Complete</h2>

		{#if elapsed !== null}
			<p class="text-lg">{formatTime(elapsed)}</p>
		{/if}

		<button
			onclick={handleNewGame}
			class="rounded-full border-2 border-accent px-6 py-2 font-medium transition hover:bg-accent hover:text-background"
		>
			New Game
		</button>
	</div>
</Dialog>
