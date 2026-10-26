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
	} from '@lucide/svelte';
	import HintDialog from '$lib/components/HintDialog.svelte';
	import GameFinishedDialog from '$lib/components/GameFinishedDialog.svelte';
	import { CELL_COLORS } from '$lib/color';
	import { resolve } from '$app/paths';
	import Sudoku from '$lib/components/Sudoku.svelte';
	import { ruleLabels } from '$lib/variants';
	import { CellSelection } from '$lib/selection.svelte';
	import { Game } from '$lib/game.svelte';

	const selection = new CellSelection();
	const game = new Game(sudokuStore, selection, () => settingsStore.settings.mistakes);

	$effect(() => {
		if (settingsStore.settings.mistakes === 'rules') game.checkRules();
	});

	const constraints = $derived(sudokuStore.puzzle?.constraints ?? []);

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

	const elapsedDisplay = $derived.by(() => {
		if (!sudokuStore.startTime) return '';
		const end = sudokuStore.endTime ?? now;
		return formatTime(end - sudokuStore.startTime);
	});

	function handleKeydown(ev: KeyboardEvent) {
		if (ev.ctrlKey || ev.metaKey) {
			const key = ev.key.toLowerCase();
			if (key === 'z' && ev.shiftKey) game.redo();
			else if (key === 'z') game.undo();
			else if (key === 'y') game.redo();
			else if (import.meta.env.DEV && key === 'd' && ev.shiftKey) game.almostSolve();
			else return;
			ev.preventDefault();
			return;
		}

		const digit = parseInt(ev.key);
		if (digit >= 1 && digit <= 9) game.enter(digit);
		else if (ev.key === 'Backspace' || ev.key === 'Delete') game.clear();
	}

	let boardSolved = $state(false);

	$effect(() => {
		if (sudokuStore.isSolved && !sudokuStore.endTime) {
			sudokuStore.endTime = Date.now();
			sudokuStore.save();

			selection.clear();
			boardSolved = true;

			setTimeout(() => {
				dialogState.gameFinishedOpen = true;
			}, 1000);
		}
	});
</script>

<svelte:window onpointerup={() => selection.pointerUp()} onkeydown={handleKeydown} />

<div class="flex h-full max-h-full flex-col">
	<header class="py-3">
		<nav class="container flex flex-row items-center justify-between">
			<div class="flex w-32 flex-row items-center gap-4">
				<a href={resolve('/')} aria-label="back">
					<ChevronLeftIcon />
				</a>
			</div>

			{#if settingsStore.settings.showTimer}
				<h3 class="font-bold">{elapsedDisplay}</h3>
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
			class="sudoku relative mx-auto h-full max-h-full w-full max-w-screen-lg flex-1 overflow-hidden px-4 py-12 md:px-8"
		>
			{#if constraints.length > 0}
				<p class="absolute inset-x-0 top-4 truncate px-4 text-center text-sm opacity-70">
					{ruleLabels(constraints).join(' · ')}
				</p>
			{/if}
			<div class="mx-auto flex aspect-square max-h-full">
				{#if sudokuStore.sudoku}
					<Sudoku
						cells={sudokuStore.sudoku}
						{constraints}
						layers={[game.layer]}
						selected={selection.cells}
						celebrate={boardSolved}
						oncellpointerdown={(ev, index) => selection.pointerDown(ev, index)}
						onpointermove={(ev) => selection.pointerMove(ev)}
					/>
				{/if}
			</div>
		</div>

		<div>
			<div class="digits mx-auto mb-8 grid w-min gap-4">
				{#each { length: 9 } as _, i (i)}
					{@const remaining = game.remaining(i + 1)}
					<button
						onclick={() => {
							game.enter(i + 1);
						}}
						class="digit-btn"
						style:background={game.mode === 'color' ? `${CELL_COLORS[i]}66` : ''}
						style:border-color={game.mode === 'color' ? CELL_COLORS[i] : ''}
						class:opacity-60={remaining === 0}
						disabled={remaining === 0}
					>
						{#if game.mode !== 'color'}
							<span class="relative">
								{i + 1}
								{#if settingsStore.settings.showRemainingDigits}
									<sup class="absolute -right-[7px] top-1">{remaining}</sup>
								{/if}
							</span>
						{/if}
					</button>
				{/each}

				<button class="digit-btn" onclick={() => game.clear()}>
					<EraserIcon size="22" />
				</button>
			</div>

			<div class="container flex max-w-3xl flex-row justify-center gap-6">
				<button
					class={['option-btn', game.mode === 'color' && 'border-accent bg-accent text-background']}
					aria-label="coloring"
					onclick={() => game.toggleMode('color')}
				>
					<PaintBucketIcon class="transition" />
				</button>

				<button
					class={['option-btn', game.mode === 'note' && 'border-accent bg-accent text-background']}
					aria-label="note"
					onclick={() => game.toggleMode('note')}
				>
					<PencilIcon class="transition" />
				</button>

				<button
					class="option-btn"
					class:opacity-30={!game.history.canUndo}
					aria-label="undo"
					disabled={!game.history.canUndo}
					onclick={() => game.undo()}
				>
					<UndoIcon />
				</button>

				<button
					class="option-btn"
					class:opacity-30={!game.history.canRedo}
					aria-label="redo"
					disabled={!game.history.canRedo}
					onclick={() => game.redo()}
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
	.digits {
		grid-template-columns: repeat(5, min-content);
	}

	.digit-btn {
		@apply grid h-12 w-12 place-items-center rounded-full border border-text bg-transparent;
	}

	.option-btn {
		@apply grid w-full place-items-center rounded-full border border-transparent py-1 transition hover:border-accent;
	}
</style>
