# Changelog

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
