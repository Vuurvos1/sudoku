<svelte:options namespace="svg" />

<script lang="ts">
	import type { ConstraintSpec } from '$lib/stores';
	import { cellCenter, type Point } from './geometry';

	let { constraint }: { constraint: Extract<ConstraintSpec, { type: 'arrow' }> } = $props();

	const CIRCLE_RADIUS = 0.4;
	// length of each side of the arrowhead, and its angle to the shaft
	const HEAD_LENGTH = 0.2;
	const HEAD_ANGLE = Math.PI / 4;

	const unit = (from: Point, to: Point): Point => {
		const dx = to.x - from.x;
		const dy = to.y - from.y;
		const length = Math.hypot(dx, dy);
		return { x: dx / length, y: dy / length };
	};
	const rotate = ({ x, y }: Point, angle: number): Point => ({
		x: x * Math.cos(angle) - y * Math.sin(angle),
		y: x * Math.sin(angle) + y * Math.cos(angle)
	});

	const centers = $derived(constraint.cells.map(cellCenter));
	const circle = $derived(centers[0]);

	// the shaft leaves the circle from its edge, towards the first shaft cell
	const shaft = $derived.by(() => {
		const out = unit(centers[0], centers[1]);
		const start = { x: circle.x + out.x * CIRCLE_RADIUS, y: circle.y + out.y * CIRCLE_RADIUS };
		return [start, ...centers.slice(1)].map(({ x, y }) => `${x},${y}`).join(' ');
	});

	// two strokes back from the tip, either side of the last segment
	const head = $derived.by(() => {
		const tip = centers[centers.length - 1];
		const back = unit(tip, centers[centers.length - 2]);
		const side = (angle: number) => {
			const d = rotate(back, angle);
			return `${tip.x + d.x * HEAD_LENGTH},${tip.y + d.y * HEAD_LENGTH}`;
		};
		return `${side(HEAD_ANGLE)} ${tip.x},${tip.y} ${side(-HEAD_ANGLE)}`;
	});
</script>

<g class="arrow" fill="none" stroke-width="0.03" stroke-linecap="round" stroke-linejoin="round">
	<circle cx={circle.x} cy={circle.y} r={CIRCLE_RADIUS} />
	<polyline points={shaft} />
	<polyline points={head} />
</g>

<style>
	.arrow {
		stroke: hsl(var(--color-text-hsl) / 0.55);
	}
</style>
