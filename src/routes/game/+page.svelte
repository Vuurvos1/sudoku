<script lang="ts">
	import { settings } from '$lib/stores/settings.svelte';
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
	import { generateSudokuPuzzle } from '$lib/sudoku';
	import SettingsDialog from '$lib/components/SettingsDialog.svelte';

	// max 57
	let difficulty = $state(55);

	let mode = $state<'fill' | 'note' | 'color'>('fill');

	// let solvedGrid = $state(generateSudokuPuzzle(difficulty));
	// let initialGrid = $state(generateSudokuPuzzle(difficulty));
	// let coloringGrid
	let sudokuGrid = $state(generateSudokuPuzzle(difficulty));

	let notes = $state<number[][][]>([
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []]
	]);

	const selectedCell = $state({ row: -1, col: -1 });

	let selectedNumber = $state(0);
	let settingsOpen = $state(false);

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
		console.info(ev.key, 'down on', { col, row });
	}

	function setSudokuCell(num: number) {
		const { col, row } = selectedCell;

		if (col < 0 || row < 0) return;

		if (mode === 'fill') {
			sudokuGrid[row][col] = num;
		} else if (mode === 'note') {
			// add or remove if needed
			if (notes[row][col].includes(num)) {
				notes[row][col] = notes[row][col].filter((i) => i !== num);
			} else {
				notes[row][col].push(num);
			}
		}
	}

	function clearCell(col: number, row: number) {
		sudokuGrid[row][col] = 0;
	}
</script>

<div class="flex h-full max-h-full flex-col">
	<header class="py-3">
		<nav class="container flex flex-row items-center justify-between">
			<div class="flex w-32 flex-row items-center gap-4">
				<!-- <button aria-label="back"> -->
				<a href="/">
					<ChevronLeftIcon />
				</a>
				<!-- </button> -->
			</div>

			{#if settings.showTimer}
				<h3 class="font-bold">2m 12s</h3>
			{/if}

			<div class="flex w-32 flex-row items-center justify-end gap-4">
				<button aria-label="toggle dark mode">
					<LightbulbIcon />
				</button>
				<button
					onclick={() => {
						settingsOpen = true;
					}}
					aria-label="settings"
				>
					<SettingsIcon />
				</button>
			</div>
		</nav>
	</header>

	<main class="flex h-auto flex-1 flex-col overflow-hidden py-4">
		<!-- Base color scheme dark, light, adapt to system -->

		<!-- <input type="color" bind:value={theme.background} />
	<input type="color" bind:value={theme.accent} /> -->

		<!-- flex items-center justify-center -->
		<div
			class="sudoku mx-auto h-full max-h-full w-full max-w-screen-lg flex-1 overflow-hidden px-4 py-12 md:px-8"
		>
			<div class="mx-auto flex aspect-square max-h-full">
				<div class="row mx-auto grid h-full max-h-full w-full max-w-full grid-cols-9 grid-rows-9">
					{#each sudokuGrid as row, i}
						{#each row as cell, j}
							<div
								class="cell border border-accent/40"
								class:selected={selectedCell.col === j && selectedCell.row === i}
								class:bg-accent={selectedCell.row >= 0 &&
									selectedCell.col >= 0 &&
									cell !== 0 &&
									sudokuGrid[selectedCell.row][selectedCell.col] === cell}
								class:bg-blue-400={selectedCell.row >= 0 &&
									selectedCell.col >= 0 &&
									notes[i][j].includes(sudokuGrid[selectedCell.row][selectedCell.col])}
							>
								<button
									onkeydown={(ev) => handleCellKeydown(ev, j, i)}
									onclick={() => handleCellClick(j, i)}
									class="grid h-full w-full place-items-center"
								>
									{#if cell > 0}
										<span class="text-xl font-medium">{cell}</span>
									{:else}
										<!-- notes -->
										<div
											class="grid h-full w-full grid-cols-3 grid-rows-3 place-items-center text-xs leading-none"
										>
											{#each { length: 9 } as _, x}
												{#if notes[i][j].length > 0 && notes[i][j].includes(x + 1)}
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
				</div>
			</div>
		</div>

		<div>
			<div class="options mx-auto mb-8 grid w-min gap-4">
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
						class="grid h-12 w-12 place-items-center rounded-full border border-text"
						class:bg-accent={selectedNumber === i + 1}
					>
						<span class="relative">
							{i + 1}
							{#if settings.showRemainingDigits}
								<sup class="absolute -right-[7px] top-1">2</sup>
							{/if}
						</span>
					</button>
				{/each}

				<button
					onclick={() => {
						setSudokuCell(0);
					}}>x</button
				>
			</div>

			<div class="container flex max-w-3xl flex-row justify-center gap-6">
				<button
					class="grid w-full place-items-center rounded-full border border-transparent py-1 transition"
					aria-label="erase"
					onclick={() => {}}
				>
					<EraserIcon />
				</button>

				<button
					class="grid w-full place-items-center rounded-full border border-transparent py-1 transition"
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
					class="grid w-full place-items-center rounded-full border border-transparent py-1 transition hover:border-accent"
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
					class="grid w-full place-items-center rounded-full py-1 transition"
					aria-label="undo"
					onclick={() => {}}
				>
					<UndoIcon />
				</button>

				<button
					class="grid w-full place-items-center rounded-full py-1 transition"
					aria-label="redo"
					onclick={() => {}}
				>
					<RedoIcon />
				</button>
			</div>
		</div>
	</main>

	<SettingsDialog bind:open={settingsOpen} />
</div>

<style lang="postcss">
	.options {
		grid-template-columns: repeat(5, min-content);
	}

	/* thiccc borders */
	.cell:nth-child(n + 10):nth-child(-n + 9) {
		@apply border-t-4 border-t-accent/75;
	}

	.cell:nth-child(n + 19):nth-child(-n + 27) {
		@apply border-b-4 border-b-accent/75;
	}

	.cell:nth-child(n + 46):nth-child(-n + 54) {
		@apply border-b-4 border-b-accent/75;
	}

	.cell:nth-child(3n):not(:nth-child(9n)) {
		@apply border-r-4 border-r-accent/75;
	}

	/* remove outer borders */
	.cell:nth-child(-n + 9) {
		border-top: none;
	}

	.cell:nth-child(n + 73) {
		border-bottom: none;
	}

	.cell:nth-child(9n) {
		border-right: none;
	}

	.cell:nth-child(1n + 1) {
		border-left: none;
	}

	.cell.selected {
		@apply bg-sky-400/40;
	}
</style>
