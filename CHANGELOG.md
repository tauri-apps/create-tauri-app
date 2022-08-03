# Changelog

## \[2.0.0-alpha.7]

- 2.0.0-alpha.7
  - [2237271](https://www.github.com/tauri-apps/create-tauri-app/commit/2237271f724cbc468a0f2e4c46ce42293b4d6a8b) alpha.7 on 2022-08-03

## \[2.0.0-alpha.6]

- 2.0.0-alpha.6
  - [9667bfa](https://www.github.com/tauri-apps/create-tauri-app/commit/9667bfa9261f158a01b8202a7fa429a9bd559d22) alpha.6 on 2022-08-03

## \[2.0.0-alpha.5]

- 2.0.0-alpha.5
  - [bc68a1d](https://www.github.com/tauri-apps/create-tauri-app/commit/bc68a1d1770239e21d0fee8e03ff1d20e96c970b) alpha.5 on 2022-08-03

## \[2.0.0-alpha.4]

- 2.0.0-alpha.4
  - [f3f6007](https://www.github.com/tauri-apps/create-tauri-app/commit/f3f60072ada609c6563151ed2a522bd6fed6ad47) alpha.4 on 2022-08-03

## \[2.0.0-alpha.3]

- 2.0.0-alpha.3
  - [b542242](https://www.github.com/tauri-apps/create-tauri-app/commit/b54224230a67e00cb5ec4622fc02963047ce6960) alpha.3 on 2022-08-03

## \[2.0.0-alpha.2]

- 2.0.0-alpha.2
  - [83b11a0](https://www.github.com/tauri-apps/create-tauri-app/commit/83b11a043475881100fda612e13506c74d4477da) alpha.2 on 2022-08-03

## \[2.0.0-alpha.1]

- 2.0.0-alpha.1
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
