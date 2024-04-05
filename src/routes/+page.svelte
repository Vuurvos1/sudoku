<script lang="ts">
	const difficulties = {
		beginner: 10,
		easy: 15,
		medium: 20,
		hard: 30,
		extreme: 40
	};

	let selectedDifficulty = $state(difficulties.medium);
</script>

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

		<a href="/game" class="rounded-full border px-6 py-2">New game</a>
	</section>
</main>
