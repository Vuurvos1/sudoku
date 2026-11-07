<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '#lib/stores/index.ts';
	import { borderMidpoint, cellCenter } from './geometry';

	let { constraint }: { constraint: Extract<ConstraintSpec, { type: 'marker' }> } = $props();

	// half the width and height of the X, drawn as two strokes like the chevron
	const X_HALF = 0.075;

	// half the chevron's depth along the border's normal, and half its height along the border
	const CHEVRON_DEPTH = 0.05;
	const CHEVRON_SPREAD = 0.095;

	const a = $derived(cellCenter(constraint.cells[0]));
	const b = $derived(cellCenter(constraint.cells[1]));
	const mid = $derived(borderMidpoint(constraint.cells));
	// unit vector from the first cell to the second (they are orthogonal neighbours)
	const dir = $derived({ x: b.x - a.x, y: b.y - a.y });

	// ">" with its tip towards the smaller (second) cell
	const chevron = $derived.by(() => {
		const tip = { x: mid.x + dir.x * CHEVRON_DEPTH, y: mid.y + dir.y * CHEVRON_DEPTH };
		const back = { x: mid.x - dir.x * CHEVRON_DEPTH, y: mid.y - dir.y * CHEVRON_DEPTH };
		const perp = { x: -dir.y * CHEVRON_SPREAD, y: dir.x * CHEVRON_SPREAD };
		return `${back.x + perp.x},${back.y + perp.y} ${tip.x},${tip.y} ${back.x - perp.x},${back.y - perp.y}`;
	});
</script>

{#if constraint.kind === 'white' || constraint.kind === 'black'}
	<circle cx={mid.x} cy={mid.y} r="0.12" class={constraint.kind} stroke-width="0.03" />
{:else if constraint.kind === 'x'}
	<path
		d="M{mid.x - X_HALF},{mid.y - X_HALF}L{mid.x + X_HALF},{mid.y + X_HALF}M{mid.x +
			X_HALF},{mid.y - X_HALF}L{mid.x - X_HALF},{mid.y + X_HALF}"
		class="x"
		fill="none"
		stroke-width="0.04"
		stroke-linecap="round"
	/>
{:else}
	<polyline
		points={chevron}
		class="chevron"
		fill="none"
		stroke-width="0.04"
		stroke-linecap="round"
		stroke-linejoin="round"
	/>
{/if}

<style>
	.white {
		fill: hsl(var(--color-background-hsl));
		stroke: hsl(var(--color-text-hsl));
	}

	.black {
		fill: hsl(var(--color-text-hsl));
		stroke: hsl(var(--color-text-hsl));
	}

	.x,
	.chevron {
		stroke: hsl(var(--color-text-hsl));
	}
</style>
