// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fmt::Display, fs, io::Write, path, str::FromStr};

use anyhow::{bail, Context};
use rust_embed::RustEmbed;

use crate::{colors::*, package_manager::PackageManager};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
#[allow(clippy::upper_case_acronyms)]
struct FRAGMENTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Template {
    Vanilla,
    VanillaTs,
    Vue,
    VueTs,
    Svelte,
    SvelteTs,
    React,
    ReactTs,
    Solid,
    SolidTs,
    Yew,
    Leptos,
    Sycamore,
}

impl Default for Template {
    fn default() -> Self {
        Template::Vanilla
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Vanilla => write!(f, "vanilla"),
            Template::VanillaTs => write!(f, "vanilla-ts"),
            Template::Vue => write!(f, "vue"),
            Template::VueTs => write!(f, "vue-ts"),
            Template::Svelte => write!(f, "svelte"),
            Template::SvelteTs => write!(f, "svelte-ts"),
            Template::React => write!(f, "react"),
            Template::ReactTs => write!(f, "react-ts"),
            Template::Solid => write!(f, "solid"),
            Template::SolidTs => write!(f, "solid-ts"),
            Template::Yew => write!(f, "yew"),
            Template::Leptos => write!(f, "leptos"),
            Template::Sycamore => write!(f, "sycamore"),
        }
    }
}

impl FromStr for Template {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vanilla" => Ok(Template::Vanilla),
            "vanilla-ts" => Ok(Template::VanillaTs),
            "vue" => Ok(Template::Vue),
            "vue-ts" => Ok(Template::VueTs),
            "svelte" => Ok(Template::Svelte),
            "svelte-ts" => Ok(Template::SvelteTs),
            "react" => Ok(Template::React),
            "react-ts" => Ok(Template::ReactTs),
            "solid" => Ok(Template::Solid),
            "solid-ts" => Ok(Template::SolidTs),
            "yew" => Ok(Template::Yew),
            "leptos" => Ok(Template::Leptos),
            "sycamore" => Ok(Template::Sycamore),
            _ => Err("Invalid template".to_string()),
        }
    }
}

impl<'a> Template {
    pub const ALL: &'a [Template] = &[
        Template::Vanilla,
        Template::VanillaTs,
        Template::Vue,
        Template::VueTs,
        Template::Svelte,
        Template::SvelteTs,
        Template::React,
        Template::ReactTs,
        Template::Solid,
        Template::SolidTs,
        Template::Yew,
        Template::Leptos,
        Template::Sycamore,
    ];

    pub fn flavors<'b>(&self, pkg_manager: PackageManager) -> Option<&'b [Flavor]> {
        match self {
            Template::Vanilla => {
                if pkg_manager == PackageManager::Cargo {
                    None
                } else {
                    Some(&[Flavor::JavaScript, Flavor::TypeScript])
                }
            }
            Template::Vue => Some(&[Flavor::JavaScript, Flavor::TypeScript]),
            Template::Svelte => Some(&[Flavor::JavaScript, Flavor::TypeScript]),
            Template::React => Some(&[Flavor::JavaScript, Flavor::TypeScript]),
            Template::Solid => Some(&[Flavor::JavaScript, Flavor::TypeScript]),
            _ => None,
        }
    }

    pub fn from_flavor(&self, flavor: Flavor) -> Self {
        match (self, flavor) {
            (Template::Vanilla, Flavor::TypeScript) => Template::VanillaTs,
            (Template::Vue, Flavor::TypeScript) => Template::VueTs,
            (Template::Svelte, Flavor::TypeScript) => Template::SvelteTs,
            (Template::React, Flavor::TypeScript) => Template::ReactTs,
            (Template::Solid, Flavor::TypeScript) => Template::SolidTs,
            _ => *self,
        }
    }

    pub fn post_init_info(&self, pkg_manager: PackageManager, alpha: bool) -> Option<String> {
        let tauri_cli_cmd = if alpha {
            "cargo install tauri-cli --version 2.0.0-alpha.2"
        } else {
            "cargo install tauri-cli"
        };

        match self {
            Template::Yew | Template::Leptos| Template::Sycamore => Some(
                format!(
                    "{ITALIC}{DIM}You also need to install:\n    1. {DIMRESET}{YELLOW}tauri-cli{WHITE}{DIM} ({DIMRESET}{BLUE}{tauri_cli_cmd}{WHITE}{DIM})\n    2. {DIMRESET}{YELLOW}trunk{WHITE}{DIM} ({DIMRESET}{BLUE}https://trunkrs.dev/#install{WHITE}{DIM})\n    3. {DIMRESET}{YELLOW}wasm32{WHITE}{DIM} rust target ({DIMRESET}{BLUE}rustup target add wasm32-unknown-unknown{WHITE}{DIM}){DIMRESET}{RESET}",
                    ITALIC = ITALIC,
                    DIM = DIM,
                    DIMRESET = DIMRESET,
                    YELLOW = YELLOW,
                    WHITE = WHITE,
                    BLUE = BLUE,
                    RESET = RESET,
                    tauri_cli_cmd = tauri_cli_cmd,
                ),
            ),
            Template::Vanilla if pkg_manager == PackageManager::Cargo => Some(
                    format!(
                        "{ITALIC}{DIM}You also need to install{DIMRESET} {YELLOW}tauri-cli{WHITE} {DIM}({DIMRESET}{BLUE}{tauri_cli_cmd}{WHITE}{DIM})",
                        ITALIC = ITALIC,
                        DIM = DIM,
                        DIMRESET = DIMRESET,
                        YELLOW = YELLOW,
                        WHITE = WHITE,
                        BLUE = BLUE,
                        tauri_cli_cmd = tauri_cli_cmd,
                    ),
                ),
            _ => None,
        }
    }

    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
        alpha: bool,
        mobile: bool,
    ) -> anyhow::Result<()> {
        let manifest_bytes = FRAGMENTS::get(&format!("fragment-{}/_cta_manifest_", self))
            .with_context(|| "Failed to get manifest bytes")?
            .data;
        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;
        let manifest = Manifest::parse(&manifest_str)?;

        let lib_name = format!("{}_lib", package_name.replace('-', "_"));

        let write_file = |file: &str| -> anyhow::Result<()> {
            let manifest = manifest.clone();

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .collect::<path::PathBuf>();

            let p = target_dir.join(p);
            let file_name = p.file_name().unwrap().to_string_lossy();

            let target_file_name = match &*file_name {
                "_gitignore" => ".gitignore",
                "_Cargo.toml" => "Cargo.toml",
                "_cta_manifest_" => return Ok(()),
                // conditional files:
                // are files that start with a special syntax
                //          "%(<list of flags separated by `-`>%)<file_name>"
                // flags are supported package managers, stable, alpha and mobile.
                // example: "%(pnpm-npm-yarn-stable-alpha)%package.json"
                name if name.starts_with("%(") && name[1..].contains(")%") => {
                    let mut s = name.strip_prefix("%(").unwrap().split(")%");
                    let (mut flags, name) = (
                        s.next().unwrap().split('-').collect::<Vec<_>>(),
                        s.next().unwrap(),
                    );

                    let for_stable = flags.contains(&"stable");
                    let for_alpha = flags.contains(&"alpha");
                    let for_mobile = flags.contains(&"mobile");

                    // remove these flags to only keep package managers flags
                    flags.retain(|e| !["stable", "alpha", "mobile"].contains(e));

                    if ((for_stable && !alpha)
                        || (for_alpha && alpha && !mobile)
                        || (for_mobile && alpha && mobile)
                        || (!for_stable && !for_alpha && !for_mobile))
                        && (flags.contains(&pkg_manager.to_string().as_str()) || flags.is_empty())
                    {
                        name
                    } else {
                        // skip writing this file
                        return Ok(());
                    }
                }
                _ => &file_name,
            };

            let mut data = FRAGMENTS::get(file).unwrap().data.to_vec();

            // Only modify specific set of files
            if [
                "Cargo.toml",
                "package.json",
                "tauri.conf.json",
                "main.rs",
                "vite.config.ts",
                "vite.config.js",
                "Trunk.toml",
            ]
            .contains(&target_file_name)
            {
                if let Ok(content) = String::from_utf8(data.to_vec()) {
                    data = content
                        .replace("{{package_name}}", package_name)
                        .replace("{{lib_name}}", &lib_name)
                        .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
                        .replace(
                            "{{fragment_before_dev_command}}",
                            manifest.before_dev_command.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_before_build_command}}",
                            manifest.before_build_command.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_dev_path}}",
                            manifest.dev_path.unwrap_or_default(),
                        )
                        .replace(
                            "{{fragment_dist_dir}}",
                            manifest.dist_dir.unwrap_or_default(),
                        )
                        .replace(
                            r#""withGlobalTauri": "{{fragment_with_global_tauri}}""#,
                            &format!(r#""withGlobalTauri": {}"#, manifest.with_global_tauri),
                        )
                        .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
                        .as_bytes()
                        .to_vec();
                }
            }

            let parent = p.parent().unwrap();
            fs::create_dir_all(parent)?;
            fs::write(parent.join(target_file_name), &data)?;
            Ok(())
        };

        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == "_base_"
        }) {
            write_file(&file)?;
        }

        // then write template files which can override files from base
        for file in FRAGMENTS::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == path::PathBuf::from(format!("fragment-{}", self))
        }) {
            write_file(&file)?;
        }

        // then write extra files specified in the fragment manifest
        for (src, dest) in manifest.files {
            let data = FRAGMENTS::get(&format!("_assets_/{}", src))
                .with_context(|| format!("Failed to get asset file bytes: {}", src))?
                .data;
            let dest = target_dir.join(dest);
            let parent = dest.parent().unwrap();
            fs::create_dir_all(parent)?;
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(dest)?;
            file.write_all(&data)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Flavor {
    JavaScript,
    TypeScript,
}

impl Display for Flavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flavor::JavaScript => write!(f, "JavaScript"),
            Flavor::TypeScript => write!(f, "TypeScript"),
        }
    }
}

#[derive(Default, Clone)]
struct Manifest<'a> {
    before_dev_command: Option<&'a str>,
    before_build_command: Option<&'a str>,
    dev_path: Option<&'a str>,
    dist_dir: Option<&'a str>,
    with_global_tauri: bool,
    files: HashMap<&'a str, &'a str>,
}

impl<'a> Manifest<'a> {
    fn parse(s: &'a str) -> Result<Self, anyhow::Error> {
        let mut manifest = Manifest::default();
        let mut in_files_section = false;
        for (i, line) in s.split('\n').enumerate() {
            let line_number = i + 1;

            // ignore the comment portion of the line
            let line = line.split('#').next().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            if line == "[files]" {
                in_files_section = true;
                continue;
            }

            if line.contains('=') {
                let mut s = line.split('=');
                let (k, v) = (
                    s.next()
                        .with_context(|| {
                            format!("parsing manifest: key is not found in line {}", line_number)
                        })?
                        .trim(),
                    s.next()
                        .with_context(|| {
                            format!(
                                "parsing manifest: value is not found in line {}",
                                line_number
                            )
                        })?
                        .trim(),
                );

                if k.is_empty() {
                    bail!("parsing manifest: key is empty in line {}", line_number);
                }

                if v.is_empty() {
                    bail!("parsing manifest: value is empty in line {}", line_number);
                }

                match k {
                    "beforeDevCommand" => manifest.before_dev_command = Some(v),
                    "beforeBuildCommand" => manifest.before_build_command = Some(v),
                    "devPath" => manifest.dev_path = Some(v),
                    "distDir" => manifest.dist_dir = Some(v),
                    "withGlobalTauri" => manifest.with_global_tauri = v.parse()?,
                    _ if in_files_section => {
                        manifest.files.insert(k, v);
                    }
                    _ => {}
                }
            }
        }
        Ok(manifest)
    }
}
