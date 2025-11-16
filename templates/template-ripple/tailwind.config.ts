import type { Config } from 'tailwindcss';
export default {
	content: [
		"./index.html",
		"./src/**/*.{ts,ripple}",
	],
	theme: {
		extend: {},
	},
	plugins: []
} satisfies Config
