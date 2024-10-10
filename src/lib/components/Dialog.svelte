<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fade } from 'svelte/transition';

	interface Props {
		open?: boolean;
		children?: Snippet;
	}

	let { open = $bindable(true), children }: Props = $props();
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
			{@render children?.()}

			<button onclick={() => (open = false)}>close</button>
		</div>
	{/if}
</div>
