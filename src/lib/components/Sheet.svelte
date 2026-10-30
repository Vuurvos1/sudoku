<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { XIcon } from '@lucide/svelte';

	let {
		open = $bindable(false),
		title,
		children
	}: { open?: boolean; title: string; children?: Snippet } = $props();

	const id = $props.id();
</script>

<svelte:window
	onkeydown={(ev) => {
		if (open && ev.key === 'Escape') open = false;
	}}
/>

{#if open}
	<!-- a bottom sheet on phones, a centered card on larger screens -->
	<div class="fixed inset-0 z-40 flex items-end justify-center sm:items-center sm:p-4">
		<button
			class="absolute inset-0 cursor-default bg-black/60"
			aria-label="Close"
			tabindex="-1"
			onclick={() => (open = false)}
			transition:fade={{ duration: 200 }}
		></button>

		<div
			role="dialog"
			aria-modal="true"
			aria-labelledby="{id}-title"
			class="relative flex max-h-[90dvh] w-full max-w-md flex-col rounded-t-3xl bg-background ring-1 ring-text/10 sm:rounded-3xl"
			transition:fly={{ y: 60, duration: 200 }}
		>
			<div class="flex items-center justify-between pb-2 pl-6 pr-4 pt-4">
				<h2 id="{id}-title" class="text-xl font-bold">{title}</h2>
				<button
					class="grid h-11 w-11 place-items-center rounded-full transition hover:bg-text/10"
					aria-label="Close"
					onclick={() => (open = false)}
				>
					<XIcon size={20} />
				</button>
			</div>

			<div
				class="overflow-y-auto px-6"
				style="padding-bottom: calc(1.5rem + env(safe-area-inset-bottom))"
			>
				{@render children?.()}
			</div>
		</div>
	</div>
{/if}
