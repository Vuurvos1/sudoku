<script lang="ts">
	interface Props {
		options: { value: string; label: string }[];
		/** the options picked; none means everything */
		selected: string[];
		/** names the group for screen readers */
		label: string;
	}

	let { options, selected = $bindable(), label }: Props = $props();

	function toggle(value: string) {
		selected = selected.includes(value)
			? selected.filter((s) => s !== value)
			: [...selected, value];
	}

	const chip = (active: boolean) => [
		'rounded-full px-3 py-1.5 text-sm font-medium transition',
		active ? 'bg-accent text-background shadow' : 'bg-text/10 opacity-70 hover:opacity-100'
	];
</script>

<div role="group" aria-label={label} class="flex flex-wrap gap-2">
	<button
		aria-pressed={selected.length === 0}
		onclick={() => (selected = [])}
		class={chip(selected.length === 0)}
	>
		All
	</button>
	{#each options as option (option.value)}
		{@const active = selected.includes(option.value)}
		<button aria-pressed={active} onclick={() => toggle(option.value)} class={chip(active)}>
			{option.label}
		</button>
	{/each}
</div>
