<script lang="ts">
	import Segmented from '#lib/components/Segmented.svelte';
	import Sheet from '#lib/components/Sheet.svelte';
	import Switch from '#lib/components/Switch.svelte';
	import ThemeSelect from '#lib/components/ThemeSelect.svelte';
	import CenterNotesIcon from '#lib/components/CenterNotesIcon.svelte';
	import CornerNotesIcon from '#lib/components/CornerNotesIcon.svelte';
	import { settingsStore, type MistakeMode, type NoteStyle } from '#lib/stores/index.ts';

	const noteStyles: { value: NoteStyle; label: string; icon: typeof CornerNotesIcon }[] = [
		{ value: 'corner', label: 'Corner', icon: CornerNotesIcon },
		{ value: 'center', label: 'Center', icon: CenterNotesIcon }
	];

	let { open = $bindable(true) }: { open?: boolean } = $props();

	const mistakeModes: { value: MistakeMode; label: string; description: string }[] = [
		{ value: 'off', label: 'Off', description: 'Mistakes are not marked.' },
		{
			value: 'rules',
			label: 'Rules',
			description: 'Marks digits that break a rule with the digits already placed.'
		},
		{
			value: 'solution',
			label: 'Solution',
			description: "Marks every digit that doesn't match the solution."
		}
	];
	const mistakeDescription = $derived(
		mistakeModes.find(({ value }) => value === settingsStore.settings.mistakes)?.description
	);
</script>

<!-- Base color scheme dark, light, adapt to system -->

<Sheet bind:open title="Settings">
	<section>
		<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider opacity-60">Appearance</h3>
		<ThemeSelect />
	</section>

	<section class="mt-7">
		<h3 class="text-xs font-semibold uppercase tracking-wider opacity-60">Gameplay</h3>
		<div class="divide-y divide-text/10">
			<Switch label="Show timer" bind:checked={settingsStore.settings.showTimer} />
			<Switch
				label="Show remaining digits"
				bind:checked={settingsStore.settings.showRemainingDigits}
			/>
		</div>
		<p class="mb-2 mt-3">Note style</p>
		<Segmented
			label="Note style"
			options={noteStyles}
			bind:value={settingsStore.settings.noteStyle}
		/>
		<p class="mt-2 text-sm opacity-60">
			How new notes are shown. Single cells can still be switched to the other style.
		</p>
	</section>

	<section class="mt-7">
		<h3 class="mb-3 text-xs font-semibold uppercase tracking-wider opacity-60">Assists</h3>
		<p class="mb-2">Mark mistakes</p>
		<Segmented
			label="Mark mistakes"
			options={mistakeModes}
			bind:value={settingsStore.settings.mistakes}
		/>
		<p class="mt-2 text-sm opacity-60">{mistakeDescription}</p>

		<div class="mt-2 divide-y divide-text/10">
			<Switch
				label="Auto-clean notes"
				hint="Placing a digit removes it from notes in the cells it sees."
				bind:checked={settingsStore.settings.autoCleanNotes}
			/>
			<Switch
				label="Highlight same digit"
				hint="Marks every cell with the selected digit."
				bind:checked={settingsStore.settings.highlightSameDigit}
			/>
			<Switch
				label="Highlight row, column and box"
				hint="Shades the cells the selection sees."
				bind:checked={settingsStore.settings.highlightHouses}
			/>
		</div>
	</section>

	<section class="mt-7">
		<h3 class="text-xs font-semibold uppercase tracking-wider opacity-60">Accessibility</h3>
		<div class="divide-y divide-text/10">
			<Switch
				label="Color-blind friendly"
				hint="Colors that stay apart with color blindness. Dutch whispers are dashed, renban lines dash-dotted, mistakes underlined and color buttons numbered."
				bind:checked={settingsStore.settings.colorBlindFriendly}
			/>
			<Switch
				label="Reduce motion"
				hint="Skips the solved animation and sliding sheets."
				bind:checked={settingsStore.settings.reduceMotion}
			/>
		</div>
	</section>
</Sheet>
