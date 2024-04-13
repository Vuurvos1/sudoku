<script lang="ts">
	import { themeStore, themes } from '$lib/stores';
</script>

<div class="my-4 flex flex-row flex-wrap gap-4">
	{#each Object.entries(themes) as [key, value]}
		<button
			class:active={themeStore.get().id === value.id}
			style="background:{value.background}"
			class="grid h-8 w-8 place-items-center rounded-full transition duration-200"
			onclick={() => {
				themeStore.setTheme(key);
			}}
		>
			<div
				style="background:{value.accent}"
				class="grid h-4 w-4 place-items-center rounded-full outline-0 transition duration-200"
			>
				<div
					style="background:{value.background}"
					class="h-4 w-4 scale-0 rounded-full transition duration-200"
				></div>
			</div>
		</button>
	{/each}
</div>

<style lang="postcss">
	button.active > div {
		box-shadow: 0 0 0 12px var(--color-accent);
	}

	button.active > div > div {
		transform: scale(1);
		transition-delay: 0.075s;
	}
</style>
