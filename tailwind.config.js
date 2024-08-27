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
        '2xl': '6rem',
      },
    },
    extend: {
      colors: {
        background: 'hsl(var(--color-background-hsl) / <alpha-value>)',
        accent: {
          DEFAULT: 'hsl(var(--color-accent-hsl) / <alpha-value>)',
          1: 'var(--color-accent-1)',
          2: 'var(--color-accent-2)',
        },
        text: 'hsl(var(--color-text-hsl) / <alpha-value>)',
      },
    },
  },
  plugins: [],
};
