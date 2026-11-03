<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '$lib/stores';
	import { cellCenter } from './geometry';

	let { constraint }: { constraint: Extract<ConstraintSpec, { type: 'line' }> } = $props();

	const points = $derived(
		constraint.cells
			.map(cellCenter)
			.map(({ x, y }) => `${x},${y}`)
			.join(' ')
	);
	const bulb = $derived(cellCenter(constraint.cells[0]));
	const isThermo = $derived(constraint.kind === 'thermo');
</script>

<!-- shapes are opaque and the group is faded as one, so the bulb and line don't double up -->
<g class={constraint.kind}>
	{#if isThermo}
		<circle cx={bulb.x} cy={bulb.y} r="0.34" stroke="none" />
	{/if}
	<polyline
		{points}
		fill="none"
		stroke-width={isThermo ? 0.26 : 0.16}
		stroke-linecap="round"
		stroke-linejoin="round"
	/>
</g>

<style>
	.germanWhisper {
		stroke: var(--german-whisper);
		opacity: 0.65;
	}

	.dutchWhisper {
		stroke: var(--dutch-whisper);
		opacity: 0.65;
	}

	/* the two whispers also differ in shape, not just color */
	:global(:root[data-color-blind]) .dutchWhisper {
		stroke-dasharray: 0.2 0.35;
	}

	.thermo {
		stroke: hsl(var(--color-text-hsl));
		fill: hsl(var(--color-text-hsl));
		opacity: 0.2;
	}
</style>
