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

To get started using `create-tauri-app` run one of the below commands in the folder you'd like to setup your project.

### Bash:

```
sh <(curl https://create.tauri.app/sh)
```

or

```
sh <(wget https://create.tauri.app/sh)
```

### Powershell:

```powershell
irm https://create.tauri.app/ps | iex
```

### Cargo:

```bash
cargo install create-tauri-app --locked
cargo create-tauri-app
```

### NPM:

```bash
npm create tauri-app@latest
```

### Yarn:

```bash
yarn create tauri-app
```

### PNPM:

```bash
pnpm create tauri-app
```

### Bun:

```bash
bunx create-tauri-app
```

<br>

## Scaffold a new project (interactive)

Follow along with the prompts to choose your project name, frontend language, package manager, and frontend framework, and frontend framework options if applicable.

1. Choose a name and a bundle identifier (unique-id for your app):
   ```
   ? Project name (tauri-app) ›
   ? Identifier (com.tauri-app.app) ›
   ```
2. Select a flavor for your frontend. First the language:
   ```
   ? Choose which language to use for your frontend ›
   Rust  (cargo)
   TypeScript / JavaScript  (pnpm, yarn, npm, bun)
   .NET  (dotnet)
   ```
3. Select a package manager (if there are multiple available):

   Options for **TypeScript / JavaScript**:

   ```
   ? Choose your package manager ›
   pnpm
   yarn
   npm
   bun
   ```

4. Select a UI Template and flavor (if there are multiple available):

   Options for **Rust**:

   ```
   ? Choose your UI template ›
   Vanilla
   Yew
   Leptos
   Sycamore
   ```

   Options for **TypeScript / JavaScript**:

   ```
   ? Choose your UI template ›
   Vanilla
   Vue
   Svelte
   React
   Solid
   Angular
   Preact

   ? Choose your UI flavor ›
   TypeScript
   JavaScript
   ```

   Options for **.NET**:

   ```
   ? Choose your UI template ›
   Blazor  (https://dotnet.microsoft.com/en-us/apps/aspnet/web-apps/blazor/)
   ```

Once completed, the utility reports that the template has been created and displays how to run it using the configured package manager. If it detects missing decencies on your system, it prints a list of packages and prompts how to install them.

## Scaffold a new project (non-interactive)

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
