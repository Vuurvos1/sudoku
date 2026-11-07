<script lang="ts" generics="T extends string | number">
	import type { Component } from 'svelte';

	interface Props {
		/** an icon is shown before the label */
		options: { value: T; label: string; icon?: Component<{ size?: number }> }[];
		value: T;
		/** names the group for screen readers */
		label: string;
	}

	let { options, value = $bindable(), label }: Props = $props();
</script>

<div
	role="radiogroup"
	aria-label={label}
	class="grid auto-cols-fr grid-flow-col rounded-full bg-text/10 p-1"
>
	{#each options as option (option.value)}
		{@const active = value === option.value}
		<button
			role="radio"
			aria-checked={active}
			onclick={() => (value = option.value)}
			class={[
				'flex items-center justify-center gap-1.5 rounded-full px-3 py-1.5 text-sm font-medium transition',
				active ? 'bg-accent text-background shadow' : 'opacity-70 hover:opacity-100'
			]}
		>
			{#if option.icon}
				<option.icon size={18} />
			{/if}
			{option.label}
		</button>
	{/each}
</div>
