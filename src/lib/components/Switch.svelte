<svelte:options runes={true} />

<script lang="ts">
	let { checked = $bindable(false), label = '' }: { checked?: boolean; label?: string } = $props();
</script>

<div class="flex items-center">
	<label class="pr-4 leading-none text-text" for="airplane-mode" id="airplane-mode-label">
		{label}
	</label>

	<button
		onclick={() => {
			checked = !checked;
		}}
		data-state={checked ? 'checked' : 'unchecked'}
		class="relative h-6 cursor-pointer rounded-full bg-gray-500 transition-colors data-[state=checked]:bg-accent"
		id="airplane-mode"
		aria-labelledby="airplane-mode-label"
	>
		<span class="thumb block rounded-full bg-white transition" />
	</button>
	<input bind:checked type="checkbox" class="sr-only" />
</div>

<style>
	button {
		--w: 2.75rem;
		--padding: 0.125rem;
		width: var(--w);
	}

	.thumb {
		--size: 1.25rem;
		width: var(--size);
		height: var(--size);
		transform: translate(var(--padding));
	}

	[data-state='checked'] .thumb {
		transform: translate(calc(var(--w) - var(--size) - var(--padding)));
	}
</style>
