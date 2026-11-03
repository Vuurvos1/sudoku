<script lang="ts">
	import type { SVGAttributes } from 'svelte/elements';

	let props: SVGAttributes<SVGSVGElement> = $props();

	// the gradient needs a unique id, in case there are several logos on a page
	const id = $props.id();
	const gradient = `logo-gradient-${id}`;
</script>

<svg viewBox="0 0 256 256" fill="none" aria-hidden="true" {...props}>
	<rect width="256" height="256" rx="128" fill="url(#{gradient})" />
	<path
		d="M54 100.25H202M54 155.75H202M100.25 54V202M155.75 54V202"
		stroke="white"
		stroke-width="10"
		stroke-linecap="round"
	/>
	<path d="M122.296 120.4V115.756H130.972V142H125.788V120.4H122.296Z" fill="white" />
	<defs>
		<radialGradient
			id={gradient}
			cx="0"
			cy="0"
			r="1"
			gradientUnits="userSpaceOnUse"
			gradientTransform="rotate(45) scale(362.039)"
		>
			<stop stop-color="#292929" />
			<stop offset="0.865" stop-color="#1E1E1E" />
		</radialGradient>
	</defs>
</svg>
