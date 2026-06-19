---
"create-tauri-app": patch
"create-tauri-app-js": patch
---

Run `svelte-kit sync` after installing generated Svelte templates so the referenced SvelteKit tsconfig exists before checks run.
