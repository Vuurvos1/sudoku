<script lang="ts">
	import { InfoIcon, SettingsIcon } from 'lucide-svelte';
	import { dialogState, sudokuStore } from '$lib/stores';
	import InfoDialog from '$lib/components/InfoDialog.svelte';
	import logo from '$lib/assets/logo.svg';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	const difficulties = {
		beginner: 10,
		easy: 15,
		medium: 20,
		hard: 30,
		extreme: 40
	};

	let selectedDifficulty = $state(difficulties.medium);
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

		<button
			onclick={() => {
				sudokuStore.generate(selectedDifficulty);
				goto(resolve('/game'));
			}}
			class="inline-block rounded-full border-2 px-6 py-2"
		>
			New game
		</button>

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
