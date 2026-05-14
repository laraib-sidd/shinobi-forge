/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        accent: {
          DEFAULT: '#b91c1c',
          foreground: '#fef2f2',
        },
      },
    },
  },
  plugins: [],
}
