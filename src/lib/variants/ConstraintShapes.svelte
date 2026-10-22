<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '$lib/stores';
	import { rendererFor, type Layer } from '.';

	let { constraints, layer }: { constraints: ConstraintSpec[]; layer: Layer } = $props();
</script>

{#each constraints as constraint, i (i)}
	{@const renderer = rendererFor(constraint)}
	{#if renderer?.layer === layer}
		<renderer.component {constraint} />
	{/if}
{/each}
