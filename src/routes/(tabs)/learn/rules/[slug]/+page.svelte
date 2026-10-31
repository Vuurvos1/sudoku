<script lang="ts">
	import { ChevronLeftIcon } from '@lucide/svelte';
	import { resolve } from '$app/paths';
	import RuleDetail from '$lib/components/RuleDetail.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<a
	href={resolve('/learn')}
	class="-ml-1 mb-4 inline-flex items-center gap-1 text-sm opacity-70 transition hover:opacity-100"
>
	<ChevronLeftIcon size={18} />
	Learn
</a>

<RuleDetail rule={data.rule} />
