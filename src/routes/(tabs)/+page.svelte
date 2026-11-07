<script lang="ts">
	import { ChevronRightIcon, Grid3x3Icon, PlayIcon, SplineIcon } from '@lucide/svelte';
	import { sudokuStore } from '#lib/stores/index.ts';
	import { puzzleRules } from '#lib/variants/index.ts';
	import { formatTime } from '#lib/time.ts';
	import NewGameSheet from '#lib/components/NewGameSheet.svelte';
	import Logo from '#lib/components/Logo.svelte';
	import { resolve } from '$app/paths';
	import { goto } from '$app/navigation';

	const canContinue = $derived(sudokuStore.sudoku !== null && !sudokuStore.isSolved);

	// the game in progress, e.g. "12:40 · Knight's move"; time first, a long rule list gets cut off
	const continueDetail = $derived.by(() => {
		const kind = puzzleRules(sudokuStore.puzzle?.constraints ?? []).join(', ');
		const { startTime } = sudokuStore;
		return startTime ? `${formatTime(Date.now() - startTime)} · ${kind}` : kind;
	});

	// the generated game waits in its own slot while a ready-made puzzle is on the board
	const generatedAside = $derived(sudokuStore.generated !== null && !sudokuStore.isGenerated);
	const generatedDetail = $derived.by(() => {
		const { generated } = sudokuStore;
		if (!generated) return '';
		const kind = puzzleRules(generated.puzzle.constraints).join(', ');
		return `${formatTime(generated.elapsed)} · ${kind}`;
	});

	async function continueGenerated() {
		await sudokuStore.continueGenerated();
		goto(resolve('game'));
	}

	let sheetOpen = $state(false);
</script>

<div class="flex flex-col gap-4">
	<div class="mb-4 text-center">
		<!-- the launch splash's logo lands here -->
		<Logo data-splash-target class="mx-auto h-auto w-20" />
		<h1 class="mt-2 text-2xl font-bold">One Sudoku</h1>
	</div>

	{#if canContinue}
		<a
			href={resolve('game')}
			class="flex items-center gap-4 rounded-2xl bg-accent/15 p-4 ring-1 ring-inset ring-accent/50 transition hover:bg-accent/20"
		>
			<div class="min-w-0 flex-1">
				<p class="text-lg font-semibold">Continue</p>
				<p class="truncate text-sm opacity-70">{continueDetail}</p>
			</div>
			<span
				class="grid h-11 w-11 shrink-0 place-items-center rounded-full bg-accent text-background"
			>
				<PlayIcon size={20} class="fill-current" />
			</span>
		</a>
	{/if}

	{#if generatedAside}
		<button
			onclick={continueGenerated}
			class="flex items-center gap-4 rounded-2xl bg-text/5 p-4 text-left transition hover:bg-text/10"
		>
			<div class="min-w-0 flex-1">
				<p class="text-lg font-semibold">Continue classic game</p>
				<p class="truncate text-sm opacity-70">{generatedDetail}</p>
			</div>
			<span class="grid h-11 w-11 shrink-0 place-items-center rounded-full bg-text/10 text-accent">
				<PlayIcon size={20} class="fill-current" />
			</span>
		</button>
	{/if}

	<div class="grid grid-cols-2 gap-3">
		<!-- a new classic game is generated, variants are picked from the ready-made ones -->
		<button onclick={() => (sheetOpen = true)} class="tile">
			<span class="tile-icon"><Grid3x3Icon size={22} /></span>
			<span>
				<span class="block text-lg font-semibold">Classic</span>
				<span class="block text-sm opacity-70">The rules you know</span>
			</span>
		</button>
		<a href="{resolve('/(tabs)/puzzles')}?section=variants" class="tile">
			<span class="tile-icon"><SplineIcon size={22} /></span>
			<span>
				<span class="block text-lg font-semibold">Variants</span>
				<span class="block text-sm opacity-70">Extra rules</span>
			</span>
		</a>
	</div>

	<a
		href={resolve('/(tabs)/puzzles')}
		class="flex items-center gap-4 rounded-2xl bg-text/5 p-4 transition hover:bg-text/10"
	>
		<div class="flex-1">
			<p class="text-lg font-semibold">Puzzles</p>
			<p class="text-sm opacity-70">Classic and variant puzzles</p>
		</div>
		<ChevronRightIcon class="opacity-50" />
	</a>
</div>

<NewGameSheet bind:open={sheetOpen} />

<style lang="postcss">
	.tile {
		@apply flex flex-col items-start gap-3 rounded-2xl bg-text/5 p-4 text-left transition hover:bg-text/10;
	}

	.tile-icon {
		@apply grid h-11 w-11 place-items-center rounded-xl bg-text/10 text-accent;
	}
</style>
