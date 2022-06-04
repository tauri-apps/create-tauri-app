# create-tauri-app

 <img align="right" src="https://github.com/tauri-apps/tauri/raw/dev/app-icon.png" height="128" width="128">

[![status](https://img.shields.io/badge/Status-Beta-green.svg)](https://github.com/tauri-apps/tauri)
[![Chat Server](https://img.shields.io/badge/chat-on%20discord-7289da.svg)](https://discord.gg/SpmNs4S)
[![devto](https://img.shields.io/badge/blog-dev.to-black.svg)](https://dev.to/tauri)

![](https://img.shields.io/github/workflow/status/tauri-apps/tauri/test%20library?label=test%20library
)
[![devto](https://img.shields.io/badge/documentation-site-purple.svg)](https://tauri.studio)

[![https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg](https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg)](https://good-labs.github.io/greater-good-affirmation)
[![support](https://img.shields.io/badge/sponsor-Opencollective-blue.svg)](https://opencollective.com/tauri)

| Component | Version                                     |
| --------- | ------------------------------------------- |
| create-tauri-app   | ![](https://img.shields.io/npm/v/create-tauri-app.svg) |

## About Tauri
Tauri is a polyglot and generic system that is very composable and allows engineers to make a wide variety of applications. It is used for building applications for Desktop Computers using a combination of Rust tools and HTML rendered in a Webview. Apps built with Tauri can ship with any number of pieces of an optional JS API / Rust API so that webviews can control the system via message passing. In fact, developers can extend the default API with their own functionality and bridge the Webview and Rust-based backend easily.

Tauri apps can have custom menus and have tray-type interfaces. They can be updated, and are managed by the user's operating system as expected. They are very small, because they use the system's webview. They do not ship a runtime, since the final binary is compiled from rust. This makes the reversing of Tauri apps not a trivial task.

## This module
This is a toolkit that will enable engineering teams to rapidly scaffold out a new tauri-apps project using the frontend framework of their choice (as long as it has been configured).

To learn more about the details of how all of these pieces fit together, please consult this [ARCHITECTURE.md](https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md) document.

## Usage
Run and answer the prompts to get started with your first Tauri app!

With npx:

```shell
npx create-tauri-app
```

With npm:
```shell
npm x create-tauri-app
```

With yarn:
```shell
yarn create tauri-app
```

## Semver
**tauri** is following [Semantic Versioning 2.0](https://semver.org/).
## Licenses
Code: (c) 2021 - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.

Logo: CC-BY-NC-ND
- Original Tauri Logo Designs by [Daniel Thompson-Yvetot](https://github.com/nothingismagick) and [Guillaume Chau](https://github.com/akryum)
