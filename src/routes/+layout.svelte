<script lang="ts">
	import '../main.css';
</script>

<slot />

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
	}

	:global(body) {
		background-color: var(--color-background);
		transition: background 0.1s linear;
	}
</style>
