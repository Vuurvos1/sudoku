<script lang="ts">
	import { settingsStore, dialogState, sudokuStore } from '$lib/stores';
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

	let mode = $state<'fill' | 'note' | 'color'>('fill');

	const selectedCell = $state({ row: -1, col: -1 });

	let selectedNumber = $state(0);

	function handleCellClick(col: number, row: number) {
		if (selectedCell.row === row && selectedCell.col === col) {
			selectedCell.row = -1;
			selectedCell.col = -1;
		} else {
			selectedCell.row = row;
			selectedCell.col = col;
		}
	}

	function handleCellKeydown(ev: KeyboardEvent, col: number, row: number) {
		const parsed = parseInt(ev.key);
		if (parsed > 0 && parsed <= 9) {
			setSudokuCell(parsed);
		} else if (ev.key === 'Backspace' || ev.key === 'Delete') {
			clearCell(col, row);
		}
	}

	function setSudokuCell(num: number) {
		if (!sudokuStore.sudoku) return;

		const { col, row } = selectedCell;

		if (col < 0 || row < 0) return;
		if (sudokuStore.sudoku[row][col].fixed) return;

		if (mode === 'fill') {
			sudokuStore.sudoku[row][col].value = num;
			sudokuStore.sudoku[row][col].notes = [];

			// clear matching notes from peers
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
					sudokuStore.sudoku[r][c].notes = sudokuStore.sudoku[r][c].notes.filter((n) => n !== num);
				}
			}
		} else if (mode === 'note') {
			// add or remove if needed
			if (sudokuStore.sudoku[row][col].notes.includes(num)) {
				sudokuStore.sudoku[row][col].notes = sudokuStore.sudoku[row][col].notes.filter(
					(i) => i !== num
				);
			} else {
				sudokuStore.sudoku[row][col].notes.push(num);
			}
		}

		sudokuStore.save();
	}

	function clearCell(col: number, row: number) {
		if (!sudokuStore.sudoku) return;

		sudokuStore.sudoku[row][col].value = 0;
	}

	function isPeer(row: number, col: number) {
		if (selectedCell.row < 0 || selectedCell.col < 0) return false;
		const peerMap = sudokuStore.peerMap;
		if (!peerMap) return false;
		const selectedIndex = selectedCell.row * 9 + selectedCell.col;
		const cellIndex = row * 9 + col;
		return peerMap[selectedIndex].includes(cellIndex);
	}
</script>

<div class="flex h-full max-h-full flex-col">
	<header class="py-3">
		<nav class="container flex flex-row items-center justify-between">
			<div class="flex w-32 flex-row items-center gap-4">
				<a href="/" aria-label="back">
					<ChevronLeftIcon />
				</a>
			</div>

			{#if settingsStore.settings.showTimer}
				<h3 class="font-bold">2m 12s</h3>
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
						{#each [1, 2, 4, 5, 7, 8] as i}
							<line x1={i} y1={0} x2={i} y2={9} class="grid-line-thin" stroke-width="0.02" />
							<line x1={0} y1={i} x2={9} y2={i} class="grid-line-thin" stroke-width="0.02" />
						{/each}
						<!-- thick lines -->
						{#each [3, 6] as i}
							<line x1={i} y1={0} x2={i} y2={9} class="stroke-accent" stroke-width="0.06" />
							<line x1={0} y1={i} x2={9} y2={i} class="stroke-accent" stroke-width="0.06" />
						{/each}
					</svg>
					<div class="row grid h-full w-full grid-cols-9 grid-rows-9">
						{#if sudokuStore.sudoku}
							{#each sudokuStore.sudoku as row, i}
								{#each row as cell, j}
									<div
										class={[
											'cell',
											(() => {
												const isSelected = selectedCell.col === j && selectedCell.row === i;
												const hasSelection = selectedCell.row >= 0 && selectedCell.col >= 0;
												const selectedValue = hasSelection
													? sudokuStore.sudoku[selectedCell.row][selectedCell.col].value
													: 0;
												const peer = isPeer(i, j);
												const sameDigit =
													hasSelection &&
													selectedValue > 0 &&
													cell.value === selectedValue &&
													!isSelected;
												const matchingNote =
													hasSelection &&
													selectedValue > 0 &&
													cell.notes.includes(selectedValue) &&
													!isSelected;
												return {
													'selected bg-accent/60': isSelected,
													'bg-accent/20': !isSelected && (peer || sameDigit),
													'bg-accent/10': !isSelected && !peer && !sameDigit && matchingNote
												};
											})()
										]}
									>
										<button
											onkeydown={(ev) => handleCellKeydown(ev, j, i)}
											onclick={() => handleCellClick(j, i)}
											class="grid h-full w-full place-items-center"
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
													{#each { length: 9 } as _, x}
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
				{#each { length: 9 } as _, i}
					<button
						onclick={() => {
							// if (selectedNumber === i + 1) {
							// 	selectedNumber = 0;
							// } else {
							// 	selectedNumber = i + 1;
							// 	if (selectedCell.row >= 0 && selectedCell.col >= 0) {
							// 		setSudokuCell(i + 1);
							// 	}
							// }
							setSudokuCell(i + 1);
						}}
						class="digit-btn"
						class:bg-accent={selectedNumber === i + 1}
					>
						<span class="relative">
							{i + 1}
							{#if settingsStore.settings.showRemainingDigits}
								<sup class="absolute -right-[7px] top-1">2</sup>
							{/if}
						</span>
					</button>
				{/each}

				<button
					class="digit-btn"
					onclick={() => {
						setSudokuCell(0);
					}}
				>
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

				<button class="option-btn" aria-label="undo" onclick={() => {}}>
					<UndoIcon />
				</button>

				<button class="option-btn" aria-label="redo" onclick={() => {}}>
					<RedoIcon />
				</button>
			</div>
		</div>
	</main>
</div>

<HintDialog bind:open={dialogState.hintOpen} hint={sudokuStore.lastHint} />

<style lang="postcss">
	/* .sudoku * * {
		box-sizing: content-box;
	} */

	.digits {
		grid-template-columns: repeat(5, min-content);
	}

	.digit-btn {
		@apply grid h-12 w-12 place-items-center rounded-full border border-text;
	}

	.option-btn {
		@apply grid w-full place-items-center rounded-full border border-transparent py-1 transition hover:border-accent;
	}

	.grid-line-thin {
		stroke: hsl(var(--color-accent-hsl) / 0.4);
	}

	.cell.highlighted {
		@apply bg-accent/20;
	}
</style>
