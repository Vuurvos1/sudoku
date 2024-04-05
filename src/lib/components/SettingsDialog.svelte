<script lang="ts">
	import Dialog from '$lib/components/Dialog.svelte';
	import Switch from '$lib/components/Switch.svelte';
	import ThemeSelect from '$lib/components/ThemeSelect.svelte';
	import { settings } from '$lib/stores/settings.svelte';

	let { open = $bindable(true) }: { open?: boolean } = $props();
</script>

<Dialog bind:open>
	<h2 class="mb-3 text-2xl font-semibold">Settings</h2>

	<h3 class="mb-2 text-xl font-semibold">Themes</h3>

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

	<ThemeSelect />

	<h3 class="mb-2 text-xl font-semibold">Gameplay</h3>

	<Switch label="Show timer" bind:checked={settings.showTimer} />

	<Switch label="Show remaining digits" bind:checked={settings.showRemainingDigits} />
</Dialog>
