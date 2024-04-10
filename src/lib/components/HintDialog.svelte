<script lang="ts">
	import Dialog from '$lib/components/Dialog.svelte';

	let { open = $bindable(false) }: { open?: boolean } = $props();
</script>

<Dialog bind:open>
	<h2 class="mb-3 text-2xl font-semibold">Hint</h2>
</Dialog>
