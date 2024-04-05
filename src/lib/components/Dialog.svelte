<script lang="ts">
	import { fade } from 'svelte/transition';

	let { open = $bindable(true) }: { open?: boolean } = $props();
</script>

<div>
	{#if open}
		<div transition:fade={{ duration: 200 }} class="fixed inset-0 z-30 bg-gray-900/70"></div>
	{/if}

	{#if open}
		<div
			transition:fade={{ duration: 150 }}
			class="fixed inset-4 z-40 rounded-lg border border-accent bg-background p-4"
		>
			<slot />

			<button onclick={() => (open = false)}>close</button>
		</div>
	{/if}
</div>
