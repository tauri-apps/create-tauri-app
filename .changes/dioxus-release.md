---
"create-tauri-app": patch
"create-tauri-app-js": patch
---

Pass `--release` to `dx` CLI in `beforeBuildCommand` for `dioxus` template to generate smaller and optimized wasm bundle.