<script lang="ts">
	import SettingsDialog from '#lib/components/SettingsDialog.svelte';
	import { dialogState, settingsStore, sudokuStore } from '#lib/stores/index.ts';
	import { hideSplash } from '#lib/splash.ts';
	import type { Snippet } from 'svelte';
	import '../main.css';

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();

	// accessibility settings act through CSS: see `data-color-blind` and `data-reduce-motion` in main.css
	$effect(() => {
		const root = document.documentElement;
		root.toggleAttribute('data-color-blind', settingsStore.settings.colorBlindFriendly);
		root.toggleAttribute('data-reduce-motion', settingsStore.settings.reduceMotion);
	});

	// the launch splash stays up until the settings and the saved game are in, so nothing
	// shifts underneath it; on Home its logo then moves into place
	$effect(() => {
		if (!settingsStore.loaded || !sudokuStore.loaded) return;
		hideSplash(document.querySelector<HTMLElement>('[data-splash-target]'), {
			animate: !settingsStore.settings.reduceMotion
		});
	});
</script>

{@render children?.()}

<SettingsDialog bind:open={dialogState.settingsOpen} />

<style lang="postcss">
	:global(:root) {
		--color-accent-hsl: var(--accent-h) var(--accent-s) var(--accent-l);
		--color-accent: hsl(var(--accent-h), var(--accent-s), var(--accent-l));
		--color-accent-1: hsl(
			calc(var(--accent-h) - 3),
			calc(var(--accent-s) * 1.02),
			calc(var(--accent-l) * 1.15)
		);
		--color-accent-2: hsl(
			calc(var(--accent-h) - 5),
			calc(var(--accent-s) * 1.05),
			calc(var(--accent-l) * 1.29)
		);

		--color-background-hsl: var(--background-h) var(--background-s) var(--background-l);
		--color-background: hsl(var(--background-h), var(--background-s), var(--background-l));

		--color-text-hsl: var(--text-h) var(--text-s) var(--text-l);
		--color-text: hsl(var(--text-h), var(--text-s), var(--text-l));
	}

	:global(body) {
		background-color: var(--color-background);
		color: var(--color-text);
		transition: background 0.1s linear;
	}
</style>
