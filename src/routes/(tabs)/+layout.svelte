<script lang="ts">
	import type { Snippet } from 'svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { BookOpenIcon, HouseIcon, LayoutGridIcon, SettingsIcon } from '@lucide/svelte';
	import { dialogState } from '$lib/stores';

	let { children }: { children?: Snippet } = $props();

	const tabs = [
		{ path: '/', label: 'Home', icon: HouseIcon },
		{ path: '/puzzles', label: 'Puzzles', icon: LayoutGridIcon },
		{ path: '/learn', label: 'Learn', icon: BookOpenIcon }
	] as const;
</script>

<!-- the main screens, with a tab bar; the game screen lives outside this group -->
<div class="flex h-full flex-col">
	<div class="relative min-h-0 flex-1 overflow-y-auto">
		<button
			class="absolute right-2 top-2 grid h-11 w-11 place-items-center rounded-full transition hover:bg-text/10"
			aria-label="Settings"
			onclick={() => (dialogState.settingsOpen = true)}
		>
			<SettingsIcon />
		</button>

		<div class="mx-auto w-full max-w-md px-6 pb-8 pt-14">
			{@render children?.()}
		</div>
	</div>

	<nav aria-label="Main" class="grid grid-cols-3 border-t border-text/10">
		{#each tabs as { path, label, icon: Icon } (path)}
			<!-- a tab stays active on its subpages, e.g. a rule under Learn -->
			{@const active =
				page.url.pathname === resolve(path) ||
				(path !== '/' && page.url.pathname.startsWith(`${resolve(path)}/`))}
			<a
				href={resolve(path)}
				aria-current={active ? 'page' : undefined}
				class={[
					'flex flex-col items-center gap-1 py-2 text-xs transition',
					active ? 'font-semibold text-accent' : 'opacity-60 hover:opacity-100'
				]}
			>
				<Icon size={22} />
				{label}
			</a>
		{/each}
	</nav>
</div>
