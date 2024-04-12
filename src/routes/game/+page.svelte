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
	import { invoke } from '@tauri-apps/api/core';

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
		console.info(ev.key, 'down on', { col, row });
	}

	function setSudokuCell(num: number) {
		if (!sudokuStore.sudoku) return;

		const { col, row } = selectedCell;

		if (col < 0 || row < 0) return;

		if (sudokuStore.sudoku[row][col].fixed) return;

		if (mode === 'fill') {
			sudokuStore.sudoku[row][col].value = num;
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
						if (!sudokuStore.sudoku) return;

						const sudokuString = sudokuStore.sudoku
							.map((row) => row.map((cell) => cell.value).join(''))
							.join('')
							.replace(/0/g, '.');

						console.log(sudokuString);

						const res = await invoke('hint_sudoku', { sudokuLine: sudokuString });
						console.log(res);
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
				<div class="row mx-auto grid h-full max-h-full w-full max-w-full grid-cols-9 grid-rows-9">
					{#if sudokuStore.sudoku}
						{#each sudokuStore.sudoku as row, i}
							{#each row as cell, j}
								<div
									class="cell border border-accent/40"
									class:selected={selectedCell.col === j && selectedCell.row === i}
									class:bg-accent={selectedCell.row >= 0 &&
										selectedCell.col >= 0 &&
										cell.value !== 0 &&
										sudokuStore.sudoku[selectedCell.row][selectedCell.col] === cell}
									class:bg-blue-400={selectedCell.row >= 0 &&
										selectedCell.col >= 0 &&
										cell.notes.includes(
											sudokuStore.sudoku[selectedCell.row][selectedCell.col].value
										)}
								>
									<button
										onkeydown={(ev) => handleCellKeydown(ev, j, i)}
										onclick={() => handleCellClick(j, i)}
										class="grid h-full w-full place-items-center"
									>
										{#if cell.value > 0}
											<span class="text-xl font-medium">{cell.value}</span>
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

<HintDialog bind:open={dialogState.hintOpen} />

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
