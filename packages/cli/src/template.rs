// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fmt::Display, fs, path, str::FromStr};

use anyhow::{bail, Context};
use rust_embed::RustEmbed;

use crate::{colors::*, package_manager::PackageManager};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
struct Fragments;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Template {
    #[default]
    Vanilla,
    Vue,
    VueTs,
    Svelte,
    SvelteTs,
    React,
    ReactTs,
    Solid,
    SolidTs,
    Yew,
    Next,
    NextTs,
    Preact,
    PreactTs,
}

impl<'a> Template {
    pub const ALL: &'a [Template] = &[
        Template::Vanilla,
        Template::Vue,
        Template::VueTs,
        Template::Svelte,
        Template::SvelteTs,
        Template::React,
        Template::ReactTs,
        Template::Solid,
        Template::SolidTs,
        Template::Yew,
        Template::Next,
        Template::NextTs,
        Template::Preact,
        Template::PreactTs,
    ];

    pub fn post_init_info(&self) -> Option<String> {
        match self {
            Template::Yew => Some(format!("{ITALIC}{DIM}You also need to install{DIMRESET} {YELLOW}tauri-cli{WHITE} {DIM}({DIMRESET}{BLUE}cargo install tauri-cli{WHITE}{DIM}) and{DIMRESET} {YELLOW}trunk{WHITE} {DIM}({DIMRESET}{BLUE}https://trunkrs.dev/#install{WHITE}{DIM}){DIMRESET}{RESET}")),
            _ => None,
        }
    }

    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
    ) -> anyhow::Result<()> {
        let manifest_bytes = Fragments::get(&format!("fragment-{}/_cta_manifest_", self))
            .with_context(|| "Failed to get manifest bytes")?
            .data;
        let manifest_str = String::from_utf8(manifest_bytes.to_vec())?;
        let manifest = Manifest::parse(&manifest_str)?;

        let write_file = |file: &str| -> anyhow::Result<()> {
            let manifest = manifest.clone();

            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .collect::<path::PathBuf>();

            let p = target_dir.join(&p);
            let file_name = p.file_name().unwrap().to_string_lossy();

            let target_file_name = match &*file_name {
                "_gitignore" => ".gitignore",
                "_Cargo.toml" => "Cargo.toml",
                "_cta_manifest_" => return Ok(()),
                // conditional files:
                // are files that start with a special syntax
                //          "_[<list of package managers separated by `-`>]_<file_name>"
                // example: "_[pnpm-npm-yarn]_package.json"
                name if name.starts_with("_[") => {
                    let mut s = name.strip_prefix("_[").unwrap().split("]_");
                    let (mut managers, name) = (s.next().unwrap().split('-'), s.next().unwrap());
                    if managers.any(|x| x == pkg_manager.to_string()) {
                        name
                    } else {
                        // skip writing this file
                        return Ok(());
                    }
                }
                _ => &file_name,
            };

            let mut data = Fragments::get(file).unwrap().data.to_vec();

            // Only modify specific set of files
            if ["Cargo.toml", "package.json", "tauri.conf.json"].contains(&target_file_name) {
                if let Ok(str_) = String::from_utf8(data.to_vec()) {
                    data = str_
                        .replace("{{package_name}}", package_name)
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
            fs::create_dir_all(&parent)?;
            fs::write(parent.join(target_file_name), &data)?;
            Ok(())
        };

        // write base files first
        for file in Fragments::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == "base"
        }) {
            write_file(&file)?;
        }

        // then write template files which can override files from base
        for file in Fragments::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == path::PathBuf::from(format!("fragment-{self}"))
        }) {
            write_file(&file)?;
        }

        // then write extra files specified in the fragment manifest
        for (src, dest) in manifest.files {
            let data = Fragments::get(&format!("_assets_/{}", src))
                .with_context(|| format!("Failed to get asset file bytes: {src}"))?
                .data;
            let dest = target_dir.join(dest);
            let parent = dest.parent().unwrap();
            fs::create_dir_all(&parent)?;
            fs::write(dest, &data)?;
        }

        Ok(())
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Vanilla => write!(f, "vanilla"),
            Template::Vue => write!(f, "vue"),
            Template::VueTs => write!(f, "vue-ts"),
            Template::Svelte => write!(f, "svelte"),
            Template::SvelteTs => write!(f, "svelte-ts"),
            Template::React => write!(f, "react"),
            Template::ReactTs => write!(f, "react-ts"),
            Template::Solid => write!(f, "solid"),
            Template::SolidTs => write!(f, "solid-ts"),
            Template::Yew => write!(f, "yew"),
            Template::Next => write!(f, "next"),
            Template::NextTs => write!(f, "next-ts"),
            Template::Preact => write!(f, "preact"),
            Template::PreactTs => write!(f, "preact-ts"),
        }
    }
}

impl FromStr for Template {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vanilla" => Ok(Template::Vanilla),
            "vue" => Ok(Template::Vue),
            "vue-ts" => Ok(Template::VueTs),
            "svelte" => Ok(Template::Svelte),
            "svelte-ts" => Ok(Template::SvelteTs),
            "react" => Ok(Template::React),
            "react-ts" => Ok(Template::ReactTs),
            "solid" => Ok(Template::Solid),
            "solid-ts" => Ok(Template::SolidTs),
            "yew" => Ok(Template::Yew),
            "next" => Ok(Template::Next),
            "next-ts" => Ok(Template::NextTs),
            "preact" => Ok(Template::Preact),
            "preact-ts" => Ok(Template::PreactTs),
            _ => Err("Invalid template".to_string()),
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
        let mut is_files_section = false;
        for (i, line) in s.split('\n').enumerate() {
            let line = line.split('#').next().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            if line == "[files]" {
                is_files_section = true;
                continue;
            }

            if line.contains('=') {
                let mut s = line.split('=');
                let (k, v) = (
                    s.next()
                        .with_context(|| {
                            format!("parsing manifest: key is not found in line {}", i + 1)
                        })?
                        .trim(),
                    s.next()
                        .with_context(|| {
                            format!("parsing manifest: value is not found in line {}", i + 1)
                        })?
                        .trim(),
                );

                if k.is_empty() {
                    bail!("parsing manifest: key is empty in line {}", i + 1);
                }

                if v.is_empty() {
                    bail!("parsing manifest: value is empty in line {}", i + 1);
                }

                match k {
                    "beforeDevCommand" => manifest.before_dev_command = Some(v),
                    "beforeBuildCommand" => manifest.before_build_command = Some(v),
                    "devPath" => manifest.dev_path = Some(v),
                    "distDir" => manifest.dist_dir = Some(v),
                    "withGlobalTauri" => manifest.with_global_tauri = v.parse()?,
                    _ if is_files_section => {
                        manifest.files.insert(k, v);
                    }
                    _ => {}
                }
            }
        }
        Ok(manifest)
    }
}
