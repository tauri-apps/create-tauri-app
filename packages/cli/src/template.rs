// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{fmt::Display, fs, path, str::FromStr};

use rust_embed::RustEmbed;

use crate::{colors::*, package_manager::PackageManager};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
struct Fragments;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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
    ];

    pub fn post_init_info(&self) -> String {
        match self {
            Template::Yew => format!("{ITALIC}{DIM}You also need to install{DIMRESET} {YELLOW}tauri-cli{WHITE} {DIM}({DIMRESET}{BLUE}cargo install tauri-cli{WHITE}{DIM}) and{DIMRESET} {YELLOW}trunk{WHITE} {DIM}({DIMRESET}{BLUE}https://trunkrs.dev/#install{WHITE}{DIM}){DIMRESET}{RESET}"),
            _ => String::new(),
        }
    }

    pub fn render(
        &self,
        target_dir: &path::Path,
        pkg_manager: PackageManager,
        package_name: &str,
    ) -> anyhow::Result<()> {
        let write_file = |file: &str| -> anyhow::Result<()> {
            // remove the first component, which is certainly the fragment directory they were in before getting embeded into the binary
            let p = path::PathBuf::from(file)
                .components()
                .skip(1)
                .collect::<Vec<_>>()
                .iter()
                .collect::<path::PathBuf>();

            let p = target_dir.join(&p);

            let target_file = match &*p.file_name().unwrap().to_string_lossy() {
                "_gitignore" => p.parent().unwrap().join(".gitignore"),
                // render conditional files
                // conditional files are files that start with a special conventions
                //  _[<list of package managers separated by `-`>]_<file_name>
                // ex: _[pnpm-npm-yarn]package.json
                name if name.starts_with("_[") => {
                    let mut s = name.strip_prefix("_[").unwrap().split("]_");
                    let (managers_str, file_name) = (s.next().unwrap(), s.next().unwrap());
                    if managers_str
                        .split('-')
                        .any(|x| x == pkg_manager.to_string().as_str())
                    {
                        p.parent().unwrap().join(file_name)
                    } else {
                        return Ok(());
                    }
                }
                _ => p,
            };

            let mut data = Fragments::get(&*file).unwrap().data.to_vec();

            if let Ok(str_) = String::from_utf8(data.to_vec()) {
                data = str_
                    .replace("{{package_name}}", package_name)
                    .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
                    .as_bytes()
                    .to_vec();
            }

            fs::create_dir_all(&target_file.parent().unwrap())?;
            fs::write(target_file, &data)?;
            Ok(())
        };

        // write base files first
        for file in Fragments::iter().filter(|e| {
            path::PathBuf::from(e.to_string())
                .components()
                .next()
                .unwrap()
                .as_os_str()
                == "fragment-base"
        }) {
            write_file(&*file)?;
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
            write_file(&*file)?;
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
            _ => Err("Invalid template".to_string()),
        }
    }
}
