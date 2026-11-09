<script lang="ts">
	import { settingsStore, dialogState, sudokuStore } from '#lib/stores/index.ts';
	import {
		UndoIcon,
		RedoIcon,
		PencilIcon,
		PaintBucketIcon,
		EraserIcon,
		SettingsIcon,
		ChevronLeftIcon,
		LightbulbIcon,
		InfoIcon,
		WandSparklesIcon,
		RotateCcwIcon
	} from '@lucide/svelte';
	import CenterNotesIcon from '#lib/components/CenterNotesIcon.svelte';
	import CornerNotesIcon from '#lib/components/CornerNotesIcon.svelte';
	import HintDialog from '#lib/components/HintDialog.svelte';
	import GameFinishedDialog from '#lib/components/GameFinishedDialog.svelte';
	import PuzzleRulesSheet from '#lib/components/PuzzleRulesSheet.svelte';
	import Menu from '#lib/components/Menu.svelte';
	import ConfirmSheet from '#lib/components/ConfirmSheet.svelte';
	import { CELL_COLORS, cellColor } from '#lib/color.ts';
	import { resolve } from '$app/paths';
	import Sudoku from '#lib/components/Sudoku.svelte';
	import { ruleLabels } from '#lib/variants/index.ts';
	import { CellSelection } from '#lib/selection.svelte.ts';
	import { Game } from '#lib/game.svelte.ts';
	import { onMount } from 'svelte';

	const selection = new CellSelection();
	const game = new Game(sudokuStore, selection, () => settingsStore.settings);

	$effect(() => {
		if (settingsStore.settings.mistakes === 'rules') game.checkRules();
	});

	const constraints = $derived(sudokuStore.puzzle?.constraints ?? []);
	let rulesOpen = $state(false);
	let confirmFillOpen = $state(false);
	let confirmRestartOpen = $state(false);

	// the button switches cells to the style that isn't the default
	const otherNoteStyle = $derived(
		game.otherNoteStyle === 'center'
			? { label: 'Center notes', icon: CenterNotesIcon }
			: { label: 'Corner notes', icon: CornerNotesIcon }
	);

	// only worth asking when there's something to lose
	function hasProgress() {
		return !!sudokuStore.sudoku
			?.flat()
			.some((cell) => !cell.fixed && (cell.value || cell.notes.length || cell.colors.length));
	}

	// back to the givens; the old moves can't be undone into the new game
	function restart() {
		sudokuStore.restart();
		game.history.clear();
		selection.clear();
		boardSolved = false;
	}

	// filling replaces notes, so ask first when there are some to lose
	function fillNotes() {
		if (game.hasNotes) confirmFillOpen = true;
		else game.fillNotes();
	}

	let now = $state(Date.now());

	// the clock only runs while the game is open and on screen
	onMount(() => {
		sudokuStore.unpause();
		return () => sudokuStore.pause();
	});

	function handleVisibilityChange() {
		if (document.hidden) sudokuStore.pause();
		else {
			sudokuStore.unpause();
			now = Date.now();
		}
	}

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
		return formatTime(sudokuStore.elapsed(now));
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

		if (ev.altKey) return;
		const key = ev.key.toLowerCase();
		const digit = parseInt(ev.key);
		if (digit >= 1 && digit <= 9) game.enter(digit);
		else if (ev.key === 'Backspace' || ev.key === 'Delete') game.clear();
		// the same toggles as the buttons: pressing again goes back to filling in digits
		else if (key === 'n') game.toggleMode('note');
		else if (key === 'c') game.toggleMode('color');
		else if (key === 'm') game.toggleNoteStyle();
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
<svelte:document onvisibilitychange={handleVisibilityChange} />

<div class="flex h-full max-h-full flex-col">
	<header class="py-3">
		<nav class="container flex flex-row items-center justify-between">
			<div class="flex w-32 flex-row items-center gap-4">
				<a href={resolve('/(tabs)')} aria-label="back">
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
				<Menu
					label="more"
					items={[
						{ label: 'Fill all notes', icon: WandSparklesIcon, onselect: fillNotes },
						{
							label: `${otherNoteStyle.label} (M)`,
							icon: otherNoteStyle.icon,
							onselect: () => game.toggleNoteStyle()
						},
						{
							label: 'Restart puzzle',
							icon: RotateCcwIcon,
							onselect: () => (hasProgress() ? (confirmRestartOpen = true) : restart())
						}
					]}
				/>
			</div>
		</nav>
	</header>

	<main class="flex h-auto flex-1 flex-col overflow-hidden py-4">
		<div
			class="sudoku relative mx-auto h-full max-h-full w-full max-w-screen-lg flex-1 overflow-hidden px-4 py-12 md:px-8"
		>
			{#if constraints.length > 0}
				<!-- opens the rules, to look up how one works mid-puzzle -->
				<button
					class="absolute inset-x-0 top-3 mx-auto flex max-w-full items-center justify-center gap-1.5 px-4 py-1 text-sm opacity-70 transition hover:opacity-100"
					onclick={() => (rulesOpen = true)}
				>
					<span class="truncate">{ruleLabels(constraints).join(' · ')}</span>
					<InfoIcon size={16} class="shrink-0" />
				</button>
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
			<div class="digits mx-auto mb-8 grid w-min gap-x-2.5 gap-y-3 sm:gap-4">
				{#each { length: 9 } as _, i (i)}
					{@const remaining = game.remaining(i + 1)}
					<button
						onclick={() => {
							game.enter(i + 1);
						}}
						class="digit-btn"
						style:background={game.mode === 'color' ? cellColor(CELL_COLORS[i], 0.4) : ''}
						style:border-color={game.mode === 'color' ? cellColor(CELL_COLORS[i]) : ''}
						class:opacity-60={game.mode !== 'color' && remaining === 0}
						disabled={game.mode !== 'color' && remaining === 0}
						aria-label={game.mode === 'color' ? `Color ${i + 1}` : undefined}
					>
						{#if game.mode === 'color'}
							<!-- numbered, so colors can be told apart without seeing them -->
							{#if settingsStore.settings.colorBlindFriendly}
								<span class="font-semibold">{i + 1}</span>
							{/if}
						{:else}
							<span class="relative">
								{i + 1}
								{#if settingsStore.settings.showRemainingDigits}
									<sup class="absolute -right-[7px] top-1">{remaining}</sup>
								{/if}
							</span>
						{/if}
					</button>
				{/each}

				<button class="digit-btn" aria-label="erase" onclick={() => game.clear()}>
					<EraserIcon size="22" />
				</button>

				<button
					class="digit-btn"
					aria-label="note style: {game.selectionNoteStyle}"
					title="Note style: {game.selectionNoteStyle} (M)"
					onclick={() => game.toggleNoteStyle()}
				>
					<!-- the style the selected cells have now; pressing switches it -->
					{#if game.selectionNoteStyle === 'center'}
						<CenterNotesIcon size={22} />
					{:else}
						<CornerNotesIcon size={22} />
					{/if}
				</button>
			</div>

			<div class="container flex max-w-3xl flex-row justify-center gap-6">
				<button
					class={['option-btn', game.mode === 'color' && 'border-accent bg-accent text-background']}
					aria-label="coloring"
					title="Coloring (C)"
					onclick={() => game.toggleMode('color')}
				>
					<PaintBucketIcon class="transition" />
				</button>

				<button
					class={['option-btn', game.mode === 'note' && 'border-accent bg-accent text-background']}
					aria-label="note"
					title="Notes (N)"
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
<PuzzleRulesSheet bind:open={rulesOpen} {constraints} />
<ConfirmSheet
	bind:open={confirmFillOpen}
	title="Replace your notes?"
	message="Filling all notes replaces the notes you already have in every empty cell. You can undo it."
	confirmLabel="Fill notes"
	onconfirm={() => game.fillNotes()}
/>
<ConfirmSheet
	bind:open={confirmRestartOpen}
	title="Restart this puzzle?"
	message="Clears every digit, note and color you've added and resets the timer. This can't be undone."
	confirmLabel="Restart"
	onconfirm={restart}
/>

<style lang="postcss">
	.digits {
		grid-template-columns: repeat(6, min-content);
	}

	.digit-btn {
		@apply grid h-11 w-11 place-items-center rounded-full border border-text bg-transparent sm:h-12 sm:w-12;
	}

	.option-btn {
		@apply grid w-full place-items-center rounded-full border border-transparent py-1 transition hover:border-accent;
	}
</style>
