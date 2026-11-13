# Tauri Specta template

This template uses [Tauri Specta](https://github.com/specta-rs/tauri-specta) to generate typed frontend bindings for Rust commands. It requires Tauri 2.

Bindings are generated in `src/bindings.js` when the app runs in development. After changing commands, run `cargo test --manifest-path src-tauri/Cargo.toml export_bindings` to regenerate them before building the frontend. Commit the generated file so production builds can run without a development build first.
