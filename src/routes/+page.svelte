<script lang="ts">
	import Switch from '$lib/components/Switch.svelte';
	import { theme, themes } from '$lib/stores/theme.svelte.ts';
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

	let mode = $state<'fill' | 'note' | 'color' | ''>('');

	const sudokuGrid = $state([
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 7, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]
	]);

	const notes = $state<number[][][]>([
		[[4], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [5], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []],
		[[], [], [], [], [], [], [], [], []]
	]);

	const selectedCell = $state({ row: -1, col: -1 });

	function handleCellClick(col: number, row: number) {
		console.info({ col, row });
		selectedCell.row = row;
		selectedCell.col = col;
	}

	function handleCellKeydown(ev: KeyboardEvent, col: number, row: number) {
		console.info(ev.key, 'down on', { col, row });
	}

	function setSudokuCell(num: number) {
		console.log('setting', selectedCell, 'to', num);

		const { col, row } = selectedCell;

		if (col < 0 || row < 0) return;

		if (mode === 'fill') {
			sudokuGrid[row][col] = num;
		} else if (mode === 'note') {
			// add or remove if needed
			if (notes[row][col].includes(num)) {
				notes[row][col] = notes[row][col].filter((i) => i === num);
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
				<button aria-label="back">
					<ChevronLeftIcon />
				</button>
			</div>

			<h1 class="font-bold">2m 12s</h1>

			<div class="flex w-32 flex-row items-center justify-end gap-4">
				<button aria-label="toggle dark mode">
					<LightbulbIcon />
				</button>
				<button aria-label="settings">
					<SettingsIcon />
				</button>
			</div>
		</nav>
	</header>

	<main class="flex h-auto flex-1 flex-col overflow-hidden py-4 text-accent">
		<!-- Base color scheme dark, light, adapt to system -->

		<!-- <Switch /> -->

		<!-- <input type="color" bind:value={theme.background} />
	<input type="color" bind:value={theme.accent} /> -->

		<div class="container my-8 flex flex-row gap-4">
			{#each Object.entries(themes) as [key, value]}
				<button
					class:active={theme.id === value.id}
					style="background:{value.background}"
					class="grid h-8 w-8 place-items-center rounded-full transition duration-200"
					onclick={() => {
						theme.background = value.background;
						theme.accent = value.accent;
						theme.id = value.id;
					}}
				>
					<div
						style="background:{value.accent}"
						class="grid h-4 w-4 place-items-center rounded-full outline-0 transition duration-200"
					>
						<div
							style="background:{value.background}"
							class="h-4 w-4 scale-0 rounded-full transition duration-200"
						></div>
					</div>
				</button>
			{/each}
		</div>

		<!-- flex items-center justify-center -->
		<div
			class="sudoku mx-auto h-full max-h-full w-full max-w-screen-lg flex-1 overflow-hidden px-6 py-12 md:px-8"
		>
			<div class="mx-auto flex aspect-square max-h-full">
				<div class="row mx-auto grid h-full max-h-full w-full max-w-full grid-cols-9 grid-rows-9">
					{#each sudokuGrid as row, i}
						{#each row as cell, j}
							<div
								class="cell border border-black"
								class:selected={selectedCell.col === j && selectedCell.row === i}
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
													<div class="text-xs">{x + 1}</div>
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
			<div class="options mx-auto grid w-min gap-4">
				{#each { length: 9 } as _, i}
					<button
						onclick={() => {
							setSudokuCell(i + 1);
						}}
						class="grid h-12 w-12 place-items-center rounded-full border border-gray-800"
					>
						<p>
							{i + 1}<sup>2</sup>
						</p>
					</button>
				{/each}

				<button>x</button>
			</div>

			<div class="container flex flex-row justify-center gap-6">
				<button
					class="grid w-full place-items-center rounded-full border border-transparent py-1 transition"
					aria-label="erase"
					onclick={() => {
						// mode = 'erase';
						mode = '';
					}}
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
							mode = '';
						} else {
							mode = 'color';
						}
					}}
				>
					<PaintBucketIcon class="transition" />
				</button>

				<button
					class="grid w-full place-items-center rounded-full border border-transparent py-1 transition"
					class:bg-accent={mode === 'note'}
					class:border-accent={mode === 'note'}
					class:text-background={mode === 'note'}
					aria-label="note"
					onclick={() => {
						if (mode === 'note') {
							mode = '';
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
					onclick={() => {
						mode = '';
					}}
				>
					<UndoIcon />
				</button>

				<button
					class="grid w-full place-items-center rounded-full py-1 transition"
					aria-label="redo"
					onclick={() => {
						mode = '';
					}}
				>
					<RedoIcon />
				</button>
			</div>
		</div>
	</main>
</div>

<style lang="postcss">
	.d1 {
		background-color: var(--color-accent);
	}

	.d2 {
		background-color: var(--color-accent-1);
	}

	.d3 {
		background-color: var(--color-accent-2);
	}

	button.active > div {
		box-shadow: 0 0 0 12px var(--color-accent);
	}

	button.active > div > div {
		transform: scale(1);
		transition-delay: 0.075s;
	}

	.options {
		grid-template-columns: repeat(5, min-content);
	}

	/* thiccc borders */
	.cell:nth-child(n + 10):nth-child(-n + 9) {
		border-top-width: 4px;
	}

	.cell:nth-child(n + 19):nth-child(-n + 27) {
		border-bottom-width: 4px;
	}

	.cell:nth-child(n + 46):nth-child(-n + 54) {
		border-bottom-width: 4px;
	}

	.cell:nth-child(3n):not(:nth-child(9n)) {
		border-right-width: 4px;
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
