<script lang="ts">
	import { ChevronRightIcon } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import RuleIcon from '$lib/components/RuleIcon.svelte';
	import { rules, techniqueLevel, techniques } from '$lib/learn';
</script>

<h1 class="mb-6 text-3xl font-bold">Learn</h1>

<section class="mt-8">
	<h2 class="mb-1 text-xs font-semibold uppercase tracking-wider opacity-60">Variant rules</h2>
	<ul>
		{#each rules as { slug, glyph, name, summary } (slug)}
			<li class="border-b border-text/10">
				<a
					href={resolve('/(tabs)/learn/rules/[slug]', { slug })}
					class="flex items-center gap-4 py-3 transition hover:opacity-80"
				>
					<span
						class="grid h-11 w-11 shrink-0 place-items-center rounded-xl bg-text/10 text-accent"
					>
						<RuleIcon {glyph} />
					</span>
					<span class="min-w-0 flex-1">
						<span class="block font-medium">{name}</span>
						<span class="block text-sm opacity-70">{summary}</span>
					</span>
					<ChevronRightIcon size={18} class="shrink-0 opacity-50" />
				</a>
			</li>
		{/each}
	</ul>
</section>

<section class="mt-8">
	<h2 class="mb-1 text-xs font-semibold uppercase tracking-wider opacity-60">Techniques</h2>
	<ul>
		{#each techniques as { slug, name, difficulty } (slug)}
			<li class="border-b border-text/10">
				<a
					href={resolve('/(tabs)/learn/techniques/[slug]', { slug })}
					class="flex items-center gap-3 py-3.5 transition hover:opacity-80"
				>
					<span class="flex-1 font-medium">{name}</span>
					<span class="text-sm opacity-60">{techniqueLevel(difficulty)}</span>
					<ChevronRightIcon size={18} class="opacity-50" />
				</a>
			</li>
		{/each}
	</ul>
</section>
