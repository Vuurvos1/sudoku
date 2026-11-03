<script lang="ts">
	import Sheet from '$lib/components/Sheet.svelte';

	let {
		open = $bindable(false),
		title,
		message,
		confirmLabel,
		onconfirm
	}: {
		open?: boolean;
		title: string;
		message: string;
		confirmLabel: string;
		onconfirm: () => void;
	} = $props();
</script>

<!-- asks before an action that throws away the player's work -->
<Sheet bind:open {title}>
	<p class="opacity-80">{message}</p>

	<div class="mt-6 grid grid-cols-2 gap-3">
		<button
			class="rounded-full py-2.5 font-semibold ring-1 ring-inset ring-text/20 transition hover:bg-text/5"
			onclick={() => (open = false)}
		>
			Cancel
		</button>
		<button
			class="rounded-full bg-accent py-2.5 font-semibold text-background transition hover:opacity-90"
			onclick={() => {
				open = false;
				onconfirm();
			}}
		>
			{confirmLabel}
		</button>
	</div>
</Sheet>
