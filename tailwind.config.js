/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		container: {
			center: true,
			padding: {
				DEFAULT: '1rem',
				sm: '2rem',
				lg: '4rem',
				xl: '5rem',
				'2xl': '6rem'
			}
		},
		extend: {
			colors: {
				background: 'hsl(var(--color-background-hsl) / <alpha-value>)',
				accent: 'hsl(var(--color-accent-hsl) / <alpha-value>)'
			}
		}
	},
	plugins: []
};
