# Tauri Contributing Guide

Please checkout [Tauri contributing guide](https://github.com/tauri-apps/tauri/blob/dev/.github/CONTRIBUTING.md).

## Development Guide

You need to have [Rust](https://www.rust-lang.org) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

### Testing your changes

```bash
cargo run
```

or

```bash
cargo run -- <cli arguments>
```

#### Adding a new template

> [!IMPORTANT]
> You should open a new issue first to discuss the addition of a certain template.

- Add a directory in `templates` and name it `template-<template-name>` where `<template-name>` is the name of the template and add all the files you need there.
- A template also must have a `.manifest` file which contains info about the template:

  ```ini
  beforeDevCommand = {% pkg_manager_run_command %} dev
  beforeBuildCommand = {% pkg_manager_run_command %} build
  devUrl = http://localhost:1420
  frontendDist = ../dist

  # the next sction is used to determine what files to copy from `templates/_assets_`
  # if you introduce a new file like an icon that is shared between multiple templates,
  # it should be added to `templates/_assets_` and add entry for it here
  # for example: `tauri.svg` is shared between all templates so it lives in `templates/_assets_`
  # and is always added to the next section
  [files]
  # the first part is the path of the file under `templates/_assets_`
  # the second part is the path that the file will be copied to under the final template directory
  tauri.svg = public/tauri.svg
  ```

- In `src/template.rs`, add an entry in the `Template` enum, and modify its methods if needed.
- In `src/package_manager.rs` add your new template to the appropriate package manager in the `templates` method.
- Modify `.scripts/generate-templates-matrix.js` and append the template name inside the template list for the appropriate package manager so the CI would run tests for it.
- Before making a commit, make sure to run `cargo fmt --all` and `pnpm format` in the repo root.

## Financial Contribution

Tauri is an MIT-licensed open source project. Its ongoing development can be supported via [Github Sponsors](https://github.com/sponsors/nothingismagick) or [Open Collective](https://opencollective.com/tauri). We prefer Github Sponsors as donations made are doubled through the matching fund program.
