<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '$lib/stores';
	import { cageOutline } from './geometry';

	let { constraint }: { constraint: Extract<ConstraintSpec, { type: 'cage' }> } = $props();

	const id = $props.id();

	const INSET = 0.055;
	const FONT_SIZE = 0.22;
	// label position, measured from the cell corner to the top-left of the digits;
	// it straddles the outline so the cutout replaces the cage's corner
	const LABEL_X = 0.04;
	const LABEL_Y = 0.035;
	// space between the label and where the outline resumes
	const CUTOUT_PADDING = 0.025;
	// height of a digit above its baseline, as a fraction of the font size; used
	// instead of dominant-baseline, which WebKitGTK (the Linux webview) ignores
	const CAP_HEIGHT = 0.72;

	// the sum sits in the cage's top-left cell (lowest index)
	const first = $derived(Math.min(...constraint.cells));
	const x = $derived(first % 9);
	const y = $derived(Math.floor(first / 9));
	const outline = $derived(cageOutline(constraint.cells, INSET));

	let label = $state<SVGTextElement>();
	let measuredWidth = $state<number | null>(null);
	// estimate until the rendered text can be measured
	const labelWidth = $derived(measuredWidth ?? 0.1 * String(constraint.sum ?? '').length);
	$effect(() => {
		void constraint.sum;
		if (label) measuredWidth = label.getBBox().width;
	});
</script>

{#if constraint.sum !== null}
	<mask id="cage-{id}" maskUnits="userSpaceOnUse" x="0" y="0" width="9" height="9">
		<rect width="9" height="9" fill="white" />
		<rect
			{x}
			{y}
			width={LABEL_X + labelWidth + CUTOUT_PADDING}
			height={LABEL_Y + FONT_SIZE * CAP_HEIGHT + CUTOUT_PADDING}
			fill="black"
		/>
	</mask>
{/if}
<path
	d={outline}
	mask={constraint.sum !== null ? `url(#cage-${id})` : undefined}
	class="cage"
	fill="none"
	stroke-width="0.018"
	stroke-dasharray="0.1 0.07"
/>
{#if constraint.sum !== null}
	<text
		bind:this={label}
		x={x + LABEL_X}
		y={y + LABEL_Y + FONT_SIZE * CAP_HEIGHT}
		class="cage-sum"
		font-size={FONT_SIZE}
	>
		{constraint.sum}
	</text>
{/if}

<style>
	.cage {
		stroke: hsl(var(--color-text-hsl) / 0.7);
	}

	.cage-sum {
		fill: hsl(var(--color-text-hsl));
	}
</style>
