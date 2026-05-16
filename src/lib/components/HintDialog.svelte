<script lang="ts">
	import Dialog from '$lib/components/Dialog.svelte';
	import type { HintResponse } from '$lib/stores/sudoku.svelte';

	let { open = $bindable(false), hint = null }: { open?: boolean; hint?: HintResponse | null } =
		$props();
</script>

<Dialog bind:open>
	<h2 class="mb-3 text-2xl font-semibold">Hint</h2>

	{#if hint}
		<p class="mb-1 text-sm font-medium text-accent">{hint.strategy}</p>
		<p class="mb-2 font-semibold">{hint.summary}</p>
		<p class="text-sm opacity-80">{hint.detail}</p>
	{:else}
		<p class="text-sm opacity-60">No hint available for the current board.</p>
	{/if}
</Dialog>
