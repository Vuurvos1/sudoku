<script lang="ts">
	import { ChevronLeftIcon } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import { techniqueLevel } from '$lib/learn';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const technique = $derived(data.technique);
</script>

<a
	href={resolve('/learn')}
	class="-ml-1 mb-4 inline-flex items-center gap-1 text-sm opacity-70 transition hover:opacity-100"
>
	<ChevronLeftIcon size={18} />
	Learn
</a>

<h1 class="text-3xl font-bold">{technique.name}</h1>
<p class="mt-1 text-sm opacity-60">{techniqueLevel(technique.difficulty)}</p>

<p class="mt-4 text-lg">{technique.summary}</p>

<h2 class="mb-2 mt-8 text-xs font-semibold uppercase tracking-wider opacity-60">How it works</h2>
<ol class="list-decimal space-y-2 pl-5 opacity-80">
	{#each technique.steps as step (step)}
		<li>{step}</li>
	{/each}
</ol>
