<script lang="ts">
	import { hexToHSL } from '$lib/color';
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

	let mode = $state<'fill' | 'note' | 'color'>('note');

	let backgroundColor = $state('#f0f0f0');
	let accentColor = $state('#ff0000');

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

<main class="py-4">
	<!-- Base color scheme dark, light, adapt to system -->
	<!-- Accent color -->

	<input type="color" bind:value={backgroundColor} />
	<p>{JSON.stringify(hexToHSL(backgroundColor))}</p>

	<input type="color" bind:value={accentColor} />
	<p>{JSON.stringify(hexToHSL(accentColor))}</p>

	<div class="container">
		<div class="sudoku mb-8">
			<div class="row mx-10 grid aspect-square grid-cols-9">
				{#each sudokuGrid as row, i}
					{#each row as cell, j}
						<div
							class="cell aspect-square border border-black"
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

	<p>Mode: {mode}</p>
	<div class="flex flex-row justify-center gap-6">
		<button
			aria-label="erase"
			onclick={() => {
				// mode = 'erase';
			}}
		>
			<EraserIcon />
		</button>

		<button
			aria-label="coloring"
			onclick={() => {
				mode = 'color';
			}}
		>
			<PaintBucketIcon />
		</button>

		<button
			aria-label="note"
			onclick={() => {
				mode = 'note';
			}}
		>
			<PencilIcon />
		</button>

		<button aria-label="undo">
			<UndoIcon />
		</button>

		<button aria-label="redo">
			<RedoIcon />
		</button>
	</div>
</main>

<style lang="postcss">
	/* --color-accent-hsl: var(--accent-h), var(--accent-s), var(--accent-l);
    --color-accent: hsl(var(--accent-h), var(--accent-s), var(--accent-l));
    --color-accent-1: hsl(calc(var(--accent-h) - 3), calc(var(--accent-s) * 1.02), calc(var(--accent-l) * 1.15));
    --color-accent-2: hsl(calc(var(--accent-h) - 5), calc(var(--accent-s) * 1.05), calc(var(--accent-l) * 1.29)); */

	.options {
		grid-template-columns: repeat(5, min-content);
	}

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

	.cell.selected {
		@apply bg-sky-400/40;
	}
</style>
