<script lang="ts" module>
	import type { Component } from 'svelte';

	export type MenuItem = { label: string; icon: Component; onselect: () => void };
</script>

<script lang="ts">
	import { EllipsisVerticalIcon } from '@lucide/svelte';
	import { fade } from 'svelte/transition';

	let { label, items }: { label: string; items: MenuItem[] } = $props();

	let open = $state(false);
	let root = $state<HTMLElement>();
</script>

<svelte:window
	onkeydown={(ev) => {
		if (open && ev.key === 'Escape') open = false;
	}}
	onpointerdown={(ev) => {
		if (open && !root?.contains(ev.target as Node)) open = false;
	}}
/>

<!-- a small dropdown for actions that don't need a button of their own -->
<div class="relative" bind:this={root}>
	<button
		aria-label={label}
		aria-haspopup="menu"
		aria-expanded={open}
		onclick={() => (open = !open)}
		class="grid place-items-center"
	>
		<EllipsisVerticalIcon />
	</button>

	{#if open}
		<div
			role="menu"
			class="absolute right-0 top-full z-30 mt-2 min-w-48 rounded-2xl bg-background p-1.5 shadow-lg ring-1 ring-text/15"
			transition:fade={{ duration: 100 }}
		>
			{#each items as { label, icon: Icon, onselect } (label)}
				<button
					role="menuitem"
					class="flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-left transition hover:bg-text/10"
					onclick={() => {
						open = false;
						onselect();
					}}
				>
					<Icon size={18} class="opacity-70" />
					{label}
				</button>
			{/each}
		</div>
	{/if}
</div>
