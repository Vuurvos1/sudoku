<script lang="ts">
	import type { BoardLayer } from '#lib/board.ts';
	import { cellGradient } from '#lib/color.ts';
	import { noteStyleOf } from '#lib/game.svelte.ts';
	import {
		settingsStore,
		type ConstraintSpec,
		type NoteStyle,
		type Sudoku
	} from '#lib/stores/index.ts';
	import ConstraintShapes from '#lib/variants/ConstraintShapes.svelte';
	import CutoutMask from '#lib/variants/CutoutMask.svelte';
	import { cellPosition, regionOutline } from '#lib/variants/geometry.ts';

	let {
		cells,
		constraints = [],
		layers = [],
		selected,
		celebrate = false,
		noteStyle,
		showNotes = true,
		oncellpointerdown,
		onpointermove
	}: {
		cells: Sudoku;
		constraints?: ConstraintSpec[];
		/** decorations, applied in order */
		layers?: BoardLayer[];
		/** cells to outline */
		selected?: ReadonlySet<number>;
		/** play the solved animation */
		celebrate?: boolean;
		/** how notes are shown in cells that weren't switched; the setting by default */
		noteStyle?: NoteStyle;
		/** draw pencil marks; previews leave them out */
		showNotes?: boolean;
		/** makes the board interactive */
		oncellpointerdown?: (ev: PointerEvent, index: number) => void;
		onpointermove?: (ev: PointerEvent) => void;
	} = $props();

	const defaultNoteStyle = $derived(noteStyle ?? settingsStore.settings.noteStyle);

	// mask ids must be unique per document, and a hint can show a second board
	const id = $props.id();

	const indices = Array.from({ length: 81 }, (_, i) => i);
	const cell = (index: number) => {
		const { row, col } = cellPosition(index);
		return cells[row][col];
	};
	const delay = (index: number) => {
		const { row, col } = cellPosition(index);
		return celebrate ? `${(row + col) * 40}ms` : '';
	};

	const placement = (index: number) => {
		for (const layer of layers) {
			const digit = layer.placement?.(index);
			if (digit !== undefined) return digit;
		}
	};

	const selectionPath = $derived(selected?.size ? regionOutline([...selected], 0) : '');

	function handlePointerDown(ev: PointerEvent) {
		const attr = (ev.target as Element).closest('[data-cell]')?.getAttribute('data-cell');
		if (attr != null) oncellpointerdown?.(ev, parseInt(attr));
	}
</script>

<div class="relative h-full w-full select-none">
	<!-- layer 1: cell highlights and colors -->
	<div
		class={['pointer-events-none absolute inset-0 z-0 grid grid-cols-9 grid-rows-9', { celebrate }]}
	>
		{#each indices as i (i)}
			{@const colors = cell(i).colors}
			<div
				class={['cell-bg relative', layers.map((l) => l.cell?.(i))]}
				style:animation-delay={delay(i)}
			>
				{#if colors.length > 0}
					<div class="absolute inset-0" style:background={cellGradient(colors)}></div>
				{/if}
			</div>
		{/each}
	</div>
	<!-- layer 2: variant shapes below the digits -->
	<svg
		class="pointer-events-none absolute inset-0 z-[1] h-full w-full"
		viewBox="0 0 9 9"
		preserveAspectRatio="none"
		stroke-linecap="round"
		overflow="visible"
	>
		<CutoutMask id="under-cutouts-{id}" {constraints} />
		<g mask="url(#under-cutouts-{id})">
			<ConstraintShapes {constraints} layer="under" />
		</g>
	</svg>
	<!-- layer 3: digits and notes, sized relative to the board -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class={[
			'relative z-[2] grid h-full w-full grid-cols-9 grid-rows-9 [container-type:inline-size]',
			{ 'touch-none': oncellpointerdown, celebrate }
		]}
		onpointerdown={oncellpointerdown && handlePointerDown}
		{onpointermove}
	>
		{#each indices as i (i)}
			{@const { value, notes } = cell(i)}
			{@const digit = value || placement(i)}
			<div data-cell={i} class="cell relative" style:animation-delay={delay(i)}>
				<div class={['grid h-full w-full place-items-center', layers.map((l) => l.content?.(i))]}>
					{#if digit}
						<!-- should also be in svg? -->
						<span
							class={['text-[5.5cqw] font-medium leading-none', layers.map((l) => l.value?.(i))]}
						>
							{digit}
						</span>
					{:else if !showNotes}
						<!-- no pencil marks -->
					{:else if noteStyleOf(cell(i), defaultNoteStyle) === 'center'}
						<!-- in a row, shrinking as it fills up so all nine still fit -->
						<div
							class={[
								'flex flex-wrap items-center justify-center px-[0.3cqw] leading-none',
								notes.length <= 4
									? 'text-[3cqw]'
									: notes.length <= 6
										? 'text-[2.3cqw]'
										: 'text-[1.8cqw]'
							]}
						>
							{#each [...notes].sort((a, b) => a - b) as note (note)}
								<span class={layers.map((l) => l.note?.(i, note))}>{note}</span>
							{/each}
						</div>
					{:else}
						<div
							class="grid h-full w-full grid-cols-3 grid-rows-3 place-items-center text-[3cqw] leading-none"
						>
							{#each { length: 9 } as _, x (x)}
								{#if notes.includes(x + 1)}
									<div class={layers.map((l) => l.note?.(i, x + 1))}>{x + 1}</div>
								{:else}
									<div></div>
								{/if}
							{/each}
						</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
	<!-- layer 4: grid lines, variant markers and selection -->
	<svg
		class="pointer-events-none absolute inset-0 z-[3] h-full w-full"
		viewBox="0 0 9 9"
		preserveAspectRatio="none"
		stroke-linecap="round"
		overflow="visible"
	>
		<!-- gaps in the grid lines and selection behind markers on cell borders -->
		<CutoutMask id="grid-cutouts-{id}" {constraints} />
		<g mask="url(#grid-cutouts-{id})">
			{#each [1, 2, 4, 5, 7, 8] as i (i)}
				<line x1={i} y1={0} x2={i} y2={9} class="grid-line-thin" stroke-width="0.02" />
				<line x1={0} y1={i} x2={9} y2={i} class="grid-line-thin" stroke-width="0.02" />
			{/each}
			{#each [3, 6] as i (i)}
				<line x1={i} y1={0} x2={i} y2={9} class="stroke-accent" stroke-width="0.06" />
				<line x1={0} y1={i} x2={9} y2={i} class="stroke-accent" stroke-width="0.06" />
			{/each}
			{#if selectionPath}
				<path d={selectionPath} fill="none" class="stroke-accent" stroke-width="0.06" />
			{/if}
		</g>
		<ConstraintShapes {constraints} layer="over" />
	</svg>
</div>

<style>
	.grid-line-thin {
		stroke: hsl(var(--color-accent-hsl) / 0.4);
	}

	@keyframes cell-celebrate {
		0% {
			background-color: transparent;
			transform: scale(1);
		}
		50% {
			background-color: hsl(var(--color-accent-hsl) / 0.4);
			transform: scale(1.05);
		}
		100% {
			background-color: transparent;
			transform: scale(1);
		}
	}

	@keyframes cell-pop {
		50% {
			transform: scale(1.05);
		}
	}

	.celebrate .cell-bg {
		animation: cell-celebrate 0.4s ease-out both;
	}

	.celebrate .cell {
		animation: cell-pop 0.4s ease-out both;
	}
</style>
