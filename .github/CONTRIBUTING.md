# Tauri Contributing Guide

Hi! We, the maintainers, are really excited that you are interested in contributing to Tauri. Before submitting your contribution though, please make sure to take a moment and read through the [Code of Conduct](CODE_OF_CONDUCT.md), as well as the appropriate section for the contribution you intend to make:

- [Issue Reporting Guidelines](#issue-reporting-guidelines)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Development Guide](#development-guide)

## Issue Reporting Guidelines

- The issue list of this repo is **exclusively** for bug reports and feature requests. Non-conforming issues will be closed immediately.

- If you have a question, you can get quick answers from the [Tauri Discord chat](https://discord.gg/SpmNs4S).

- Try to search for your issue, it may have already been answered or even fixed in the development branch (`dev`).

- Check if the issue is reproducible with the latest stable version of Tauri. If you are using a pre-release, please indicate the specific version you are using.

- It is **required** that you clearly describe the steps necessary to reproduce the issue you are running into. Although we would love to help our users as much as possible, diagnosing issues without clear reproduction steps is extremely time-consuming and simply not sustainable.

- Use only the minimum amount of code necessary to reproduce the unexpected behavior. A good bug report should isolate specific methods that exhibit unexpected behavior and precisely define how expectations were violated. What did you expect the method or methods to do, and how did the observed behavior differ? The more precisely you isolate the issue, the faster we can investigate.

- Issues with no clear repro steps will not be triaged. If an issue labeled "need repro" receives no further input from the issue author for more than 5 days, it will be closed.

- If your issue is resolved but still open, don’t hesitate to close it. In case you found a solution by yourself, it could be helpful to explain how you fixed it.

- Most importantly, we beg your patience: the team must balance your request against many other responsibilities — fixing other bugs, answering other questions, new features, new documentation, etc. The issue list is not paid support and we cannot make guarantees about how fast your issue can be resolved.

## Pull Request Guidelines

- You have to [sign your commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

- It's OK to have multiple small commits as you work on the PR - we will let GitHub automatically squash it before merging.

- If adding new feature:

  - Provide convincing reason to add this feature. Ideally you should open a suggestion issue first and have it greenlighted before working on it.

- If fixing a bug:
  - If you are resolving a special issue, add `(fix: #xxxx[,#xxx])` (#xxxx is the issue id) in your PR title for a better release log, e.g. `fix: update entities encoding/decoding (fix #3899)`.
  - Provide detailed description of the bug in the PR, or link to an issue that does.

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

### Templates

A template is a just combination of two fragments, [`_base_`](../packages/cli/fragments/_base_) (which is shared between all templates) and another fragment that is specific to the template.

#### Adding a new template

> You should open a new issue first to discuss the addition of a certain template.

- Add a directory in `<repo-root>/packages/cli/fragments` and name `fragment-template` where `template` is the name of the template and add all the files you need there as they should appear after the template is created.
- A template also must have a `_cta_manifest_` file which contains info about the template:
  ```ini
  beforeDevCommand = {% pkg_manager_run_command %} dev
  beforeBuildCommand = {% pkg_manager_run_command %} build
  devPath = http://localhost:1420
  distDir = ../dist

  # the next sction is used to determine what files to copy from `fragments/_assets_`
  # if you introduce a new file like an icon that is shared between multiple templates,
  # it should be added to `fragments/_assets_` and add entry for it here
  # for example: `tauri.svg` is shared between all templates so it lives in `fragments/_assets_`
  # and is always added to the next section
  [files]
  # the first part is the path of the file under `_fragments/_assets_`
  # the second part is the path that the file will be copied to under the final template directory
  tauri.svg = public/tauri.svg
  ```
- In `<repo-root>/packages/cli/src/template.rs`, add an entry in the `Template` enum, modify `post_init_info`, `flavors` and `from_flavor` if needed and modify `FromStr` and `Display` implementation
- In `<repo-root>/packages/cli/src/package_manager.rs` add your new template to the appropriate package manager in the `templates` method
- Modify `<repo-root>/.scripts/generate-templates-matrix.js` and append the template name inside the template list for the appropriate package manager so the CI would run tests for it.
- If the template requires system dependencies, add a post init note in `<repo-root>/packages/cli/src/template.rs` in `post_init_info` method.
- Before making a commit, make sure to run `cargo fmt --all` and `pnpm format` in the repo root.

## Financial Contribution

Tauri is an MIT-licensed open source project. Its ongoing development can be supported via [Github Sponsors](https://github.com/sponsors/nothingismagick) or [Open Collective](https://opencollective.com/tauri). We prefer Github Sponsors as donations made are doubled through the matching fund program.
