# Changelog

## \[4.0.3]

- [`db2f4de`](https://www.github.com/tauri-apps/create-tauri-app/commit/db2f4de39a097a5a8fb7c62889c60fb056aae37a) Fix `blazor` template rendered `Home.razor` incorrectly.

### Dependencies

- Upgraded to `create-tauri-app@4.0.3`

## \[4.0.2]

- [`2321bd4`](https://www.github.com/tauri-apps/create-tauri-app/commit/2321bd4d97e1b9a13108906fd723718f80c32af7) ([#734](https://www.github.com/tauri-apps/create-tauri-app/pull/734) by [@amrbashir](https://www.github.com/tauri-apps/create-tauri-app/../../amrbashir)) Fix `blazor` template `onsubmit` handler.

### Dependencies

- Upgraded to `create-tauri-app@4.0.2`

## \[4.0.1]

- [`96dcd36`](https://www.github.com/tauri-apps/create-tauri-app/commit/96dcd36354a0c656d2ca439bd51e30660fb51235) ([#726](https://www.github.com/tauri-apps/create-tauri-app/pull/726)) Updated `vue-tsc` dependency in `vue-ts` template to fix issues with newer typescript versions.

### Dependencies

- Upgraded to `create-tauri-app@4.0.1`

## \[4.0.0]

- [`11c26e1`](https://www.github.com/tauri-apps/create-tauri-app/commit/11c26e1fc331aa3cf5fa9ba94617bd5be5686ec5)([#271](https://www.github.com/tauri-apps/create-tauri-app/pull/271)) Replace Svelte templates with SvelteKit based on the community [request](https://discord.com/channels/616186924390023171/1232318266542915617/1239666162032181369).

### Dependencies

- Upgraded to `create-tauri-app@4.0.0`

## \[3.14.3]

- [`29ea869`](https://www.github.com/tauri-apps/create-tauri-app/commit/29ea869972ef175578734076c828f38aac20e46d)([#705](https://www.github.com/tauri-apps/create-tauri-app/pull/705)) Fix `capabilities` directory not created when generating a template with mobile support.

### Dependencies

- Upgraded to `create-tauri-app@3.14.1`

## \[3.14.2]

- [`9914fc8`](https://www.github.com/tauri-apps/create-tauri-app/commit/9914fc8ab5749f090ddbb2f2106a66098bbe2a0b) Fix `Cannot find module 'create-tauri-app-darwin-x64'` again.

## \[3.14.1]

- [`edef1cb`](https://www.github.com/tauri-apps/create-tauri-app/commit/edef1cb5a78530b0c16657e11f12474fd473cad0)([#695](https://www.github.com/tauri-apps/create-tauri-app/pull/695)) Fix `Cannot find module 'create-tauri-app-darwin-x64'`

## \[3.14.0]

- [`4edb69f`](https://www.github.com/tauri-apps/create-tauri-app/commit/4edb69f8b79a8f03eb5d29772509fad68f28e904)([#656](https://www.github.com/tauri-apps/create-tauri-app/pull/656)) Add Blazor template
- [`7000c6c`](https://www.github.com/tauri-apps/create-tauri-app/commit/7000c6ce2db33234169cc2b5f98264feb5e02351)([#687](https://www.github.com/tauri-apps/create-tauri-app/pull/687)) Fix `yew`, `leptos` and `sycamore` templates failing with latest versions of `trunk` CLI.

### Dependencies

- Upgraded to `create-tauri-app@3.14.0`

## \[3.13.17]

- [`616a294`](https://www.github.com/tauri-apps/create-tauri-app/commit/616a294a9adaa5d7113961ba8af3bb85727e16c3)([#553](https://www.github.com/tauri-apps/create-tauri-app/pull/553)) Update `leptos` template to `v0.6`

### Dependencies

- Upgraded to `create-tauri-app@3.13.17`

## \[3.13.16]

- [`7a851c8`](https://www.github.com/tauri-apps/create-tauri-app/commit/7a851c8733590ad268b5035eadb74f1d2536c2b5)([#647](https://www.github.com/tauri-apps/create-tauri-app/pull/647)) Ignore `gen/schemas` directory for `--beta` templates.
- [`9f202d7`](https://www.github.com/tauri-apps/create-tauri-app/commit/9f202d7b8846b0023be541a7d6abe91c7c562530)([#649](https://www.github.com/tauri-apps/create-tauri-app/pull/649)) Add `console_error_panic_hook` to `yew`, `leptos` and `sycamore` templates.

### Dependencies

- Upgraded to `create-tauri-app@3.13.16`

## \[3.13.15]

- [`c90a561`](https://www.github.com/tauri-apps/create-tauri-app/commit/c90a5614780eb2b4f0cd5b9ca3ce527bd02bd177)([#645](https://www.github.com/tauri-apps/create-tauri-app/pull/645)) Fix incorrect schema path in beta template's capability.

### Dependencies

- Upgraded to `create-tauri-app@3.13.15`

## \[3.13.14]

- [`88680f1`](https://www.github.com/tauri-apps/create-tauri-app/commit/88680f14c19e2b23bd1f8847be0983a33642578b) Move `vite-env.d.ts` inside `src` directory for `solid-ts` template to fix a typescript error on importing `.svg`;

### Dependencies

- Upgraded to `create-tauri-app@3.13.14`

## \[3.13.13]

- [`44df26b`](https://www.github.com/tauri-apps/create-tauri-app/commit/44df26b63561c50220af9421b6996be46f58d4ce) Fix `leptos` template has invalid dependency specifier in `Cargo.toml`

### Dependencies

- Upgraded to `create-tauri-app@3.13.13`

## \[3.13.12]

- [`0881d1e`](https://www.github.com/tauri-apps/create-tauri-app/commit/0881d1e248f98e43f0ad25d0ac6d654a371061d1)([#635](https://www.github.com/tauri-apps/create-tauri-app/pull/635)) Removed the deprecated `serde-serialize` feature of `wasm-bindgen` in favor of `serde-wasm-bindgen` to prevent cyclic dependency issues.

### Dependencies

- Upgraded to `create-tauri-app@3.13.12`

## \[3.13.11]

- [`209f0d7`](https://www.github.com/tauri-apps/create-tauri-app/commit/209f0d7bc32f40c635e11dd6f9588d1b0850cb12) Fix missing closing quote in the capability file in beta templates.

### Dependencies

- Upgraded to `create-tauri-app@3.13.11`

## \[3.13.10]

- [`1458321`](https://www.github.com/tauri-apps/create-tauri-app/commit/14583219c7efff47c4cb20ed297a067d1dbf5a1a) Fix incorrect schema path in beta temaplate's capability.

### Dependencies

- Upgraded to `create-tauri-app@3.13.10`

## \[3.13.9]

- [`4a2f33b`](https://www.github.com/tauri-apps/create-tauri-app/commit/4a2f33b0c29c9a79cc156abfde5de296578b3f50)([#628](https://www.github.com/tauri-apps/create-tauri-app/pull/628)) Add `--no-mobile` flag.
- [`0574bd8`](https://www.github.com/tauri-apps/create-tauri-app/commit/0574bd824d22af48b63faffa81324723469f551d)([#627](https://www.github.com/tauri-apps/create-tauri-app/pull/627)) Generate tauri v2 beta tempaltes without default features in Cargo.toml as it is no longer needed.

### Dependencies

- Upgraded to `create-tauri-app@3.13.9`

## \[3.13.8]

- [`66a0298`](https://www.github.com/tauri-apps/create-tauri-app/commit/66a0298f76d37452d85fe0e1d7171ed342a0dec1)([#621](https://www.github.com/tauri-apps/create-tauri-app/pull/621)) Fix `productName` incorrectly set when initializing a project in current directory

### Dependencies

- Upgraded to `create-tauri-app@3.13.8`

## \[3.13.7]

- [`6fbdc50`](https://www.github.com/tauri-apps/create-tauri-app/commit/6fbdc50f199eabd948860a688e3294c31eab91c9)([#618](https://www.github.com/tauri-apps/create-tauri-app/pull/618)) Fixed `@tauri-apps/plugin-shell` version in generated templates.

### Dependencies

- Upgraded to `create-tauri-app@3.13.7`

## \[3.13.6]

- [`28b301d`](https://www.github.com/tauri-apps/create-tauri-app/commit/28b301d1f586f5cfde492d2f60ae13a5bc127f91)([#614](https://www.github.com/tauri-apps/create-tauri-app/pull/614)) Change all templates to specify tauri crates and packages version as `^1` so it will always pull latest versions.

### Dependencies

- Upgraded to `create-tauri-app@3.13.6`

## \[3.13.5]

- [`ff5bf7e`](https://www.github.com/tauri-apps/create-tauri-app/commit/ff5bf7e5d0a7157b09f7d499f096b601f22cf838)([#609](https://www.github.com/tauri-apps/create-tauri-app/pull/609)) Update `yew` template to `0.21`

### Dependencies

- Upgraded to `create-tauri-app@3.13.5`

## \[3.13.4]

- [`683f2bb`](https://www.github.com/tauri-apps/create-tauri-app/commit/683f2bbfe038e9ff6c3a62f8729d7409ba3111a9)([#607](https://www.github.com/tauri-apps/create-tauri-app/pull/607)) Update `@vitejs/plugin-vue` version for `vue` and `vue-ts` templates.

### Dependencies

- Upgraded to `create-tauri-app@3.13.4`

## \[3.13.3]

- [`f91ce78`](https://www.github.com/tauri-apps/create-tauri-app/commit/f91ce78498f4c337d54a30df4c6a2bc74fd10564)([#598](https://www.github.com/tauri-apps/create-tauri-app/pull/598)) Fix `vanilla` template generating an invalid `tauri.conf.json` file.

### Dependencies

- Upgraded to `create-tauri-app@3.13.3`

## \[3.13.2]

- [`fe2e7a4`](https://www.github.com/tauri-apps/create-tauri-app/commit/fe2e7a421f1b626f134af62c093842b26ceee833)([#595](https://www.github.com/tauri-apps/create-tauri-app/pull/595)) Fixed an issue where `tauri.conf.json` was missing in projects with mobile support.

### Dependencies

- Upgraded to `create-tauri-app@3.13.2`

## \[3.13.1]

- [`8b3624b`](https://www.github.com/tauri-apps/create-tauri-app/commit/8b3624b6e87c758c1ac1507acce5ed8b7a8b24b8)([#593](https://www.github.com/tauri-apps/create-tauri-app/pull/593)) Fix tauri v2 beta templates.

### Dependencies

- Upgraded to `create-tauri-app@3.13.1`

## \[3.13.0]

- [`4dbdcc9`](https://www.github.com/tauri-apps/create-tauri-app/commit/4dbdcc97a34e53c81350acd8e3b2b72302a796de)([#589](https://www.github.com/tauri-apps/create-tauri-app/pull/589)) Updated the templates to Tauri v2 beta.

### Dependencies

- Upgraded to `create-tauri-app@3.13.0`

## \[3.11.7]

- [`3e6e530`](https://www.github.com/tauri-apps/create-tauri-app/commit/3e6e53044e00cee37369e96826166bd575c77ba3)([#568](https://www.github.com/tauri-apps/create-tauri-app/pull/568)) Fix warnings in `leptos` template related to reactivity.

### Dependencies

- Upgraded to `create-tauri-app@3.12.2`

## \[3.11.6]

- [`6171792`](https://www.github.com/tauri-apps/create-tauri-app/commit/6171792b9641fc5210f4ca79842631e68e59b64e)([#561](https://www.github.com/tauri-apps/create-tauri-app/pull/561)) Upgrade `@vitejs/plugin-react` to `4.2.1` to fix peer depenency for react templates.

### Dependencies

- Upgraded to `create-tauri-app@3.12.1`

## \[3.11.5]

- [`4869b73`](https://www.github.com/tauri-apps/create-tauri-app/commit/4869b730af5679ae574319702ad2414b23299d01)([#558](https://www.github.com/tauri-apps/create-tauri-app/pull/558)) Adapt templates to `@tauri-apps/api@2.0.0-alpha.13` by changing the `primitives` import to `core`.

### Dependencies

- Upgraded to `create-tauri-app@3.12.0`

## \[3.11.4]

- [`96cff24`](https://www.github.com/tauri-apps/create-tauri-app/commit/96cff24c4c86c633e3037cde916f125a12cc9fe0) Fix `--alpha` templates generating an incorrect `main.rs` that failed to compile.

### Dependencies

- Upgraded to `create-tauri-app@3.11.4`

## \[3.11.3]

- [`af40cfc`](https://www.github.com/tauri-apps/create-tauri-app/commit/af40cfcba6477b7e6d5139a74dd7dee408c7810d) Add `-f/--force` flag to force creating directory even if it is empty.
- [`0c40f2b`](https://www.github.com/tauri-apps/create-tauri-app/commit/0c40f2b2f1c81bd07915ba87232769a87100bb0a) Suggest installing `trunk` for `--alpha` templates instead of `amrbashir/trunk` fork as the new version contains all the need features from the fork.
- [`d1978aa`](https://www.github.com/tauri-apps/create-tauri-app/commit/d1978aa2e92b6022ac04d1213a49de899d5f3abf) Fix detection of `tauri-cli` when choosing `cargo` as the package manager.

### Dependencies

- Upgraded to `create-tauri-app@3.11.3`

## \[3.11.2]

- [`aae95d1`](https://www.github.com/tauri-apps/create-tauri-app/commit/aae95d1e48a198b14264dd124ba1172df58ef4ea)([#544](https://www.github.com/tauri-apps/create-tauri-app/pull/544)) Update `angular` template to Angluar 17
- [`aae95d1`](https://www.github.com/tauri-apps/create-tauri-app/commit/aae95d1e48a198b14264dd124ba1172df58ef4ea)([#544](https://www.github.com/tauri-apps/create-tauri-app/pull/544)) Update templates to latest tauri versions (1.5 and 2.0.0-alpha.18).
- [`d2ceefc`](https://www.github.com/tauri-apps/create-tauri-app/commit/d2ceefcc4b9e5af1b0486a9e785e29def957c5e2)([#545](https://www.github.com/tauri-apps/create-tauri-app/pull/545)) Add `src-tauri` by default to vite server ignored watch list.

### Dependencies

- Upgraded to `create-tauri-app@3.11.2`

## \[3.11.1]

- [`e1d1f88`](https://www.github.com/tauri-apps/create-tauri-app/commit/e1d1f8882a12acde242aa266374e0f6772bbf0d3)([#531](https://www.github.com/tauri-apps/create-tauri-app/pull/531)) Fix `angluar` template generated with a malformed `angluar.json`

### Dependencies

- Upgraded to `create-tauri-app@3.11.1`

## \[3.11.0]

- [`c6153e2`](https://www.github.com/tauri-apps/create-tauri-app/commit/c6153e26eb7a2753f42497c28eaf402e4944107b)([#526](https://www.github.com/tauri-apps/create-tauri-app/pull/526)) Fix incorrect `beforeDevCommand` in `angluar` template when using `npm` as the package manager.
- [`c5c0d99`](https://www.github.com/tauri-apps/create-tauri-app/commit/c5c0d9954c105f2ea09fb86749431d9ac37c3479)([#529](https://www.github.com/tauri-apps/create-tauri-app/pull/529)) Generate `tauri.conf.json > project > productName` using the "Project name", which can contain spaces and uppercase letters, instead of "Package name".
- [`3ca02f5`](https://www.github.com/tauri-apps/create-tauri-app/commit/3ca02f58895338cdf95011c2fcefb2cdeab206d4)([#527](https://www.github.com/tauri-apps/create-tauri-app/pull/527)) Disallow using an uppercase in the package name.

### Dependencies

- Upgraded to `create-tauri-app@3.11.0`

## \[3.10.1]

### Dependencies

- Upgraded to `create-tauri-app@3.10.1`

## \[3.10.0]

- [`6c50fc3`](https://www.github.com/tauri-apps/create-tauri-app/commit/6c50fc38019f5284d78873750a3fcdd1d7835931)([#90](https://www.github.com/tauri-apps/create-tauri-app/pull/90)) Update tauri to latest version for all templates and fix `invoke` import in `--alpha` templates.

### Dependencies

- Upgraded to `create-tauri-app@3.10.0`

## \[3.9.0]

- [`e867502`](https://www.github.com/tauri-apps/create-tauri-app/commit/e867502ed59c816e16bf457090e730b92333cafb)([#502](https://www.github.com/tauri-apps/create-tauri-app/pull/502)) Upgrade to leptos 0.5, serde-wasm-bindgen 0.6 and fixing some `cargo fmt`

### Dependencies

- Upgraded to `create-tauri-app@3.9.0`

## \[3.8.0]

- [`f095dce`](https://www.github.com/tauri-apps/create-tauri-app/commit/f095dce22ba24cc2ef13b31491ae02d0162c01a8)([#498](https://www.github.com/tauri-apps/create-tauri-app/pull/498)) Update `@tauri-apps/api` to latest version for `--alpha` templates.
- [`a962ef8`](https://www.github.com/tauri-apps/create-tauri-app/commit/a962ef868d1b86f44d124077ce52301873c52e41)([#481](https://www.github.com/tauri-apps/create-tauri-app/pull/481)) Support and detect Bun package manager
- [`5e48a5c`](https://www.github.com/tauri-apps/create-tauri-app/commit/5e48a5c39fe356efa5734ae5c5ada00b8fabb339)([#486](https://www.github.com/tauri-apps/create-tauri-app/pull/486)) Auto-detect package manager used to run `create-tauri-app` and promote the most relevant category for it and select it and also make the package manager selected by default in the managers selection list.
- [`724ff2b`](https://www.github.com/tauri-apps/create-tauri-app/commit/724ff2b134023f9818b3457da50ca6ef320fec93)([#485](https://www.github.com/tauri-apps/create-tauri-app/pull/485)) Skip deleting `.git` directory when initalizing a project in an already existing directoy.
- [`f095dce`](https://www.github.com/tauri-apps/create-tauri-app/commit/f095dce22ba24cc2ef13b31491ae02d0162c01a8)([#498](https://www.github.com/tauri-apps/create-tauri-app/pull/498)) Update tauri versions for all templates to version `1.5`.

### Dependencies

- Upgraded to `create-tauri-app@3.8.0`

## \[3.7.3]

- [`3449afc`](https://www.github.com/tauri-apps/create-tauri-app/commit/3449afcc9c872d0d20a7751c25b47b9c602f2804)([#476](https://www.github.com/tauri-apps/create-tauri-app/pull/476)) Fix typescript error in `solid-ts` template when importing some non-standard files.
- [`f7256ee`](https://www.github.com/tauri-apps/create-tauri-app/commit/f7256eecfedb3e786a342b574a0737b4ceb110e7)([#471](https://www.github.com/tauri-apps/create-tauri-app/pull/471)) Improve the error messages for  unsupported package manager, unsupported template or when a supported template is used with a package manager that is not intended to be used with.

### Dependencies

- Upgraded to `create-tauri-app@3.7.3`

## \[3.7.2]

- [`3ea6acf`](https://www.github.com/tauri-apps/create-tauri-app/commit/3ea6acf54b1f9517043121b0ce0bacf8910f175c) Cleanup the vite config file for all templates.

### Dependencies

- Upgraded to `create-tauri-app@3.7.2`

## \[3.7.1]

### Dependencies

- Upgraded to `create-tauri-app@3.7.1`

## \[3.7.0]

- [`a092565`](https://www.github.com/tauri-apps/create-tauri-app/commit/a09256550479e1d555b14d8fb0137454f974c6f2)([#455](https://www.github.com/tauri-apps/create-tauri-app/pull/455)) Adds `preact` and `preact-ts` templates.

### Dependencies

- Upgraded to `create-tauri-app@3.7.0`

## \[3.6.2]

- [`701ab3b`](https://www.github.com/tauri-apps/create-tauri-app/commit/701ab3b5ec1dd62e246d96b33ff20527712612d4)([#447](https://www.github.com/tauri-apps/create-tauri-app/pull/447)) Fix build panic with newer minor versions of `rust-embed`.

## \[3.6.1]

- [`ae9e141`](https://www.github.com/tauri-apps/create-tauri-app/commit/ae9e141c524fd749c57144a75b0ba161779d8222)([#441](https://www.github.com/tauri-apps/create-tauri-app/pull/441)) Fix crash when passing a flavored template like `vue-ts` to the CLI.

## \[3.6.0]

- [`6579f38`](https://www.github.com/tauri-apps/create-tauri-app/commit/6579f38438fb4f1f4d5743dfe1ded55e45a5cbb6)([#437](https://www.github.com/tauri-apps/create-tauri-app/pull/437)) Update tauri to `1.4`

## \[3.5.0]

- [`88944c8`](https://www.github.com/tauri-apps/create-tauri-app/commit/88944c8883d97812f1136ba9905a49b075ba4050)([#430](https://www.github.com/tauri-apps/create-tauri-app/pull/430)) Recursively clean the target directory if bootstrapping a template into a non-empty directory.
- [`7d04484`](https://www.github.com/tauri-apps/create-tauri-app/commit/7d044845293e86c56103b068d32c2baa88bcaf64)([#421](https://www.github.com/tauri-apps/create-tauri-app/pull/421)) Print info to install `tauri-cli@2.0.0-alpha` if it wasn't detected and `--alpha` flag was passed.
- [`9aeaca7`](https://www.github.com/tauri-apps/create-tauri-app/commit/9aeaca7aaf39f0b090ae09905f2890f54a8caff9)([#424](https://www.github.com/tauri-apps/create-tauri-app/pull/424)) Update `--alpha` templates for `tauri@2.0.0-alpha.9`
- [`9aeaca7`](https://www.github.com/tauri-apps/create-tauri-app/commit/9aeaca7aaf39f0b090ae09905f2890f54a8caff9)([#424](https://www.github.com/tauri-apps/create-tauri-app/pull/424)) Update `vue`, `vue-ts`, `vanilla`, `vanilla-ts`, `solid`, `solid-ts`, `svelte`, and `svelte-ts` to use `<form>`

## \[3.4.0]

- Disable analytics for Angular template
  - [3949533](https://www.github.com/tauri-apps/create-tauri-app/commit/3949533d8091752a96b3b9b9f67dd44894c3c45e) fix(templates/angular): add missing `--` for npm, closes [#411](https://www.github.com/tauri-apps/create-tauri-app/pull/411) ([#412](https://www.github.com/tauri-apps/create-tauri-app/pull/412)) on 2023-05-03
- Fix `beforeDevCommand` missing `--` for npm + Angular template
  - [3949533](https://www.github.com/tauri-apps/create-tauri-app/commit/3949533d8091752a96b3b9b9f67dd44894c3c45e) fix(templates/angular): add missing `--` for npm, closes [#411](https://www.github.com/tauri-apps/create-tauri-app/pull/411) ([#412](https://www.github.com/tauri-apps/create-tauri-app/pull/412)) on 2023-05-03
- Update tauri dependencies for all templates
  - [7b992e5](https://www.github.com/tauri-apps/create-tauri-app/commit/7b992e544c6824dbf29d0fab38488e3b516142b1) feat: show missing deps after template bootstrap ([#367](https://www.github.com/tauri-apps/create-tauri-app/pull/367)) on 2023-03-13
  - [093a0d9](https://www.github.com/tauri-apps/create-tauri-app/commit/093a0d9a9cb25b5cd33b0a6c7b5ecfd961699db7) apply version updates ([#369](https://www.github.com/tauri-apps/create-tauri-app/pull/369)) on 2023-03-15
  - [9f0b36e](https://www.github.com/tauri-apps/create-tauri-app/commit/9f0b36e7eaf39d7bcafd79e4901eae2702c9f6db) chore(deps): update solid, vite and tauri deps ([#391](https://www.github.com/tauri-apps/create-tauri-app/pull/391)) on 2023-04-04
  - [9e6a5b0](https://www.github.com/tauri-apps/create-tauri-app/commit/9e6a5b0522c85e561740791a6b3b4fdfb68ea8d4) Apply Version Updates From Current Changes ([#392](https://www.github.com/tauri-apps/create-tauri-app/pull/392)) on 2023-04-04
  - [ecc0676](https://www.github.com/tauri-apps/create-tauri-app/commit/ecc06767d2319c1dc20b3d0aa4791fbefc959b36) chore(deps): update tauri deps for templates ([#414](https://www.github.com/tauri-apps/create-tauri-app/pull/414)) on 2023-05-03

## \[3.3.6]

- Add missing `tauri` script in package.json for Angular template.
  - [b81a89c](https://www.github.com/tauri-apps/create-tauri-app/commit/b81a89c30c7f73e8adaa21c0ffff22c2c25d87b6) fix: add missing tauri script for angular template, closes [#404](https://www.github.com/tauri-apps/create-tauri-app/pull/404) on 2023-04-14
- Update tauri dependencies for all templates.
  - [61aee5a](https://www.github.com/tauri-apps/create-tauri-app/commit/61aee5a052a41092c96d837b14197e7b1a84050f) chore(deps): update tchore(deps): update tauri deps in all templatesauri deps in all templates on 2023-04-16

## \[3.3.5]

- Update `tauri` deps for all templates
  - [2da95ed](https://www.github.com/tauri-apps/create-tauri-app/commit/2da95ed48fc2c44d6b73d67206ceaa1963217648) chore(deps): update tauri to latest version, closes [#229](https://www.github.com/tauri-apps/create-tauri-app/pull/229) on 2022-11-18
  - [2c0f09f](https://www.github.com/tauri-apps/create-tauri-app/commit/2c0f09ff1c99d4300bdc6fbdbb1cb7605dbaff7a) Apply Version Updates From Current Changes ([#230](https://www.github.com/tauri-apps/create-tauri-app/pull/230)) on 2022-11-18
  - [6cdeebd](https://www.github.com/tauri-apps/create-tauri-app/commit/6cdeebde405052261d3baa755f04c35a3c1cfcea) chore: add change file on 2023-04-13

## \[3.3.4]

- Correct the usage of signal getter in Solid template
  - [8c2e92b](https://www.github.com/tauri-apps/create-tauri-app/commit/8c2e92ba232d5307052c6b6a8d6ba8112f9e0f4d) fix(solid.js): update greetMsg signal getter ([#397](https://www.github.com/tauri-apps/create-tauri-app/pull/397)) on 2023-04-05

## \[3.3.3]

- Fix publishing standalone binaries to GitHub releases.
  - [c248817](https://www.github.com/tauri-apps/create-tauri-app/commit/c248817f21c8259447a695dc8b8432c747e9c2b4) ci: update to ubuntu-latest ([#395](https://www.github.com/tauri-apps/create-tauri-app/pull/395)) on 2023-04-04
  - [ce35f27](https://www.github.com/tauri-apps/create-tauri-app/commit/ce35f271a66442532912367a2e3c0e8fbfe7b173) chore: update chang file (dummy commit to run CI) on 2023-04-04

## \[3.3.2]

- Fix publishing arm64 modules to npm
  - [b8a6cda](https://www.github.com/tauri-apps/create-tauri-app/commit/b8a6cda8b86e089913c7041c72a77f82b2583e83) fix(napi): add missing windows arm64 files on 2023-04-04

## \[3.3.1]

- Update `solid-js`, `vite` and `tauri` dependencies.
  - [7b992e5](https://www.github.com/tauri-apps/create-tauri-app/commit/7b992e544c6824dbf29d0fab38488e3b516142b1) feat: show missing deps after template bootstrap ([#367](https://www.github.com/tauri-apps/create-tauri-app/pull/367)) on 2023-03-13
  - [093a0d9](https://www.github.com/tauri-apps/create-tauri-app/commit/093a0d9a9cb25b5cd33b0a6c7b5ecfd961699db7) apply version updates ([#369](https://www.github.com/tauri-apps/create-tauri-app/pull/369)) on 2023-03-15
  - [9f0b36e](https://www.github.com/tauri-apps/create-tauri-app/commit/9f0b36e7eaf39d7bcafd79e4901eae2702c9f6db) chore(deps): update solid, vite and tauri deps ([#391](https://www.github.com/tauri-apps/create-tauri-app/pull/391)) on 2023-04-04

## \[3.3.0]

- Release `aarch64` binaries and node modules.
  - [a5e4694](https://www.github.com/tauri-apps/create-tauri-app/commit/a5e4694e94b3f51e07805c94414eb86c5e95ba87) feat: build cli for windows aarch64 ([#386](https://www.github.com/tauri-apps/create-tauri-app/pull/386)) on 2023-03-27
- Update `@tauri-apps/cli` verstion for `--alpha` templates.
  - [e51b085](https://www.github.com/tauri-apps/create-tauri-app/commit/e51b085a0ac213e4445981fc25835b006766087f) chore: update `@tauri-apps/cli` on 2023-03-26

## \[3.2.1]

- Fix missing hover styles for sycamore logo.
  - [763ef80](https://www.github.com/tauri-apps/create-tauri-app/commit/763ef8081a6760ed011b54a88eacd79d8ff6b142) style.css -> styles.css ([#376](https://www.github.com/tauri-apps/create-tauri-app/pull/376)) on 2023-03-19
- Update tauri dependencies for all `--alpha` templates.
  - [3d783aa](https://www.github.com/tauri-apps/create-tauri-app/commit/3d783aabcf4f93222634e5f0fbc9f0a3a8c613af) feat: update tauri deps for alpha templates ([#379](https://www.github.com/tauri-apps/create-tauri-app/pull/379)) on 2023-03-20

## \[3.2.0]

- Fix `--alpha` templates that are generated without mobile support.
  - [d2c5f1d](https://www.github.com/tauri-apps/create-tauri-app/commit/d2c5f1d12cb0cbedb2247765b053048cb90da60d) fix: remove `[lib]` section in 2.0-alpha tempaltes ([#374](https://www.github.com/tauri-apps/create-tauri-app/pull/374)) on 2023-03-14
- Show a table of missing dependencies with installation instructions.
  - [7b992e5](https://www.github.com/tauri-apps/create-tauri-app/commit/7b992e544c6824dbf29d0fab38488e3b516142b1) feat: show missing deps after template bootstrap ([#367](https://www.github.com/tauri-apps/create-tauri-app/pull/367)) on 2023-03-13
- Bump MSRV to 1.59
  - [7c3231c](https://www.github.com/tauri-apps/create-tauri-app/commit/7c3231c08505dfaf139f498b6ca2241c40006c8f) feat: strip debug symbols from binaries ([#370](https://www.github.com/tauri-apps/create-tauri-app/pull/370)) on 2023-03-14

## \[3.1.2]

- Fix unwanted refresh when clicking on the greet button in `leptos` template
  - Bumped due to a bump in create-tauri-app.
  - [c60e21e](https://www.github.com/tauri-apps/create-tauri-app/commit/c60e21e5fd9d14a5d915c58b19dbe6058baa1ddd) 🔧 Fix(Fragments/Leptos): Fix unwanted refresh and update to Leptos v0.2 ([#362](https://www.github.com/tauri-apps/create-tauri-app/pull/362)) on 2023-03-08
- Update Leptos to v0.2
  - Bumped due to a bump in create-tauri-app.
  - [c60e21e](https://www.github.com/tauri-apps/create-tauri-app/commit/c60e21e5fd9d14a5d915c58b19dbe6058baa1ddd) 🔧 Fix(Fragments/Leptos): Fix unwanted refresh and update to Leptos v0.2 ([#362](https://www.github.com/tauri-apps/create-tauri-app/pull/362)) on 2023-03-08

## \[3.1.1]

- Add missing `"type": "module"` to `package.json` in SolidJs templates.
  - [578b90e](https://www.github.com/tauri-apps/create-tauri-app/commit/578b90e0e7939085652f82b60699f9f685e59b2d) fix(framgents/solid): add missing `"type": "module"`, closes [#358](https://www.github.com/tauri-apps/create-tauri-app/pull/358) ([#360](https://www.github.com/tauri-apps/create-tauri-app/pull/360)) on 2023-03-08

## \[3.1.0]

- Bump MSRV to `1.58`
  - [3d571e7](https://www.github.com/tauri-apps/create-tauri-app/commit/3d571e7790e56f2fc4f9a03afad2065bebacae32) feat: bump MSRV to 1.58 ([#356](https://www.github.com/tauri-apps/create-tauri-app/pull/356)) on 2023-03-06
  - [0eee8dc](https://www.github.com/tauri-apps/create-tauri-app/commit/0eee8dc9b70f4067f1364f03ea69491aa720c6d3) fix(fragments/vanilla): use correct styles import on 2023-03-07
- Fix styles import in `vanilla` template
  - [0eee8dc](https://www.github.com/tauri-apps/create-tauri-app/commit/0eee8dc9b70f4067f1364f03ea69491aa720c6d3) fix(fragments/vanilla): use correct styles import on 2023-03-07

## \[3.0.3]

- Fix wrong `package.json` in `solid-ts` template.
  - [709ba83](https://www.github.com/tauri-apps/create-tauri-app/commit/709ba8398f253b6f490c56e1b9e2d41269870db1) Fix the template fragment of solid-ts ([#351](https://www.github.com/tauri-apps/create-tauri-app/pull/351)) on 2023-03-04

## \[3.0.2]

- Fix missing Javascript import in `vanilla` template
  - [918e933](https://www.github.com/tauri-apps/create-tauri-app/commit/918e93356a534b89c33b01403036c37720d7aff4) fix(fragment/vanilla): fix missing js import ([#347](https://www.github.com/tauri-apps/create-tauri-app/pull/347)) on 2023-03-02

## \[3.0.1]

- Allow passing arguments though `CTA_ARGS` for powershell script `$env:CTA_ARGS="--template svelte --manager pnpm";iwr -useb https://create.tauri.app/ps | iex`
  - [e950d08](https://www.github.com/tauri-apps/create-tauri-app/commit/e950d088d2a4fa20594926f60550eb2236f8ac67) feat: allow passing args to powershell scripts ([#345](https://www.github.com/tauri-apps/create-tauri-app/pull/345)) on 2023-02-28

## \[3.0.0]

- Fix panic when creating a template in the current directory.
  - [e4d11e3](https://www.github.com/tauri-apps/create-tauri-app/commit/e4d11e3c6db45b41bb76d27f5b4990d3a7dc5b05) fix: clean current dir by removing its files, closes [#339](https://www.github.com/tauri-apps/create-tauri-app/pull/339) ([#342](https://www.github.com/tauri-apps/create-tauri-app/pull/342)) on 2023-02-25
- **Breaking Change** Removed `next`, `next-ts`, `preact`, `preact-ts`, `clojurescript`, `svelte-kit`, `svelte-kit-ts` templates.
  - [cea09e2](https://www.github.com/tauri-apps/create-tauri-app/commit/cea09e2dbee7b8bdbf4228253503a2ff9e6d0b40) feat: create-tauri-app@3 ([#284](https://www.github.com/tauri-apps/create-tauri-app/pull/284)) on 2023-02-15
  - [902527b](https://www.github.com/tauri-apps/create-tauri-app/commit/902527bf4210acd49376f929d625ae74b8149f7c) chore: update changelog file on 2023-02-27
- Add `leptos` rust template.
  - [cea09e2](https://www.github.com/tauri-apps/create-tauri-app/commit/cea09e2dbee7b8bdbf4228253503a2ff9e6d0b40) feat: create-tauri-app@3 ([#284](https://www.github.com/tauri-apps/create-tauri-app/pull/284)) on 2023-02-15

## \[2.8.0]

- Fix generated output to use HTML forms properly.
  - [4beabe2](https://www.github.com/tauri-apps/create-tauri-app/commit/4beabe2e15b41b18535d9a56cdba1f9e509e459c) feat: use forms in templates ([#331](https://www.github.com/tauri-apps/create-tauri-app/pull/331)) on 2023-02-13
- Add `sycamore` template for `cargo` package manager.
  - [fbe297d](https://www.github.com/tauri-apps/create-tauri-app/commit/fbe297dcd3bd3a131eaebf21293979dfad38a008) chore: change file for sycamore on 2023-02-13

## \[2.7.10]

- Update `@svelte/kit` to new major version `1.x` and update related dependencies. Also, fixed the `check` and `check:watch` scripts in `svelte-kit-ts` template.
  - Bumped due to a bump in create-tauri-app.
  - [afe0b5e](https://www.github.com/tauri-apps/create-tauri-app/commit/afe0b5edfe08e0113f171676ed4b92474537e18a) Update svelte kit ([#323](https://www.github.com/tauri-apps/create-tauri-app/pull/323)) on 2023-02-06

## \[2.7.9]

- Fix incorrect allowlist in `react-ts` template by deleting the leftover `tauri.conf.json` file.
  - [f25b374](https://www.github.com/tauri-apps/create-tauri-app/commit/f25b374d822a3e65ad47b821f7af2745f1b6af9e) fix: delete leftover `tauri.conf.json` file in `react-ts` template, fixes [#318](https://www.github.com/tauri-apps/create-tauri-app/pull/318) ([#319](https://www.github.com/tauri-apps/create-tauri-app/pull/319)) on 2023-02-02

## \[2.7.8]

- Fix crashed caused by whitespace in "Project name" by trimming it.
  - [2f5cf5c](https://www.github.com/tauri-apps/create-tauri-app/commit/2f5cf5c99385baae766c266f82dd5c6aa31ab32e) fix(cli): trim project_name whitespace, closes [#308](https://www.github.com/tauri-apps/create-tauri-app/pull/308) on 2023-01-21

## \[2.7.7]

- Wrap the `cd <dir>` instruction in quotes if the project name containts spaces.
  - Bumped due to a bump in create-tauri-app.
  - [a5c6dd9](https://www.github.com/tauri-apps/create-tauri-app/commit/a5c6dd945fc4bdcd12aeb200bde2c35732060c0d) fix: wrap the `cd <dir>` instruction in quotes ([#305](https://www.github.com/tauri-apps/create-tauri-app/pull/305)) on 2023-01-18

## \[2.7.6]

- Update `vite.conf.json` target option to match the latest tauri docs.
  - [3ea6acf](https://www.github.com/tauri-apps/create-tauri-app/commit/3ea6acf54b1f9517043121b0ce0bacf8910f175c) feat(templates): update vite conf to match latest tauri docs on 2023-01-02

## \[2.7.5]

- Add `svelte-process` preprocessor to `svelte-ts` and `svelte-kit-ts` templates by default so typescript can work correctly inside `.svelte` files.
  - Bumped due to a bump in create-tauri-app.
  - [b9034ec](https://www.github.com/tauri-apps/create-tauri-app/commit/b9034eca8f8ed6041fe92f476fee3e5c77553e79) fix: add typescript support in .svelte files ([#277](https://www.github.com/tauri-apps/create-tauri-app/pull/277)) on 2022-12-30

## \[2.7.4]

- Fix paring `svelte-kit` and `svelte-kit-ts` from command line.
  - Bumped due to a bump in create-tauri-app.
  - [11c26e1](https://www.github.com/tauri-apps/create-tauri-app/commit/11c26e1fc331aa3cf5fa9ba94617bd5be5686ec5) fix(cli): parse svelte-kit templates correctly, closes [#270](https://www.github.com/tauri-apps/create-tauri-app/pull/270) ([#271](https://www.github.com/tauri-apps/create-tauri-app/pull/271)) on 2022-12-21

## \[2.7.3]

- Add `shell-open` cargo feature to match the allowlist in the generated project.
  - [fb6e439](https://www.github.com/tauri-apps/create-tauri-app/commit/fb6e4392b57334f68f41260fbdceb59761f31f5f) fix: add `shell-open` cargo feature to base ([#264](https://www.github.com/tauri-apps/create-tauri-app/pull/264)) on 2022-12-15

## \[2.7.2]

- Update `vanilla-ts`, `vue`, `vue-ts`, `solid`, `solid-ts`, `svelte`, `svelte-ts`, `react`, `react-ts` templates to use `vite@4.0.0`.
  - [c9e5fd5](https://www.github.com/tauri-apps/create-tauri-app/commit/c9e5fd5488b2f5cb53b4ff4856e2162d762968b9) chore(deps): update to `vite@4` ([#257](https://www.github.com/tauri-apps/create-tauri-app/pull/257)) on 2022-12-11

## \[2.7.1]

- Enable `allowlist > shell > open` in the generated project's `tauri.conf.json` so clicking to open external links would work.
  - [bab7f59](https://www.github.com/tauri-apps/create-tauri-app/commit/bab7f5952ce70f7294e4c2551e3ae8ca755851a1) fix(fragments/base): enable `allowlist>shell>open` ([#254](https://www.github.com/tauri-apps/create-tauri-app/pull/254)) on 2022-12-09

## \[2.7.0]

- Disable allowlist by default for all templates.
  - [78e7cab](https://www.github.com/tauri-apps/create-tauri-app/commit/78e7cab4c3e16881ad354672a9c553800edd1b22) feat: disable allowlist by default ([#246](https://www.github.com/tauri-apps/create-tauri-app/pull/246)) on 2022-12-05
  - [4a26bd8](https://www.github.com/tauri-apps/create-tauri-app/commit/4a26bd8e9bc36ef8318bf6c49ee860ac49a9dd70) chore: update bump to minor on 2022-12-05
- Disable SSR by default in `svelte-kit` and `svelte-kit-ts` templates.
  - [79da92d](https://www.github.com/tauri-apps/create-tauri-app/commit/79da92d38acd25106d6054c0442c11e40499d217) Fix: disable SSR by default in SvelteKit templates (fix for "feat: Sveltekit templates [#200](https://www.github.com/tauri-apps/create-tauri-app/pull/200)") ([#241](https://www.github.com/tauri-apps/create-tauri-app/pull/241)) on 2022-11-24
- Update `yew` template to use `yew@0.20`
  - [386e870](https://www.github.com/tauri-apps/create-tauri-app/commit/386e8701873ffbdd6f097eb31ab335d3f8d45d64) chore: add changefile on 2022-12-05

## \[2.6.5]

- Revert back to `rust-embed` 6.4.
  - Bumped due to a bump in create-tauri-app.
  - [ea586ca](https://www.github.com/tauri-apps/create-tauri-app/commit/ea586caf8cf2149f7fc722e7e6dcb416a7e5c295) fix: revert `rust-embed` to 6.4 on 2022-11-23

## \[2.6.4]

- Fix a few panics introduced by some changes in latest `rust-embed` crate versions.
  - Bumped due to a bump in create-tauri-app.
  - [8034aac](https://www.github.com/tauri-apps/create-tauri-app/commit/8034aacd075dcf54c6d462dd465af66e14012fec) fix: fix few panics with latest `rust-embed`, closes [#236](https://www.github.com/tauri-apps/create-tauri-app/pull/236) ([#237](https://www.github.com/tauri-apps/create-tauri-app/pull/237)) on 2022-11-23

## \[2.6.3]

- Update `tauri` to latest version `1.2`.
  - Bumped due to a bump in create-tauri-app.
  - [2da95ed](https://www.github.com/tauri-apps/create-tauri-app/commit/2da95ed48fc2c44d6b73d67206ceaa1963217648) chore(deps): update tauri to latest version, closes [#229](https://www.github.com/tauri-apps/create-tauri-app/pull/229) on 2022-11-18

## \[2.6.2]

- Change `vanilla` and `vanilla-ts` templates to attach the click handler through Javascript, instead of assigining to the `window` object.
  - [f5f8628](https://www.github.com/tauri-apps/create-tauri-app/commit/f5f862869c3eb8805dc052dc8594714f3a923831) refactor: remove vanilla templates assignment to window object ([#212](https://www.github.com/tauri-apps/create-tauri-app/pull/212)) on 2022-10-11

## \[2.6.1]

- Only prompt for supported package managers when using `--template` cli option.
  - [8ba553c](https://www.github.com/tauri-apps/create-tauri-app/commit/8ba553cda5a4c765a4d2e7be2cadb291c2bcbc75) feat: only prompt for pkg managers supported by template, closes [#208](https://www.github.com/tauri-apps/create-tauri-app/pull/208) ([#209](https://www.github.com/tauri-apps/create-tauri-app/pull/209)) on 2022-10-10

## \[2.6.0]

- Fix crash when nodejs binary has the version in its name, for example `node18`
  - [09c0ed6](https://www.github.com/tauri-apps/create-tauri-app/commit/09c0ed6bfba893fbfb1ac304720b72716f7137d1) fix(cli/node): fix invoking the node cli from a shim, closes [#193](https://www.github.com/tauri-apps/create-tauri-app/pull/193) ([#207](https://www.github.com/tauri-apps/create-tauri-app/pull/207)) on 2022-10-04
- Add `svelte-kit` and `svelte-kit-ts` template.
  - [0b09cc1](https://www.github.com/tauri-apps/create-tauri-app/commit/0b09cc167784b4400c2078e61f1b7bbe45ba54a3) feat: add `Sveltekit` templates ([#200](https://www.github.com/tauri-apps/create-tauri-app/pull/200)) on 2022-10-04
- Use `import` in vanilla-ts instead of the global Tauri object.
  - [dcca18c](https://www.github.com/tauri-apps/create-tauri-app/commit/dcca18c2de3bd14304bee849015c33880304d647) fix: Use `import` in vanilla-ts fragment. ([#198](https://www.github.com/tauri-apps/create-tauri-app/pull/198)) on 2022-10-01
  - [ee4469b](https://www.github.com/tauri-apps/create-tauri-app/commit/ee4469b97a05ab66ccbb28807eb2e53f333b5d48) chore: fix changefile on 2022-10-04

## \[2.5.0]

- Add `clojurescript` template.
  - [6ca747e](https://www.github.com/tauri-apps/create-tauri-app/commit/6ca747eb6ff5eaded5bb073e5f1dd551a843b19b) feat(cli/templates) add clojurescript ([#185](https://www.github.com/tauri-apps/create-tauri-app/pull/185)) on 2022-09-22

## \[2.4.2]

- Replace deprecated functions in `yew` template.
  - Bumped due to a bump in create-tauri-app.
  - [16b0210](https://www.github.com/tauri-apps/create-tauri-app/commit/16b02100ced05ff46235bb64ff276fa834007eb5) fix(cli/fragments/yew): replace deprecated code ([#182](https://www.github.com/tauri-apps/create-tauri-app/pull/182)) on 2022-09-19

## \[2.4.1]

- Update `tauri` dependencies in templates to `1.1`
  - Bumped due to a bump in create-tauri-app.
  - [84e0ba0](https://www.github.com/tauri-apps/create-tauri-app/commit/84e0ba03a7c4f764398b3bb4eef4f0320a24b63c) chore(deps): update tauri to 1.1 in templates. close [#179](https://www.github.com/tauri-apps/create-tauri-app/pull/179) on 2022-09-16

## \[2.4.0]

- Add `angular` template
  - [459228f](https://www.github.com/tauri-apps/create-tauri-app/commit/459228fd06b6bc41624c1274555dc0c1852d3ac8) Add Angular template ([#167](https://www.github.com/tauri-apps/create-tauri-app/pull/167)) on 2022-09-11
  - [27f6568](https://www.github.com/tauri-apps/create-tauri-app/commit/27f65687566486fcbfef4509898bceb9db780149) chore: typo on 2022-09-11
  - [8b43ad1](https://www.github.com/tauri-apps/create-tauri-app/commit/8b43ad1bb621ade4d89ae8e52f560eeb68558955) Update angular.md on 2022-09-11

## \[2.3.1]

- Fix building in `next` and `next-ts` templates by removing the `experimental` option from `next.config.js` since `images.unoptimized` is now stable.
  - [975a851](https://www.github.com/tauri-apps/create-tauri-app/commit/975a851818975599ad0e7ca145f7f13d8b3f2875) fix(cli/fragment-next): update to stable next.config.js ([#168](https://www.github.com/tauri-apps/create-tauri-app/pull/168)) on 2022-09-10
  - [7ee3aaa](https://www.github.com/tauri-apps/create-tauri-app/commit/7ee3aaa4dd3de7296ac28319f8c7b5b5b08b995e) fix: allow building on msrv 1.57 ([#170](https://www.github.com/tauri-apps/create-tauri-app/pull/170)) on 2022-09-10

## \[2.3.0]

- Add `vanilla-ts` templates.
  - [8799cdf](https://www.github.com/tauri-apps/create-tauri-app/commit/8799cdf010f94ee880dc18a04d520a7496015d49) feat(cli/templates) add `vanilla-ts` template, closes [#155](https://www.github.com/tauri-apps/create-tauri-app/pull/155) ([#156](https://www.github.com/tauri-apps/create-tauri-app/pull/156)) on 2022-08-31
- Fix yew template triggering trunk rebuilds when tauri files change.
  - [d00f8b5](https://www.github.com/tauri-apps/create-tauri-app/commit/d00f8b57ec6269545dbb81e4e1682eaf51925d55) fix(cli/fragments/yew): ignore `src-tauri` for `trunk serve`, closes [#160](https://www.github.com/tauri-apps/create-tauri-app/pull/160) on 2022-09-05
  - [717ffd7](https://www.github.com/tauri-apps/create-tauri-app/commit/717ffd7b43edd51f9d1cb0552f95d53e8689d51b) chore: adjust changefile on 2022-09-05

## \[2.2.0]

- Add `preact` and `preact-ts` templates.
  - [0f778e2](https://www.github.com/tauri-apps/create-tauri-app/commit/0f778e26d724cb5b8114d58b7a634a4cfdf978e0) feat: add `preact` and `preact-tS` templates ([#145](https://www.github.com/tauri-apps/create-tauri-app/pull/145)) on 2022-08-30
  - [3801c7d](https://www.github.com/tauri-apps/create-tauri-app/commit/3801c7def4faa06ba73a440da842308938869b3d) chore: change preact bump to minor on 2022-08-31

## \[2.1.1]

- Add an optimized macOS icon so that building from a template doesn't cause errors.
  - Bumped due to a bump in create-tauri-app.
  - [a28848c](https://www.github.com/tauri-apps/create-tauri-app/commit/a28848c009c111da367e19be3bc93669f9b8bf6b) fix: add missing `icon.icns` file, closes [#143](https://www.github.com/tauri-apps/create-tauri-app/pull/143) ([#152](https://www.github.com/tauri-apps/create-tauri-app/pull/152)) on 2022-08-30
- Fix solid template IDE type errors by changing `className` to `class`
  - Bumped due to a bump in create-tauri-app.
  - [ea9a90c](https://www.github.com/tauri-apps/create-tauri-app/commit/ea9a90c30385ac3e3ea081ac43c1479e563b3bac) fix(cli/fragments): change solid's `className` to `class`, closes [#144](https://www.github.com/tauri-apps/create-tauri-app/pull/144) ([#150](https://www.github.com/tauri-apps/create-tauri-app/pull/150)) on 2022-08-30
- Fixed yew template "beforeDevCommand" from "trunk build" to "trunk serve". Before when you called "tauri dev" infinite loop will occur waiting for dev server to become available at "http://localhost:1420".
  - Bumped due to a bump in create-tauri-app.
  - [675b091](https://www.github.com/tauri-apps/create-tauri-app/commit/675b091f3033dec0413d9d43329be8c46dd31f9c) fix(cli/fragments): fix yew fragment beforeDevCommand and withGlobalTauri ([#147](https://www.github.com/tauri-apps/create-tauri-app/pull/147)) on 2022-08-30
- Changed "withGlobalTauri" for yew template from "false" to "true" so example frontend can actually "invoke" backend methods
  - Bumped due to a bump in create-tauri-app.
  - [675b091](https://www.github.com/tauri-apps/create-tauri-app/commit/675b091f3033dec0413d9d43329be8c46dd31f9c) fix(cli/fragments): fix yew fragment beforeDevCommand and withGlobalTauri ([#147](https://www.github.com/tauri-apps/create-tauri-app/pull/147)) on 2022-08-30

## \[2.1.0]

- Add `next` and `next-ts` templates
  - [cbe1200](https://www.github.com/tauri-apps/create-tauri-app/commit/cbe1200f72b606d8f100ecc335bb7df4fb49e4b3) feat(cli/templates) add `next` and `next-ts` ([#137](https://www.github.com/tauri-apps/create-tauri-app/pull/137)) on 2022-08-25
  - [4dc7efb](https://www.github.com/tauri-apps/create-tauri-app/commit/4dc7efb85960b75198c5be207b412589486c4360) chore: fix change file bump on 2022-08-25
  - [7ff112e](https://www.github.com/tauri-apps/create-tauri-app/commit/7ff112e6b1866937c38e1c55590ce0eed08c9c77) chore: bump the node cli to minor on 2022-08-27

## \[2.0.5]

- Fix packaging templates when publishing to crates.io
  - Bumped due to a bump in create-tauri-app.
  - [613cfd3](https://www.github.com/tauri-apps/create-tauri-app/commit/613cfd3294046bed51c955d2259894306c3569ea) fix(cli): rename Cargo.toml in fragments to \_Cargo.toml on 2022-08-23

## \[2.0.4]

- Add `@types/node` as a dev dependency to typescript templates.
  - Bumped due to a bump in create-tauri-app.
  - [ff7265c](https://www.github.com/tauri-apps/create-tauri-app/commit/ff7265c8a6e070c45c41a0586f45b3ce291a8121) feat(cli/templates): add `@typs/node` to typescript templates on 2022-08-22
- Fix `solid` and `solid-ts` vite config file.
  - Bumped due to a bump in create-tauri-app.
  - [246ada4](https://www.github.com/tauri-apps/create-tauri-app/commit/246ada459fb1e084d5b5750fbf7811ec38716666) fix(cli/tamplates): fix solid template vite config on 2022-08-22

## \[2.0.3]

- Fix css import in react templates.
  - Bumped due to a bump in create-tauri-app.
  - [a448c5e](https://www.github.com/tauri-apps/create-tauri-app/commit/a448c5e1779c6d0f195693d79427d930fec0915e) fix(cli/tempaltes): fix css imports in react templates on 2022-08-22

## \[2.0.2]

- Fix react-ts template port.
  - Bumped due to a bump in create-tauri-app.
  - [4598b99](https://www.github.com/tauri-apps/create-tauri-app/commit/4598b9951da3275bc03be92d373056e8cc1b8c02) fix(cli/fragments): fix react-ts port on 2022-08-22

## \[2.0.1]

- Fix missing features in yew fragment
  - Bumped due to a bump in create-tauri-app.
  - [71e5449](https://www.github.com/tauri-apps/create-tauri-app/commit/71e544909aa8b6e42bc412f332b917ad9d52fc76) fix: missing features in yew fragment, fixes [#122](https://www.github.com/tauri-apps/create-tauri-app/pull/122) ([#123](https://www.github.com/tauri-apps/create-tauri-app/pull/123)) on 2022-08-22

## \[2.0.0]

- New templates that are customized towards a better experience with Tauri.
  - [6c50fc3](https://www.github.com/tauri-apps/create-tauri-app/commit/6c50fc38019f5284d78873750a3fcdd1d7835931) refactor: rewrite in rust ([#90](https://www.github.com/tauri-apps/create-tauri-app/pull/90)) on 2022-08-22
- Rewrote `create-tauri-app` in rust to make it accessible to all communities and not only Node.js, and now you can use `create-tauri-app` through `npm`, `yarn`, `pnpm`, `cargo` or directly through your shell using `powershell` or `bash`. Check out the [README.md](https://github.com/tauri-apps/create-tauri-app#create-tauri-app) for different ways to use it.
  - [6c50fc3](https://www.github.com/tauri-apps/create-tauri-app/commit/6c50fc38019f5284d78873750a3fcdd1d7835931) refactor: rewrite in rust ([#90](https://www.github.com/tauri-apps/create-tauri-app/pull/90)) on 2022-08-22

## \[2.0.0-beta.1]

- Add missing `lang="ts"` for `App.vue` in `vue-ts` template
  - Bumped due to a bump in create-tauri-app.
  - [4132eb8](https://www.github.com/tauri-apps/create-tauri-app/commit/4132eb8b946b62cf93fa237a5ca95fb13a2b1fac) fix vue-ts template on 2022-08-09

## \[2.0.0-beta.0]

- - Beta
- Changed all templates server port to 1420
- Updated styles of all templates
- [d8c1abc](https://www.github.com/tauri-apps/create-tauri-app/commit/d8c1abccbe256e4dc0e07d90d7bd6ee43e48fcf6) prepare for beta on 2022-08-09
- [600566a](https://www.github.com/tauri-apps/create-tauri-app/commit/600566a79a8e108330033795a36eeea095cfc8ce) fix covector bump on 2022-08-09

## \[2.0.0-alpha.11]

- 2.0.0-alpha.11
  - Bumped due to a bump in create-tauri-app.
  - [486857c](https://www.github.com/tauri-apps/create-tauri-app/commit/486857c0a9065abac2a937b0942012c07f077176) alpha.11 on 2022-08-07

## \[2.0.0-alpha.10]

- 2.0.0-alpha.10
  - Bumped due to a bump in create-tauri-app.
  - [a899bf7](https://www.github.com/tauri-apps/create-tauri-app/commit/a899bf7a8d50b926f960bbee9cde1b737671d003) alpha.10 on 2022-08-06

## \[2.0.0-alpha.9]

- Add missing shebang for node cli
  - Bumped due to a bump in create-tauri-app.
  - [4d16ce2](https://www.github.com/tauri-apps/create-tauri-app/commit/4d16ce204e76086357963c5e595779b82493960d) add missing shebang on 2022-08-06

## \[2.0.0-alpha.8]

- Use dimmed white instead of black
  - Bumped due to a bump in create-tauri-app.
  - [a0b876d](https://www.github.com/tauri-apps/create-tauri-app/commit/a0b876d3a2333f2b152f8dd7063549f74e24210a) add missing changefile on 2022-08-05

## \[2.0.0-alpha.7]

- 2.0.0-alpha.7
  - Bumped due to a bump in create-tauri-app.
  - [2237271](https://www.github.com/tauri-apps/create-tauri-app/commit/2237271f724cbc468a0f2e4c46ce42293b4d6a8b) alpha.7 on 2022-08-03

## \[2.0.0-alpha.6]

- 2.0.0-alpha.6
  - Bumped due to a bump in create-tauri-app.
  - [9667bfa](https://www.github.com/tauri-apps/create-tauri-app/commit/9667bfa9261f158a01b8202a7fa429a9bd559d22) alpha.6 on 2022-08-03

## \[2.0.0-alpha.5]

- 2.0.0-alpha.5
  - Bumped due to a bump in create-tauri-app.
  - [bc68a1d](https://www.github.com/tauri-apps/create-tauri-app/commit/bc68a1d1770239e21d0fee8e03ff1d20e96c970b) alpha.5 on 2022-08-03

## \[2.0.0-alpha.4]

- 2.0.0-alpha.4
  - Bumped due to a bump in create-tauri-app.
  - [f3f6007](https://www.github.com/tauri-apps/create-tauri-app/commit/f3f60072ada609c6563151ed2a522bd6fed6ad47) alpha.4 on 2022-08-03

## \[2.0.0-alpha.3]

- 2.0.0-alpha.3
  - Bumped due to a bump in create-tauri-app.
  - [b542242](https://www.github.com/tauri-apps/create-tauri-app/commit/b54224230a67e00cb5ec4622fc02963047ce6960) alpha.3 on 2022-08-03

## \[2.0.0-alpha.2]

- 2.0.0-alpha.2
  - Bumped due to a bump in create-tauri-app.
  - [83b11a0](https://www.github.com/tauri-apps/create-tauri-app/commit/83b11a043475881100fda612e13506c74d4477da) alpha.2 on 2022-08-03

## \[2.0.0-alpha.1]

- 2.0.0-alpha.1
  - Bumped due to a bump in create-tauri-app.
  - [82199ba](https://www.github.com/tauri-apps/create-tauri-app/commit/82199baf6de47ab9fe48ebc13fb481ba910adaf5) split publish workflow on 2022-08-03

## \[2.0.0-alpha.0]

- Rewrite in rust.
  - [60576c3](https://www.github.com/tauri-apps/create-tauri-app/commit/60576c3e247d50e859d602dc8fe733bb8cf2ca3f) changefile on 2022-07-27
  - [07e7902](https://www.github.com/tauri-apps/create-tauri-app/commit/07e7902b565b8fe9cefc860d647ce109221ee5df) fix npm bump on 2022-07-31

## \[1.0.2]

- Update the vite recipe to use port 5173, the new default in vite@v3.
  - [ef82e5e](https://www.github.com/tauri-apps/create-tauri-app/commit/ef82e5e749e99191af04728b402e05562be11cd1) fix: Use vite's new default port 5173 in devPath ([#81](https://www.github.com/tauri-apps/create-tauri-app/pull/81)) on 2022-07-13

## \[1.0.1]

- Update tauri.studio links to tauri.app
  - [fe5d9ca](https://www.github.com/tauri-apps/create-tauri-app/commit/fe5d9caecf0988beda5f1f9bf371bb467fc4b717) Update website link ([#75](https://www.github.com/tauri-apps/create-tauri-app/pull/75)) on 2022-06-19

## \[1.0.0]

- Stable v1.0.0
  - [69968b6](https://www.github.com/tauri-apps/create-tauri-app/commit/69968b615280443ccbdd7271d2f00ace650de3c4) covector: prepare for stable v1.0.0 on 2022-06-19

## \[1.0.0-rc.5]

- Fix crash when using yarn because of using `@latest` tag.
  - [761d0ac](https://www.github.com/tauri-apps/create-tauri-app/commit/761d0acaf1405aa25761ab8e118c2505b8896801) fix: don't use `@latest` tag with yarn, closes [#62](https://www.github.com/tauri-apps/create-tauri-app/pull/62) on 2022-06-05

## \[1.0.0-rc.4]

- `create-tauri-app` will no longer install or prompt for `@tauri-apps/api` package in the `VanillaJs` recipe.
  - [0c37e73](https://www.github.com/tauri-apps/create-tauri-app/commit/0c37e7382fabce0cff716069582309197ac60461) fix: don't install `@tauri-apps/api` for vanillajs on 2022-04-30
- Update vulnerable packages: `ejs` and `minimist`.
  - [3128215](https://www.github.com/tauri-apps/create-tauri-app/commit/31282158760b2bf8692b92b0d493fefe0a359fa5) fix: update vulnerable packages on 2022-06-04
- - More Solid templates have been added!
  - [a77df69](https://www.github.com/tauri-apps/create-tauri-app/commit/a77df69f6c6745121d8cd29f62cf30ace32e88c3) feat(solid): 📦 add more solid templates ([#42](https://www.github.com/tauri-apps/create-tauri-app/pull/42)) on 2022-05-20
- Use `@latest` tag when creating the recipe so `npm` wouldn't load from cache.
  - [d462775](https://www.github.com/tauri-apps/create-tauri-app/commit/d462775742db185d6d595c57b44dde8b5e6719e0) fix: use `@latest` tag ([#58](https://www.github.com/tauri-apps/create-tauri-app/pull/58)) on 2022-06-04

## \[1.0.0-rc.3]

- - `create-tauri-app` handles different package managers usage better now.
  - [a073dbc](https://www.github.com/tauri-apps/create-tauri-app/commit/a073dbc51d14530be75152f314480e2cc2496181) refactor: version aware package manager usage, fix [#4](https://www.github.com/tauri-apps/create-tauri-app/pull/4), fix [#2](https://www.github.com/tauri-apps/create-tauri-app/pull/2), fix [#6](https://www.github.com/tauri-apps/create-tauri-app/pull/6) ([#5](https://www.github.com/tauri-apps/create-tauri-app/pull/5)) on 2022-03-03

## \[1.0.0-rc.2]

- Fix crash when using `create-react-app` recipe.
  - [c0103604](https://www.github.com/tauri-apps/tauri/commit/c01036043dcec52ba360fc38268b8b4ae4c470a1) fix(cta): fix cli install with other deps, fixes [#3417](https://www.github.com/tauri-apps/tauri/pull/3417) ([#3420](https://www.github.com/tauri-apps/tauri/pull/3420)) on 2022-02-12

## \[1.0.0-rc.1]

- Fix `create-tauri-app` failing to bootstrap projects.
  - [536c0cd7](https://www.github.com/tauri-apps/tauri/commit/536c0cd7d40b838765016e1554901dea8720d9af) fix(cta): filter out empty args, fixes [#3393](https://www.github.com/tauri-apps/tauri/pull/3393) ([#3396](https://www.github.com/tauri-apps/tauri/pull/3396)) on 2022-02-11

## \[1.0.0-rc.0]

- Add empty description to Cargo.toml in dominator recipe.
  - [97edb3ac](https://www.github.com/tauri-apps/tauri/commit/97edb3ac49d59c5c95ad8486c17b3c333f4f86a2) Fix: [#2508](https://www.github.com/tauri-apps/tauri/pull/2508). Update dominator recipe description. ([#2514](https://www.github.com/tauri-apps/tauri/pull/2514)) on 2021-08-24
- `create-tauri-app` should now be fully compatiable with CI environments.
  - [f5e77ff4](https://www.github.com/tauri-apps/tauri/commit/f5e77ff48f00e14476f95cce257d091377ba987c) refactor(cta): use `commander` instead of `minimst` ([#2551](https://www.github.com/tauri-apps/tauri/pull/2551)) on 2022-01-01
- Stop react recipe from opening in browser by default.
  - [ea51504e](https://www.github.com/tauri-apps/tauri/commit/ea51504e3a57eedc28e40573fbcc899b8a5c358c) fix(cta): stop react recipe from opening in browser, closes [#2793](https://www.github.com/tauri-apps/tauri/pull/2793) ([#2988](https://www.github.com/tauri-apps/tauri/pull/2988)) on 2021-11-30
- Add SolidJS recipe using the official template.
  - [71ea86a4](https://www.github.com/tauri-apps/tauri/commit/71ea86a443f2585fa98edd79f2361bd85b380f0c) feat(cta): add SolidJS recipe ([#2619](https://www.github.com/tauri-apps/tauri/pull/2619)) on 2021-09-22

## \[1.0.0-beta.4]

- [`pnpm`](https://pnpm.io) package manager is now officially supported, either run `pnpx create-tauri-app` or explicitly specifiy it `npx create-tauri-app --manager pnpm`.
  - [235e0f67](https://www.github.com/tauri-apps/tauri/commit/235e0f6785b87dc83cc6ebb6f5b022a82fa18eec) feat(CTA): add official support for `pnpm` package manager ([#2348](https://www.github.com/tauri-apps/tauri/pull/2348)) on 2021-08-06
- `create-tauri-app` will prompt users to install `@tauri-apps/api` npm package.
  - [c0f42ad0](https://www.github.com/tauri-apps/tauri/commit/c0f42ad0e3d30623b83cfcd692eb1bcb4c4391a2) feat(cta): prompt users to install `@tauri-apps/api` package ([#2251](https://www.github.com/tauri-apps/tauri/pull/2251)) on 2021-07-29
- Add Svelte recipe using the official template.
  - [151c3157](https://www.github.com/tauri-apps/tauri/commit/151c3157bef28c267592ebdf717e4ff66a5b27e1) Add svelte recipe to create-tauri-app ([#2276](https://www.github.com/tauri-apps/tauri/pull/2276)) ([#2279](https://www.github.com/tauri-apps/tauri/pull/2279)) on 2021-07-22
- Adjust check for `dev` mode and switch CTA test to a script runner. The script gives us more control and better output into any failures.
  - [c410e034](https://www.github.com/tauri-apps/tauri/commit/c410e034f74d0624c8465b1f30bb7af58eb98b34) convert jest tests to child_process run script ([#2308](https://www.github.com/tauri-apps/tauri/pull/2308)) on 2021-08-08
- Update vite recipe to use the new vite npm [package](https://github.com/vitejs/vite/tree/main/packages/create-vite).
  - [718d9513](https://www.github.com/tauri-apps/tauri/commit/718d9513ce8013594a21c7fedb2dcb3dcd7bbad8) refactor(cta): update `vite` recipe to use their new npm package ([#2220](https://www.github.com/tauri-apps/tauri/pull/2220)) on 2021-07-29

## \[1.0.0-beta.3]

- Added Angular CLI recipe.
  - [489fad55](https://www.github.com/tauri-apps/tauri/commit/489fad55242b3489c7c551fdfdd031ebad2d9b9c) Angular create tauri app \[[#1934](https://www.github.com/tauri-apps/tauri/pull/1934)] ([#2203](https://www.github.com/tauri-apps/tauri/pull/2203)) on 2021-07-14

## \[1.0.0-beta.2]

- Fixes the `beforeDevCommand` on vite recipe.
  - [3c21ddc7](https://www.github.com/tauri-apps/tauri/commit/3c21ddc73cd7ab8141b730ceade46fc2dfadd996) fix(cta): use correct `beforeDevCommand` for vite recipe ([#1931](https://www.github.com/tauri-apps/tauri/pull/1931)) on 2021-06-01

## \[1.0.0-beta.1]

- Work around bugs between esbuild and npm by installing directly at the end of the sequence. Also default to using the latest on all of the installs instead of npx's cache.
  - [8a164d0](https://www.github.com/tauri-apps/tauri/commit/8a164d0a1f8eb69bdcec7ae4362d26b2f3c7ff55) fix: CTA cache and vite build ([#1806](https://www.github.com/tauri-apps/tauri/pull/1806)) on 2021-05-12

## \[1.0.0-beta.0]

- Explicitly install deps after a vite recipe.
  - [397b7af](https://www.github.com/tauri-apps/tauri/commit/397b7af395a213bf826aa52398467b7b3352b666) chore: CTA defaults in CI mode ([#1671](https://www.github.com/tauri-apps/tauri/pull/1671)) on 2021-05-05
- Shift everything out of the `bin` and into `.ts` so we can apply Typescript types.
  - [c3acbd6](https://www.github.com/tauri-apps/tauri/commit/c3acbd68ec169188c782cbaf7d100d80b3a4f39a) chore: shift CTA from bin to .ts ([#1651](https://www.github.com/tauri-apps/tauri/pull/1651)) on 2021-04-29
- We setup an e2e type test suite for CTA. It is mostly an internal change, but should help with stability moving forward.
  - [af6411d](https://www.github.com/tauri-apps/tauri/commit/af6411d5f8c9fd1c3d9b4f3c2d79e8f1bd0efbf2) feat: setup testing for CTA ([#1615](https://www.github.com/tauri-apps/tauri/pull/1615)) on 2021-04-27
- Add support for all vite templates
  - [cea3ba9](https://www.github.com/tauri-apps/tauri/commit/cea3ba9f97de9d0181a84ad085a852517bd33a65) feat(cta): add support for all vite templates ([#1670](https://www.github.com/tauri-apps/tauri/pull/1670)) on 2021-05-07
- Add a welcome prompt to let the user know about the process and links to more info including prerequisite setup steps. Also add links to each of the templates to give the user more context what they are getting into.
  - [ea28d01](https://www.github.com/tauri-apps/tauri/commit/ea28d0169168953e11416231e50b08061413a27e) create-tauri-app welcome prompt and recipes links ([#1748](https://www.github.com/tauri-apps/tauri/pull/1748)) on 2021-05-09

## \[1.0.0-beta-rc.4]

- Manually set `tauri` script instead of using `npm set-script` for compatabilty with older npm versions
  - [f708ff8](https://www.github.com/tauri-apps/tauri/commit/f708ff824e7933341536aecb49f6ee35eea621da) fix(CTA): [#1569](https://www.github.com/tauri-apps/tauri/pull/1569), manually set tauri script for compatability with older npm ([#1572](https://www.github.com/tauri-apps/tauri/pull/1572)) on 2021-04-22

## \[1.0.0-beta-rc.3]

- Remove `lodash` dependency and replace with es6 builtins
  - [edab7a6](https://www.github.com/tauri-apps/tauri/commit/edab7a66864d21b51694bf8771d21627b526c2b9) chore(deps): remove lodash from create-tauri-app ([#1532](https://www.github.com/tauri-apps/tauri/pull/1532)) on 2021-04-18
- Remove `tauri` dependency from vanilla recipe
  - [3998046](https://www.github.com/tauri-apps/tauri/commit/399804648924139c6240351a76812a3071b51f65) fix(cta): remove `tauri` dep from vanilla recipe ([#1502](https://www.github.com/tauri-apps/tauri/pull/1502)) on 2021-04-15
- Fix adding `tauri` script to package.json
  - [6c00e88](https://www.github.com/tauri-apps/tauri/commit/6c00e88e0ffa10eb7eecc312d66c5dde7dc03d0b) fix(cta): fix adding `tauri` script to package.json ([#1501](https://www.github.com/tauri-apps/tauri/pull/1501)) on 2021-04-15
  - [345f2db](https://www.github.com/tauri-apps/tauri/commit/345f2dbfc545427750c08351d1b98e966b2436c0) Apply Version Updates From Current Changes ([#1499](https://www.github.com/tauri-apps/tauri/pull/1499)) on 2021-04-14
  - [098b729](https://www.github.com/tauri-apps/tauri/commit/098b729e677dc5dc322f22a6cbd5a652a8dfa1b0) chore: CTA version was decremented, fix and adjust changelog to compensate ([#1530](https://www.github.com/tauri-apps/tauri/pull/1530)) on 2021-04-18

## \[1.0.0-beta-rc.2]

- CTA also needs the template directory published as it doesn't get bundled into the `dist` directory.
  - [7b6108e](https://www.github.com/tauri-apps/tauri/commit/7b6108e37be652a1efa4018fc1908aa0a2cdacd6) fix: cta templates dir missing ([#1496](https://www.github.com/tauri-apps/tauri/pull/1496)) on 2021-04-14

## \[1.0.0-beta-rc.1]

- CTA was missing the `files` property in the package.json which mean that the `dist` directory was not published and used.
  - [414f9a7](https://www.github.com/tauri-apps/tauri/commit/414f9a78c9b636933fd741d1b6fe7f097f496fc9) fix: cta dist publish ([#1493](https://www.github.com/tauri-apps/tauri/pull/1493)) on 2021-04-14

## \[1.0.0-beta-rc.0]

- Add vanilla javascript option to `create-tauri-app` through templating.
  - [c580338](https://www.github.com/tauri-apps/tauri/commit/c580338f07b71551f7fd2712e13ad0acef100095) feat(cli): add create-tauri-app ([#1106](https://www.github.com/tauri-apps/tauri/pull/1106)) on 2021-03-07
- Use a test based on an npm env var to determine which package manager to use.
  - [6e0598c](https://www.github.com/tauri-apps/tauri/commit/6e0598c807ce02a3964788c06ec1025abc1fb250) feat: derive package manager from env var on 2021-04-12
- Add initial `vite` support starting with `vue` and `vue-ts`
  - [80b7bd7](https://www.github.com/tauri-apps/tauri/commit/80b7bd7de86f59e0cafaa0efdc6e82a0db7d7ba2) feat(CTA): add initial vite support with `vue` and `vue-ts` ([#1467](https://www.github.com/tauri-apps/tauri/pull/1467)) on 2021-04-13
- Revert `tauri create` deletion and shift remaining pieces that weren't deleted to `create-tauri-app`.
  - [4ec20a4](https://www.github.com/tauri-apps/tauri/commit/4ec20a4a28823614186365c5a90512d77170cff2) feat: shift tauri create \[not wired up] ([#1330](https://www.github.com/tauri-apps/tauri/pull/1330)) on 2021-03-07
  - [aea6145](https://www.github.com/tauri-apps/tauri/commit/aea614587bddab930d552512b54e18624fbf573e) refactor(repo): add /tooling folder ([#1457](https://www.github.com/tauri-apps/tauri/pull/1457)) on 2021-04-12
