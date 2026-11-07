<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '#lib/stores/index.ts';

	let { constraint: _ }: { constraint: Extract<ConstraintSpec, { type: 'diagonals' }> } = $props();
</script>

<line x1="0" y1="0" x2="9" y2="9" class="diagonal" stroke-width="0.04" />
<line x1="9" y1="0" x2="0" y2="9" class="diagonal" stroke-width="0.04" />

<style>
	.diagonal {
		stroke: hsl(var(--color-accent-hsl) / 0.35);
	}
</style>
