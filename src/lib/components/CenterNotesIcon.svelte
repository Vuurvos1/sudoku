<script lang="ts">
	let { size = 24, class: className }: { size?: number; class?: string } = $props();
</script>

<!-- a cell with its notes in a row across the middle, as centered notes look on the board -->
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
	<text
		x="12"
		y="12"
		font-size="6.5"
		font-weight="600"
		text-anchor="middle"
		dominant-baseline="central"
		fill="currentColor"
		stroke="none">123</text
	>
</svg>
