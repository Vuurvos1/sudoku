<script lang="ts">
	import { settingsStore, dialogState, sudokuStore } from '$lib/stores';
	import { HistoryStore } from '$lib/stores/history.svelte';
	import {
		UndoIcon,
		RedoIcon,
		PencilIcon,
		PaintBucketIcon,
		EraserIcon,
		SettingsIcon,
		ChevronLeftIcon,
		LightbulbIcon
	} from 'lucide-svelte';
	import HintDialog from '$lib/components/HintDialog.svelte';
	import GameFinishedDialog from '$lib/components/GameFinishedDialog.svelte';
	import { CELL_COLORS, cellGradient } from '$lib/color';
	import { resolve } from '$app/paths';

	let mode = $state<'fill' | 'note' | 'color'>('fill');

	const history = new HistoryStore<ReturnType<typeof sudokuStore.copy>>();

	let selectedCells = $state(new Set<number>());
	const primaryCell = $derived(
		selectedCells.size === 1 ? selectedCells.values().next().value ?? null : null
	);
	let isDragging = $state(false);
	let dragMode = $state<'add' | 'remove'>('add');

	function cellIndex(row: number, col: number): number {
		return row * 9 + col;
	}

	function cellFromIndex(index: number): { row: number; col: number } {
		return { row: Math.floor(index / 9), col: index % 9 };
	}

	function selectionBorders(row: number, col: number): string {
		const index = cellIndex(row, col);
		if (!selectedCells.has(index)) return '';

		const top = row > 0 && selectedCells.has(cellIndex(row - 1, col)) ? '' : 'border-t-2';
		const bottom = row < 8 && selectedCells.has(cellIndex(row + 1, col)) ? '' : 'border-b-2';
		const left = col > 0 && selectedCells.has(cellIndex(row, col - 1)) ? '' : 'border-l-2';
		const right = col < 8 && selectedCells.has(cellIndex(row, col + 1)) ? '' : 'border-r-2';

		return [top, bottom, left, right].filter(Boolean).join(' ');
	}

	let selectedNumber = $state(0);

	let now = $state(Date.now());

	$effect(() => {
		if (sudokuStore.isSolved || !sudokuStore.startTime) return;

		const interval = setInterval(() => {
			now = Date.now();
		}, 1000);

		return () => clearInterval(interval);
	});

	function formatTime(ms: number): string {
		const totalSeconds = Math.floor(ms / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}m ${seconds}s`;
	}

	const elapsedDisplay = $derived(() => {
		if (!sudokuStore.startTime) return '';
		const end = sudokuStore.endTime ?? now;
		return formatTime(end - sudokuStore.startTime);
	});

	let boardSolved = $state(false);

	$effect(() => {
		if (sudokuStore.isSolved && !sudokuStore.endTime) {
			sudokuStore.endTime = Date.now();
			sudokuStore.save();

			selectedCells = new Set();
			boardSolved = true;

			setTimeout(() => {
				dialogState.gameFinishedOpen = true;
			}, 1000);
		}
	});

	function handlePointerDown(ev: PointerEvent, col: number, row: number) {
		const index = cellIndex(row, col);

		if (ev.ctrlKey || ev.metaKey) {
			if (selectedCells.has(index)) {
				selectedCells.delete(index);
			} else {
				selectedCells.add(index);
			}
			selectedCells = new Set(selectedCells);
			return;
		}

		isDragging = true;
		dragMode = 'add';

		if (selectedCells.size === 1 && selectedCells.has(index)) {
			// clicking the only selected cell — deselect
			selectedCells.clear();
			selectedCells = new Set(selectedCells);
			return;
		}

		selectedCells.clear();
		selectedCells.add(index);
		selectedCells = new Set(selectedCells);
	}

	function handlePointerMove(_ev: PointerEvent, col: number, row: number) {
		if (!isDragging) return;

		const index = cellIndex(row, col);

		if (dragMode === 'add') {
			if (!selectedCells.has(index)) {
				selectedCells.add(index);
				selectedCells = new Set(selectedCells);
			}
		} else {
			if (selectedCells.has(index)) {
				selectedCells.delete(index);
				selectedCells = new Set(selectedCells);
			}
		}
	}

	function handlePointerUp() {
		isDragging = false;
	}

	function handleCellKeydown(ev: KeyboardEvent) {
		const parsed = parseInt(ev.key);
		if (parsed > 0 && parsed <= 9) {
			setSudokuCell(parsed);
		} else if (ev.key === 'Backspace' || ev.key === 'Delete') {
			clearCell();
		}
	}

	function pushInitialState() {
		if (history.current === undefined) {
			history.pushState(sudokuStore.copy());
		}
	}

	function setSudokuCell(num: number) {
		if (!sudokuStore.sudoku) return;
		if (selectedCells.size === 0) return;

		pushInitialState();

		const isMulti = selectedCells.size > 1;
		const effectiveMode = isMulti && mode === 'fill' ? 'note' : mode;

		if (isMulti) {
			if (effectiveMode === 'note') {
				const allHaveNote = [...selectedCells].every((idx) => {
					const { row, col } = cellFromIndex(idx);
					return sudokuStore.sudoku![row][col].notes.includes(num);
				});

				for (const idx of selectedCells) {
					const { row, col } = cellFromIndex(idx);
					const cell = sudokuStore.sudoku[row][col];
					if (cell.fixed) continue;
					if (allHaveNote) {
						cell.notes = cell.notes.filter((n) => n !== num);
					} else if (!cell.notes.includes(num)) {
						cell.notes.push(num);
					}
				}
			} else if (effectiveMode === 'color') {
				const colorHex = CELL_COLORS[num - 1];
				const allHaveColor = [...selectedCells].every((idx) => {
					const { row, col } = cellFromIndex(idx);
					return sudokuStore.sudoku![row][col].colors.includes(colorHex);
				});

				for (const idx of selectedCells) {
					const { row, col } = cellFromIndex(idx);
					const cell = sudokuStore.sudoku[row][col];
					if (allHaveColor) {
						cell.colors = cell.colors.filter((c) => c !== colorHex);
					} else if (!cell.colors.includes(colorHex)) {
						cell.colors = [...cell.colors, colorHex];
					}
				}
			}
		} else {
			const idx = [...selectedCells][0];
			const { row, col } = cellFromIndex(idx);
			const cell = sudokuStore.sudoku[row][col];

			if (mode !== 'color' && cell.fixed) return;

			if (mode === 'fill') {
				cell.value = num;
				cell.notes = [];

				const boxStartRow = Math.floor(row / 3) * 3;
				const boxStartCol = Math.floor(col / 3) * 3;
				for (let i = 0; i < 9; i++) {
					sudokuStore.sudoku[row][i].notes = sudokuStore.sudoku[row][i].notes.filter(
						(n) => n !== num
					);
					sudokuStore.sudoku[i][col].notes = sudokuStore.sudoku[i][col].notes.filter(
						(n) => n !== num
					);
				}
				for (let r = boxStartRow; r < boxStartRow + 3; r++) {
					for (let c = boxStartCol; c < boxStartCol + 3; c++) {
						sudokuStore.sudoku[r][c].notes = sudokuStore.sudoku[r][c].notes.filter(
							(n) => n !== num
						);
					}
				}
			} else if (mode === 'note') {
				if (cell.notes.includes(num)) {
					cell.notes = cell.notes.filter((i) => i !== num);
				} else {
					cell.notes.push(num);
				}
			} else if (mode === 'color') {
				const colorHex = CELL_COLORS[num - 1];
				if (cell.colors.includes(colorHex)) {
					cell.colors = cell.colors.filter((c) => c !== colorHex);
				} else {
					cell.colors = [...cell.colors, colorHex];
				}
			}
		}

		history.pushState(sudokuStore.copy());
		sudokuStore.save();
	}

	function clearCell() {
		if (!sudokuStore.sudoku) return;
		if (selectedCells.size === 0) return;

		pushInitialState();

		for (const idx of selectedCells) {
			const { row, col } = cellFromIndex(idx);
			const cell = sudokuStore.sudoku[row][col];
			if (cell.fixed) continue;

			if (mode === 'color') {
				cell.colors = [];
			} else {
				cell.value = 0;
				cell.notes = [];
			}
		}

		history.pushState(sudokuStore.copy());
		sudokuStore.save();
	}

	function isPeer(row: number, col: number): boolean {
		if (selectedCells.size === 0) return false;
		const peerMap = sudokuStore.peerMap;
		if (!peerMap) return false;
		const target = cellIndex(row, col);
		if (selectedCells.has(target)) return false;
		for (const selected of selectedCells) {
			if (!peerMap[selected].includes(target)) return false;
		}
		return true;
	}

	function devAlmostSolve() {
		if (!sudokuStore.sudoku || !sudokuStore.solved) return;
		let skipped = false;
		for (let i = 0; i < 9; i++) {
			for (let j = 0; j < 9; j++) {
				if (sudokuStore.sudoku[i][j].fixed) continue;
				if (!skipped) {
					skipped = true;
					sudokuStore.sudoku[i][j].value = 0;
					continue;
				}
				sudokuStore.sudoku[i][j].value = sudokuStore.solved[i][j];
			}
		}
	}

	function remainingCount(digit: number): number {
		if (!sudokuStore.sudoku) return 9;
		let count = 0;
		for (const row of sudokuStore.sudoku) {
			for (const cell of row) {
				if (cell.value === digit) count++;
			}
		}
		return 9 - count;
	}

	function handleUndo() {
		const state = history.undo();
		if (state) sudokuStore.restore(state);
	}

	function handleRedo() {
		const state = history.redo();
		if (state) sudokuStore.restore(state);
	}
</script>

<svelte:window
	onpointerup={handlePointerUp}
	onkeydown={(ev) => {
		const key = ev.key.toLowerCase();
		if (key === 'z' && (ev.ctrlKey || ev.metaKey) && ev.shiftKey) {
			ev.preventDefault();
			handleRedo();
		} else if (key === 'z' && (ev.ctrlKey || ev.metaKey)) {
			ev.preventDefault();
			handleUndo();
		} else if (key === 'y' && (ev.ctrlKey || ev.metaKey)) {
			ev.preventDefault();
			handleRedo();
		} else if (import.meta.env.DEV && key === 'd' && (ev.ctrlKey || ev.metaKey) && ev.shiftKey) {
			ev.preventDefault();
			devAlmostSolve();
		}
	}}
/>

<div class="flex h-full max-h-full flex-col">
	<header class="py-3">
		<nav class="container flex flex-row items-center justify-between">
			<div class="flex w-32 flex-row items-center gap-4">
				<a href={resolve('/')} aria-label="back">
					<ChevronLeftIcon />
				</a>
			</div>

			{#if settingsStore.settings.showTimer}
				<h3 class="font-bold">{elapsedDisplay()}</h3>
			{/if}

			<div class="flex w-32 flex-row items-center justify-end gap-4">
				<button
					onclick={async () => {
						await sudokuStore.hint();
						dialogState.hintOpen = true;
					}}
					aria-label="hint"
				>
					<LightbulbIcon />
				</button>
				<button
					onclick={() => {
						dialogState.settingsOpen = true;
					}}
					aria-label="settings"
				>
					<SettingsIcon />
				</button>
			</div>
		</nav>
	</header>

	<main class="flex h-auto flex-1 flex-col overflow-hidden py-4">
		<div
			class="sudoku mx-auto h-full max-h-full w-full max-w-screen-lg flex-1 overflow-hidden px-4 py-12 md:px-8"
		>
			<div class="mx-auto flex aspect-square max-h-full">
				<div class="relative mx-auto h-full max-h-full w-full max-w-full">
					<svg
						class="pointer-events-none absolute inset-0 z-10 h-full w-full"
						viewBox="0 0 9 9"
						preserveAspectRatio="none"
						stroke-linecap="round"
						overflow="visible"
					>
						<!-- thin lines -->
						{#each [1, 2, 4, 5, 7, 8] as i (i)}
							<line x1={i} y1={0} x2={i} y2={9} class="grid-line-thin" stroke-width="0.02" />
							<line x1={0} y1={i} x2={9} y2={i} class="grid-line-thin" stroke-width="0.02" />
						{/each}
						<!-- thick lines -->
						{#each [3, 6] as i (i)}
							<line x1={i} y1={0} x2={i} y2={9} class="stroke-accent" stroke-width="0.06" />
							<line x1={0} y1={i} x2={9} y2={i} class="stroke-accent" stroke-width="0.06" />
						{/each}
					</svg>
					<div class={['row grid h-full w-full grid-cols-9 grid-rows-9', { solved: boardSolved }]}>
						{#if sudokuStore.sudoku}
							{#each sudokuStore.sudoku as row, i (i)}
								{#each row as cell, j (j)}
									<div
										class={[
											'cell border-accent',
											selectionBorders(i, j),
											(() => {
												const index = cellIndex(i, j);
												const isSelected = selectedCells.has(index);
												const hasSelection = selectedCells.size > 0;
												const primaryValue =
													hasSelection && primaryCell !== null
														? sudokuStore.sudoku[Math.floor(primaryCell / 9)][primaryCell % 9].value
														: 0;
												const peer = isPeer(i, j);
												const isSingle = selectedCells.size === 1;
												const sameDigit =
													isSingle &&
													primaryValue > 0 &&
													cell.value === primaryValue &&
													!isSelected;
												const matchingNote =
													isSingle &&
													primaryValue > 0 &&
													cell.notes.includes(primaryValue) &&
													!isSelected;
												return {
													'bg-accent/60': isSelected,
													'bg-accent/20': !isSelected && (peer || sameDigit),
													'bg-accent/10': !isSelected && !peer && !sameDigit && matchingNote
												};
											})()
										]}
										style:animation-delay={boardSolved ? `${(i + j) * 40}ms` : ''}
									>
										{#if cell.colors.length > 0}
											<div
												class="absolute inset-0"
												style:background={cellGradient(cell.colors)}
											></div>
										{/if}
										<button
											onkeydown={handleCellKeydown}
											onpointerdown={(ev) => handlePointerDown(ev, j, i)}
											onpointermove={(ev) => handlePointerMove(ev, j, i)}
											class="relative z-10 grid h-full w-full touch-none place-items-center"
										>
											{#if cell.value > 0}
												<span
													class:text-red-400={sudokuStore.solved &&
														cell.value !== sudokuStore.solved[i][j]}
													class="text-xl font-medium"
												>
													{cell.value}
												</span>
											{:else}
												<!-- notes -->
												<div
													class="grid h-full w-full grid-cols-3 grid-rows-3 place-items-center text-xs leading-none"
												>
													{#each { length: 9 } as _, x (x)}
														{#if cell.notes.length > 0 && cell.notes.includes(x + 1)}
															<div class="text-xs leading-none">{x + 1}</div>
														{:else}
															<div></div>
														{/if}
													{/each}
												</div>
											{/if}
										</button>
									</div>
								{/each}
							{/each}
						{/if}
					</div>
				</div>
			</div>
		</div>

		<div>
			<div class="digits mx-auto mb-8 grid w-min gap-4">
				{#each { length: 9 } as _, i (i)}
					<button
						onclick={() => {
							setSudokuCell(i + 1);
						}}
						class="digit-btn"
						class:bg-accent={selectedNumber === i + 1}
						style:background={mode === 'color' ? `${CELL_COLORS[i]}66` : ''}
						style:border-color={mode === 'color' ? CELL_COLORS[i] : ''}
					>
						{#if mode !== 'color'}
							<span class="relative">
								{i + 1}
								{#if settingsStore.settings.showRemainingDigits}
									<sup class="absolute -right-[7px] top-1">{remainingCount(i + 1)}</sup>
								{/if}
							</span>
						{/if}
					</button>
				{/each}

				<button class="digit-btn" onclick={() => clearCell()}>
					<EraserIcon size="22" />
				</button>
			</div>

			<div class="container flex max-w-3xl flex-row justify-center gap-6">
				<button
					class="option-btn"
					class:bg-accent={mode === 'color'}
					class:border-accent={mode === 'color'}
					class:text-background={mode === 'color'}
					aria-label="coloring"
					onclick={() => {
						if (mode === 'color') {
							mode = 'fill';
						} else {
							mode = 'color';
						}
					}}
				>
					<PaintBucketIcon class="transition" />
				</button>

				<button
					class="option-btn"
					class:bg-accent={mode === 'note'}
					class:border-accent={mode === 'note'}
					class:text-background={mode === 'note'}
					aria-label="note"
					onclick={() => {
						if (mode === 'note') {
							mode = 'fill';
						} else {
							mode = 'note';
						}
					}}
				>
					<PencilIcon class="transition" />
				</button>

				<button
					class="option-btn"
					class:opacity-30={!history.canUndo}
					aria-label="undo"
					disabled={!history.canUndo}
					onclick={handleUndo}
				>
					<UndoIcon />
				</button>

				<button
					class="option-btn"
					class:opacity-30={!history.canRedo}
					aria-label="redo"
					disabled={!history.canRedo}
					onclick={handleRedo}
				>
					<RedoIcon />
				</button>
			</div>
		</div>
	</main>
</div>

<HintDialog bind:open={dialogState.hintOpen} hint={sudokuStore.lastHint} />
<GameFinishedDialog bind:open={dialogState.gameFinishedOpen} />

<style lang="postcss">
	/* .sudoku * * {
		box-sizing: content-box;
	} */

	.digits {
		grid-template-columns: repeat(5, min-content);
	}

	.digit-btn {
		@apply grid h-12 w-12 place-items-center rounded-full border border-text bg-transparent;
	}

	.option-btn {
		@apply grid w-full place-items-center rounded-full border border-transparent py-1 transition hover:border-accent;
	}

	.grid-line-thin {
		stroke: hsl(var(--color-accent-hsl) / 0.4);
	}

	.cell {
		@apply relative;
		border: 2px solid transparent;
	}

	.cell.border-t-2 {
		border-top-color: hsl(var(--color-accent-hsl));
	}

	.cell.border-b-2 {
		border-bottom-color: hsl(var(--color-accent-hsl));
	}

	.cell.border-l-2 {
		border-left-color: hsl(var(--color-accent-hsl));
	}

	.cell.border-r-2 {
		border-right-color: hsl(var(--color-accent-hsl));
	}

	.cell.highlighted {
		@apply bg-accent/20;
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

	.solved .cell {
		animation: cell-celebrate 0.4s ease-out both;
	}
</style>
