<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '#lib/stores/index.ts';
	import { gridCutouts } from '.';

	// a separate id per layer, since each layer is its own <svg>
	let { id, constraints }: { id: string; constraints: ConstraintSpec[] } = $props();

	const cutouts = $derived(gridCutouts(constraints));
</script>

<!-- hides whatever it masks inside a circle behind each marker on a cell border -->
<mask {id} maskUnits="userSpaceOnUse" x="0" y="0" width="9" height="9">
	<rect width="9" height="9" fill="white" />
	{#each cutouts as { x, y, r }, i (i)}
		<circle cx={x} cy={y} {r} fill="black" />
	{/each}
</mask>
