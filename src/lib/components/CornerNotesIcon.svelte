<script lang="ts">
	let { size = 24, class: className }: { size?: number; class?: string } = $props();
</script>

<!-- a cell with its notes in their own spots, as corner notes look on the board -->
<svg
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="1.5"
	class={className}
	aria-hidden="true"
>
	<rect x="3" y="3" width="18" height="18" rx="2" />
	<g
		font-size="5.5"
		font-weight="600"
		text-anchor="middle"
		dominant-baseline="central"
		fill="currentColor"
		stroke="none"
	>
		<text x="7.5" y="7.5">1</text>
		<text x="16.5" y="7.5">3</text>
		<text x="7.5" y="16.5">7</text>
		<text x="16.5" y="16.5">9</text>
	</g>
</svg>
