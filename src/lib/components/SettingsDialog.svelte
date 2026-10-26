<script lang="ts">
	import Dialog from '$lib/components/Dialog.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import ThemeSelect from '$lib/components/ThemeSelect.svelte';
	import { NoteStyles, settingsStore, type MistakeMode } from '$lib/stores';

	let { open = $bindable(true) }: { open?: boolean } = $props();

	const mistakeModes: { mode: MistakeMode; label: string; description: string }[] = [
		{ mode: 'off', label: 'Off', description: 'Mistakes are not marked.' },
		{
			mode: 'rules',
			label: 'Rules',
			description: 'Marks digits that break a rule with the digits already placed.'
		},
		{
			mode: 'solution',
			label: 'Solution',
			description: "Marks every digit that doesn't match the solution."
		}
	];
	const mistakeDescription = $derived(
		mistakeModes.find(({ mode }) => mode === settingsStore.settings.mistakes)?.description
	);
</script>

<!-- Base color scheme dark, light, adapt to system -->

<Dialog bind:open>
	<h2 class="mb-3 text-2xl font-semibold">Settings</h2>

	<h3 class="mb-2 text-xl font-semibold">Themes</h3>

	<ThemeSelect />

	<h3 class="mb-2 text-xl font-semibold">Gameplay</h3>

	<Switch label="Show timer" bind:checked={settingsStore.settings.showTimer} />

	<Switch label="Show remaining digits" bind:checked={settingsStore.settings.showRemainingDigits} />

	<h4 class="text-lg font-semibold">Mistakes</h4>

	<div class="flex w-min flex-row divide-x rounded border">
		{#each mistakeModes as { mode, label } (mode)}
			<button
				onclick={() => (settingsStore.settings.mistakes = mode)}
				class="px-2 py-1 hover:bg-accent-1"
				class:bg-accent={settingsStore.settings.mistakes === mode}
				class:text-background={settingsStore.settings.mistakes === mode}
			>
				{label}
			</button>
		{/each}
	</div>
	<p class="mb-2 mt-1 text-sm opacity-70">{mistakeDescription}</p>

	<h4 class="text-lg font-semibold">Note style</h4>

	<div class="flex w-min flex-row divide-x rounded border">
		<button
			onclick={() => (settingsStore.settings.noteStyle = NoteStyles.Grid)}
			class="p-1 hover:bg-accent-1"
			class:bg-accent={settingsStore.settings.noteStyle === NoteStyles.Grid}
			class:text-background={settingsStore.settings.noteStyle === NoteStyles.Grid}
			aria-label="grid note style"
		>
			<svg
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3Z"
				/>
				<path d="M16 12H16.01" />
				<path d="M16 16H16.01" />
				<path d="M8 8H8.01" />
			</svg>
		</button>

		<button
			onclick={() => (settingsStore.settings.noteStyle = NoteStyles.Row)}
			class="p-1 hover:bg-accent-1"
			class:bg-accent={settingsStore.settings.noteStyle === NoteStyles.Row}
			class:text-background={settingsStore.settings.noteStyle === NoteStyles.Row}
			aria-label="row note style"
		>
			<svg
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M19 3H5C3.89543 3 3 3.89543 3 5V19C3 20.1046 3.89543 21 5 21H19C20.1046 21 21 20.1046 21 19V5C21 3.89543 20.1046 3 19 3Z"
				/>
				<path d="M16 12H16.01" />
				<path d="M12 12H12.01" />
				<path d="M8 12H8.01" />
			</svg>
		</button>
	</div>
</Dialog>
