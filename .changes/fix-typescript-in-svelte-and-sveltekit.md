---
"create-tauri-app": patch
---

Fixed an [issue](https://github.com/tauri-apps/create-tauri-app/issues/276) where typescript was not working inside .svelte files in the svelte-kit-ts and svelte-ts templates. In both cases, this was due to the lack of a preprocessor. This was fixed differently in each template.

For sveltekit, the preprocessor was added to the svelte.config.js file. For svelte, the preprocessor was added to the vite.config.js file. For svelte, this required adding svelte-preprocess as a dev dependency.