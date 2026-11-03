<script lang="ts">
	interface Props {
		checked?: boolean;
		label: string;
		/** a line under the label explaining the setting */
		hint?: string;
	}

	let { checked = $bindable(false), label, hint }: Props = $props();
</script>

<!-- a settings row: label on the left, the switch lined up on the right -->
<div class="flex items-center gap-4 py-3">
	<div class="flex-1">
		<p>{label}</p>
		{#if hint}
			<p class="mt-0.5 text-sm opacity-60">{hint}</p>
		{/if}
	</div>

	<button
		role="switch"
		aria-checked={checked}
		aria-label={label}
		onclick={() => (checked = !checked)}
		data-state={checked ? 'checked' : 'unchecked'}
		class="relative h-6 shrink-0 cursor-pointer rounded-full bg-gray-500 transition-colors data-[state=checked]:bg-accent"
	>
		<span class="thumb block rounded-full bg-white transition"></span>
	</button>
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
