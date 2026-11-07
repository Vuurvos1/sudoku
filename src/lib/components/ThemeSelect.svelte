<script lang="ts">
	import { themeStore, themes } from '#lib/stores/index.ts';
</script>

<div role="radiogroup" aria-label="Theme" class="grid grid-cols-4 gap-2">
	{#each Object.entries(themes) as [key, theme] (key)}
		{@const active = themeStore.get().id === theme.id}
		<button
			role="radio"
			aria-checked={active}
			onclick={() => themeStore.setTheme(key)}
			class="flex flex-col items-center gap-2 py-1 text-sm capitalize"
		>
			<span
				class={[
					'grid h-11 w-11 place-items-center rounded-full transition',
					active ? 'ring-2 ring-accent ring-offset-2 ring-offset-background' : 'ring-1 ring-text/30'
				]}
				style:background={theme.background}
			>
				<span class="h-4 w-4 rounded-full" style:background={theme.accent}></span>
			</span>
			<span class={active ? 'font-semibold' : 'opacity-70'}>{key}</span>
		</button>
	{/each}
</div>
