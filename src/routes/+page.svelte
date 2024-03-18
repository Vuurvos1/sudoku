<script lang="ts">
	import Greet from '$lib/Greet.svelte';

	let mode = $state<'fill' | 'note' | 'color'>('note');

	const sudokuGrid = $state([
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]
	]);

	const notes = $state([
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

	// const selectedCell = $state({ row: 0, col: 0 });
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
						{#if mode === 'note'}
							{#each notes[i][j] as note}
								<span>{note}</span>
							{/each}
						{:else}
							<input
								type="text"
								class="w-10 h-10 text-center"
								bind:value={sudokuGrid[i][j]}
								placeholder="0"
							/>
						{/if}
					</div>
				{/each}
			{/each}
		</div>
	</div>

	<div>
		<!-- modes -->
	</div>

	<div class="grid grid-cols-5">
		<button>1</button>
		<button>2</button>
		<button>3</button>
		<button>4</button>
		<button>5</button>
		<button>6</button>
		<button>7</button>
		<button>8</button>
		<button>9</button>

		<button>x</button>
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
