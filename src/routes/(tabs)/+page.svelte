<script lang="ts">
	import { ChevronRightIcon, Grid3x3Icon, PlayIcon, SplineIcon } from '@lucide/svelte';
	import { sudokuStore } from '$lib/stores';
	import { ruleLabels } from '$lib/variants';
	import { formatTime } from '$lib/time';
	import NewGameSheet, { type GameKind } from '$lib/components/NewGameSheet.svelte';
	import Logo from '$lib/components/Logo.svelte';
	import { resolve } from '$app/paths';

	const canContinue = $derived(sudokuStore.sudoku !== null && !sudokuStore.isSolved);

	// the game in progress, e.g. "12:40 · Knight's move"; time first, a long rule list gets cut off
	const continueDetail = $derived.by(() => {
		const constraints = sudokuStore.puzzle?.constraints ?? [];
		const kind = constraints.length > 0 ? ruleLabels(constraints).join(', ') : 'Classic';
		const { startTime } = sudokuStore;
		return startTime ? `${formatTime(Date.now() - startTime)} · ${kind}` : kind;
	});

	let sheetOpen = $state(false);
	let sheetKind = $state<GameKind>('classic');

	function newGame(kind: GameKind) {
		sheetKind = kind;
		sheetOpen = true;
	}

	const kinds = [
		{ kind: 'classic', title: 'Classic', detail: 'The rules you know', icon: Grid3x3Icon },
		{ kind: 'variants', title: 'Variants', detail: 'Extra rules', icon: SplineIcon }
	] as const;
</script>

<div class="flex flex-col gap-4">
	<div class="mb-4 text-center">
		<!-- the launch splash's logo lands here -->
		<Logo data-splash-target class="mx-auto h-auto w-20" />
		<h1 class="mt-2 text-2xl font-bold">One Sudoku</h1>
	</div>

	{#if canContinue}
		<a
			href={resolve('/game')}
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

	<div class="grid grid-cols-2 gap-3">
		{#each kinds as { kind, title, detail, icon: Icon } (kind)}
			<button
				onclick={() => newGame(kind)}
				class="flex flex-col items-start gap-3 rounded-2xl bg-text/5 p-4 text-left transition hover:bg-text/10"
			>
				<span class="grid h-11 w-11 place-items-center rounded-xl bg-text/10 text-accent">
					<Icon size={22} />
				</span>
				<span>
					<span class="block text-lg font-semibold">{title}</span>
					<span class="block text-sm opacity-70">{detail}</span>
				</span>
			</button>
		{/each}
	</div>

	<a
		href={resolve('/puzzles')}
		class="flex items-center gap-4 rounded-2xl bg-text/5 p-4 transition hover:bg-text/10"
	>
		<div class="flex-1">
			<p class="text-lg font-semibold">Puzzles</p>
			<p class="text-sm opacity-70">Hand-made packs and imports</p>
		</div>
		<ChevronRightIcon class="opacity-50" />
	</a>
</div>

<NewGameSheet bind:open={sheetOpen} kind={sheetKind} />
