<script lang="ts">
	import { InfoIcon, SettingsIcon } from '@lucide/svelte';
	import { dialogState, sudokuStore, type ConstraintSpec } from '$lib/stores';
	import { ruleLabel } from '$lib/variants';
	import InfoDialog from '$lib/components/InfoDialog.svelte';
	import logo from '$lib/assets/logo.svg';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	const difficulties = {
		easy: 1,
		medium: 2,
		hard: 3,
		expert: 4
	};

	let selectedDifficulty = $state(difficulties.easy);

	// rules that apply to the whole grid, so they can be toggled for a generated puzzle
	type GlobalRule = Extract<ConstraintSpec['type'], 'diagonals' | 'antiKnight' | 'antiKing'>;
	const rules: GlobalRule[] = ['diagonals', 'antiKnight', 'antiKing'];
	let selectedRules = $state<GlobalRule[]>([]);
</script>

<header class="px-4 py-4 sm:px-6">
	<div class="flex flex-row justify-end gap-4">
		<button
			onclick={() => {
				dialogState.infoOpen = true;
			}}
		>
			<InfoIcon />
		</button>

		<button
			onclick={() => {
				dialogState.settingsOpen = true;
			}}
		>
			<SettingsIcon />
		</button>
	</div>
</header>

<main class="py-4">
	<section class="container">
		<!-- TODO: change to inline svg for better animating -->
		<img src={logo} alt="One Sudoku" class="mx-auto h-auto w-40 max-w-full" />

		<h1 class="my-4 text-center text-3xl font-semibold">One Sudoku</h1>
	</section>

	<section class="container flex flex-col items-center justify-center">
		<!-- TODO: game modes? standard, killer, x... -->

		<div class="mb-6">
			<h2>Difficulty</h2>

			<select bind:value={selectedDifficulty}>
				{#each Object.entries(difficulties) as [key, value] (key)}
					<option {value}>{key}</option>
				{/each}
			</select>

			<!-- <p>selected: {selectedDifficulty}</p> -->
		</div>

		<fieldset class="mb-6">
			<legend>Rules</legend>

			{#each rules as rule (rule)}
				<label class="flex flex-row items-center gap-2">
					<input type="checkbox" value={rule} bind:group={selectedRules} />
					{ruleLabel({ type: rule })}
				</label>
			{/each}
		</fieldset>

		<button
			onclick={() => {
				sudokuStore.generate(
					selectedDifficulty,
					selectedRules.map((type) => ({ type }))
				);
				goto(resolve('/game'));
			}}
			class="inline-block rounded-full border-2 px-6 py-2"
		>
			New game
		</button>

		{#if import.meta.env.DEV}
			<button
				onclick={() => {
					sudokuStore.showcase();
					goto(resolve('/game'));
				}}
				class="mt-4 inline-block rounded-full border-2 border-dashed px-6 py-2"
			>
				Test puzzle
			</button>
		{/if}

		{#if sudokuStore.sudoku}
			<button
				onclick={() => {
					goto(resolve('/game'));
				}}
				class="mt-4 inline-block rounded-full border-2 px-6 py-2"
			>
				Continue
			</button>
		{/if}
	</section>
</main>

<InfoDialog bind:open={dialogState.infoOpen} />
