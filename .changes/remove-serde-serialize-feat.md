---
create-tauri-app: patch
create-tauri-app-js: patch
---

Removed the deprecated `serde-serialize` feature of `wasm-bindgen` in favor of `serde-wasm-bindgen` to prevent cyclic dependency issues.