<script lang="ts">
	import Greet from '$lib/Greet.svelte';

	let mode = $state<'fill' | 'note' | 'color'>('note');

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

<main class="container">
	<!-- Base color scheme dark, light, adapt to system -->
	<!-- Accent color -->

	<input type="color" />

	<div class="flex flex-row">
		<Greet />
	</div>

	<div class="sudoku flex flex-row">
		<div class="row grid grid-cols-9 border-2 border-black">
			{#each sudokuGrid as row, i}
				{#each row as cell, j}
					<div class="cell border border-black">
						<button
							onkeydown={(ev) => handleCellKeydown(ev, j, i)}
							onclick={() => handleCellClick(j, i)}
							class="w-12 h-12 grid place-items-center"
						>
							{#if cell > 0}
								<span class="text-xl font-medium">{cell}</span>
							{:else}
								<!-- notes -->
								<div
									class="grid grid-cols-3 grid-rows-3 text-xs leading-none place-items-center w-full h-full"
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

	<div class="grid grid-cols-5">
		{#each { length: 9 } as _, i}
			<button
				onclick={() => {
					setSudokuCell(i + 1);
				}}
			>
				{i + 1}
			</button>
		{/each}

		<button>x</button>
	</div>

	<div>
		<!-- modes -->
		<p>Mode: {mode}</p>

		<button>Undo</button>
		<button>Redo</button>

		<button
			onclick={() => {
				mode = 'fill';
			}}
		>
			Fill
		</button>
		<button
			onclick={() => {
				mode = 'note';
			}}
		>
			Pencil
		</button>
		<button>Color</button>
	</div>
</main>

<style>
	/* --color-accent-hsl: var(--accent-h), var(--accent-s), var(--accent-l);
    --color-accent: hsl(var(--accent-h), var(--accent-s), var(--accent-l));
    --color-accent-1: hsl(calc(var(--accent-h) - 3), calc(var(--accent-s) * 1.02), calc(var(--accent-l) * 1.15));
    --color-accent-2: hsl(calc(var(--accent-h) - 5), calc(var(--accent-s) * 1.05), calc(var(--accent-l) * 1.29)); */

	.sudoku .cell:nth-child(n + 10):nth-child(-n + 9) {
		border-top-width: 4px;
	}

	.sudoku .cell:nth-child(n + 19):nth-child(-n + 27) {
		border-bottom-width: 4px;
	}

	.sudoku .cell:nth-child(n + 46):nth-child(-n + 54) {
		border-bottom-width: 4px;
	}

	.sudoku .cell:nth-child(3n):not(:nth-child(9n)) {
		border-right-width: 4px;
	}
</style>
