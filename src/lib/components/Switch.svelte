<script lang="ts">
	interface Props {
		checked?: boolean;
		label?: string;
		id?: string;
	}

	let { checked = $bindable(false), label = '', id }: Props = $props();
</script>

<div class="switch">
	<label class="flex items-center gap-4 pr-4 text-text">
		{#if label}
			<span class="leading-none">
				{label}
			</span>
		{/if}

		<button
			onclick={() => {
				checked = !checked;
			}}
			data-state={checked ? 'checked' : 'unchecked'}
			class="relative h-6 cursor-pointer rounded-full bg-gray-500 transition-colors data-[state=checked]:bg-accent"
		>
			<span class="thumb block rounded-full bg-white transition"></span>
		</button>
		<input bind:checked type="checkbox" class="sr-only" {id} />
	</label>
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
