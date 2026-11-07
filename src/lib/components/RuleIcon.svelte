<script lang="ts">
	import { ChessKingIcon, ChessKnightIcon } from '@lucide/svelte';
	import type { RuleGlyph } from '#lib/learn.ts';

	let { glyph, size = 24 }: { glyph: RuleGlyph; size?: number } = $props();
</script>

<!-- a small drawing of a rule, in the style of the board -->
{#if glyph === 'antiKnight'}
	<ChessKnightIcon size={size - 2} />
{:else if glyph === 'antiKing'}
	<ChessKingIcon size={size - 2} />
{:else}
	<svg
		width={size}
		height={size}
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		stroke-linecap="round"
		stroke-linejoin="round"
		aria-hidden="true"
	>
		{#if glyph === 'diagonals'}
			<rect x="3" y="3" width="18" height="18" rx="2" stroke-width="1.5" />
			<path d="M6 6l12 12M18 6 6 18" />
		{:else if glyph === 'cage'}
			<path d="M4 4h16v16H4z" stroke-dasharray="2.5 2.5" stroke-width="1.5" />
			<text x="6" y="11" font-size="7" fill="currentColor" stroke="none">15</text>
		{:else if glyph === 'thermo'}
			<circle cx="6" cy="18" r="3.5" fill="currentColor" />
			<path d="M8 16 18 6" stroke-width="3.5" />
		{:else if glyph === 'arrow'}
			<circle cx="6.5" cy="17.5" r="3.5" />
			<path d="M9 15 19 5M13 5h6v6" />
		{:else if glyph === 'whisper'}
			<path d="M3 16 9 8l6 8 6-8" stroke-width="3" />
		{:else if glyph === 'kropki'}
			<circle cx="7" cy="12" r="3.5" />
			<circle cx="17" cy="12" r="3.5" fill="currentColor" />
		{:else if glyph === 'xv'}
			<path d="M8 8l8 8M16 8l-8 8" stroke-width="2.5" />
		{:else if glyph === 'chevron'}
			<path d="m9 6 6 6-6 6" stroke-width="2.5" />
		{/if}
	</svg>
{/if}
