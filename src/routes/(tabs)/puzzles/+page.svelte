<script lang="ts">
	import { ChevronRightIcon } from '@lucide/svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { sudokuStore } from '$lib/stores';
</script>

<h1 class="mb-6 text-3xl font-bold">Puzzles</h1>

<p class="opacity-70">
	Hand-made packs, a weekly featured puzzle and puzzles imported from SudokuPad or f-puzzles will
	live here.
</p>

{#if import.meta.env.DEV}
	<h2 class="mb-1 mt-8 text-xs font-semibold uppercase tracking-wider opacity-60">Development</h2>
	<button
		class="flex w-full items-center gap-3 border-b border-text/10 py-3 text-left"
		onclick={() => {
			sudokuStore.showcase();
			goto(resolve('/game'));
		}}
	>
		<span class="flex-1">
			<span class="block">Test puzzle</span>
			<span class="block text-sm opacity-60">Uses every variant rule</span>
		</span>
		<ChevronRightIcon size={18} class="opacity-50" />
	</button>
{/if}
