<img src="https://github.com/tauri-apps/create-tauri-app/raw/dev/.github/splash.png" alt="Rapidly scaffold out a new Tauri app project." />

[![](https://img.shields.io/crates/v/create-tauri-app)](https://crates.io/crates/create-tauri-app)
[![](https://img.shields.io/npm/v/create-tauri-app.svg)](https://www.npmjs.com/package/create-tauri-app)
[![status](https://img.shields.io/badge/status-stable-blue.svg)](https://github.com/tauri-apps/tauri)
[![Chat Server](https://img.shields.io/badge/chat-discord-7289da.svg)](https://discord.gg/SpmNs4S)
[![website](https://img.shields.io/badge/website-tauri.app-purple.svg)](https://tauri.app)
[![https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg](https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg)](https://good-labs.github.io/greater-good-affirmation)
[![support](https://img.shields.io/badge/sponsor-Open%20Collective-blue.svg)](https://opencollective.com/tauri)
[![changelog](https://img.shields.io/badge/CHANGELOG-yellowgreen)](./CHANGELOG.md)

# Usage

## Bash:

```
sh <(curl https://create.tauri.app/sh)
```

or

```
sh <(wget https://create.tauri.app/sh)
```

## Powershell:

```powershell
irm https://create.tauri.app/ps | iex
```

## Cargo:

```bash
cargo install create-tauri-app
cargo create-tauri-app
```

## NPM:

```bash
npm create tauri-app@latest
```

## Yarn:

```bash
yarn create tauri-app
```

## PNPM:

```bash
pnpm create tauri-app
```

## Bun:

```bash
bunx create-tauri-app
```

<br>

You can also directly specify the project name, package manager and the template you want to use via additional command line options. For example, to scaffold a Svelte project in a `my-tauri-app` directory, run:

```bash
# curl
sh <(curl https://create.tauri.app/sh) my-tauri-app --template svelte --manager pnpm
# wget
sh <(wget https://create.tauri.app/sh) my-tauri-app --template svelte --manager pnpm
# powershell
$env:CTA_ARGS="--template svelte --manager pnpm";irm https://create.tauri.app/ps | iex
# cargo
cargo create-tauri-app my-tauri-app --template svelte --manager pnpm
# npm 6.x
npm create tauri-app@latest my-tauri-app --template svelte --manager pnpm
# npm 7+, extra double-dash is needed:
npm create tauri-app@latest my-tauri-app -- --template svelte --manager pnpm
# yarn
yarn create tauri-app my-tauri-app --template svelte --manager pnpm
# pnpm
pnpm create tauri-app my-tauri-app --template svelte --manager pnpm
# Bun
bunx create-tauri-app my-tauri-app --template svelte --manager bun
```

Currently supported template presets include:

- `vanilla`
- `vanilla-ts`
- `vue`
- `vue-ts`
- `svelte`
- `svelte-ts`
- `react`
- `react-ts`
- `solid`
- `solid-ts`
- `angular`
- `yew`
- `leptos`
- `sycamore`
- `blazor`

You can use `.` for the project name to scaffold in the current directory.

## Semver

**create-tauri-app** is following [Semantic Versioning 2.0](https://semver.org/).

## Licenses

Code: (c) 2022 - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.

Logo: CC-BY-NC-ND

- Original Tauri Logo Designs by [Daniel Thompson-Yvetot](https://github.com/nothingismagick) and [Guillaume Chau](https://github.com/akryum)
