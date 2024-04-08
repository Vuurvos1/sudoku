<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { InfoIcon, SettingsIcon } from 'lucide-svelte';

	const difficulties = {
		beginner: 10,
		easy: 15,
		medium: 20,
		hard: 30,
		extreme: 40
	};

	let selectedDifficulty = $state(difficulties.medium);

	let res = $state<any>();

	async function generateSudoku() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		res = await invoke('generate_sudoku');
	}
</script>

<header class="px-4 py-4 sm:px-6">
	<div class="flex flex-row justify-end gap-4">
		<button>
			<InfoIcon />
		</button>

		<button>
			<SettingsIcon />
		</button>
	</div>
</header>

<main class="py-20">
	<section class="container">
		<h1 class="text-center text-3xl font-semibold">One Sudoku</h1>
	</section>

	<section class="container">
		<!-- TODO: game modes? standard, killer, x... -->

		<h2>Difficulty</h2>

		<select bind:value={selectedDifficulty}>
			{#each Object.entries(difficulties) as [key, value]}
				<option {value}>{key}</option>
			{/each}
		</select>

		<p>selected: {selectedDifficulty}</p>

		<pre>
			res
			{JSON.stringify(res, null, 2)}
		</pre>

		<button onclick={generateSudoku}>generate</button>

		<a href="/game" class="rounded-full border px-6 py-2">New game</a>
	</section>
</main>
