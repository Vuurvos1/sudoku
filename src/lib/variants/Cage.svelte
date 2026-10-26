<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '$lib/stores';
	import { regionOutline } from './geometry';

	let { constraint }: { constraint: Extract<ConstraintSpec, { type: 'cage' }> } = $props();

	const INSET = 0.055;
	const FONT_SIZE = 0.22;
	// label position, measured from the cell corner to the top-left of the digits;
	// it straddles the outline, which is cut away around it
	const LABEL_X = 0.04;
	const LABEL_Y = 0.035;
	// visible space between the digits' ink and where the outline resumes
	const CUTOUT_PADDING = 0.05;
	const STROKE_WIDTH = 0.018;
	// height of a digit above its baseline, as a fraction of the font size; used
	// instead of dominant-baseline, which WebKitGTK (the Linux webview) ignores
	const CAP_HEIGHT = 0.72;

	// the sum sits in the cage's top-left cell (lowest index)
	const first = $derived(Math.min(...constraint.cells));
	const x = $derived(first % 9);
	const y = $derived(Math.floor(first / 9));

	let label = $state<SVGTextElement>();
	// ink extent of the digits from the text origin, as a fraction of the font size;
	// getBBox only gives the advance box, which includes side bearings and ascent
	let ink = $state<{ right: number; descent: number } | null>(null);
	$effect(() => {
		const text = String(constraint.sum ?? '');
		const el = label;
		if (!el) return;
		const measure = () => {
			const style = getComputedStyle(el);
			const ctx = document.createElement('canvas').getContext('2d');
			if (!ctx) return;
			ctx.font = `${style.fontStyle} ${style.fontWeight} 100px ${style.fontFamily}`;
			const m = ctx.measureText(text);
			ink = { right: m.actualBoundingBoxRight / 100, descent: m.actualBoundingBoxDescent / 100 };
		};
		measure();
		// remeasure once a web font has replaced the fallback
		document.fonts.ready.then(measure);
	});
	// estimate until the rendered text can be measured
	const inkRight = $derived(ink?.right ?? 0.5 * String(constraint.sum ?? '').length);
	const inkDescent = $derived(ink?.descent ?? 0);
	const outline = $derived(
		regionOutline(
			constraint.cells,
			INSET,
			constraint.sum !== null
				? {
						// the round line caps reach half a stroke into the gap
						width: LABEL_X + FONT_SIZE * inkRight + CUTOUT_PADDING + STROKE_WIDTH / 2,
						height:
							LABEL_Y + FONT_SIZE * (CAP_HEIGHT + inkDescent) + CUTOUT_PADDING + STROKE_WIDTH / 2
					}
				: undefined
		)
	);
</script>

<path
	d={outline}
	class="cage"
	fill="none"
	stroke-width={STROKE_WIDTH}
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
