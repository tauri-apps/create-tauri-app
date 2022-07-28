// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input, Select};
use rust_embed::RustEmbed;
use std::{ffi::OsString, fs, path, process::exit};

use crate::package_manager::PackageManager;

mod cli;
mod package_manager;
mod template;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
struct Fragments;

pub fn run<I, A>(args: I, bin_name: Option<String>)
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    if let Err(e) = try_run(args, bin_name) {
        eprintln!("{}: {:#}", style("error").red().bold(), e);
        exit(1);
    }
}

fn try_run<I, A>(args: I, bin_name: Option<String>) -> anyhow::Result<()>
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    let args = cli::parse(args.into_iter().map(Into::into).collect(), bin_name)?;
    let defaults = cli::Args::default();
    let skip = args.skip_prompts;
    let cwd = std::env::current_dir()?;

    let project_name = args.project_name.unwrap_or_else(|| {
        if skip {
            defaults.project_name.unwrap()
        } else {
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default("tauri-app".into())
                .interact_text()
                .unwrap()
        }
    });

    let target_dir = cwd.join(&project_name);

    let package_name = if is_valid_pkg_name(&project_name) {
        project_name.clone()
    } else {
        let valid_name = to_valid_pkg_name(&project_name);
        if skip {
            valid_name
        } else {
            Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Package name")
            .default(valid_name.clone())
            .with_initial_text(valid_name)
            .validate_with(|input: &String| {
                if is_valid_pkg_name(&input) {
                    Ok(())
                } else {
                    Err("Package name should only include alphanumeric character and hyphens \"-\" and doesn't start with numbers")
                }
            })
            .interact_text()?
        }
    };

    if target_dir.exists() && !target_dir.read_dir()?.next().is_none() {
        let overrwite = if skip {
            false
        } else {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "{} directory is not empty, do you want to overwrite?",
                    if target_dir == cwd {
                        "Current directory".to_string()
                    } else {
                        target_dir
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                    }
                ))
                .default(false)
                .interact()?
        };
        if !overrwite {
            eprintln!("{} Operation Cancelled", style("✘").red());
            exit(1);
        }
    };

    let pkg_manager = args.manager.unwrap_or_else(|| {
        if skip {
            defaults.manager.unwrap()
        } else {
            let managers = PackageManager::ALL;
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose your package manager")
                .items(managers)
                .default(0)
                .interact()
                .unwrap();
            managers[index]
        }
    });

    let templates = pkg_manager.templates();
    let template = args.template.unwrap_or_else(|| {
        if skip {
            defaults.template.unwrap()
        } else {
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose your UI template")
                .items(templates)
                .default(0)
                .interact()
                .unwrap();
            templates[index]
        }
    });

    if !templates.contains(&template) {
        eprintln!(
            "{}: the {} template is not suppported for {} package manager\n       possible templates for {} are: [{}]",
            style("error").red().bold(),
            style(template).green(),
            style(pkg_manager).green(),
            style(pkg_manager).green(),
            templates.into_iter().map(|e|style(e).green().to_string()).collect::<Vec<_>>().join(", ")
        );
        exit(1);
    }

    if target_dir.exists() {
        // safe to remove, because upon reaching this line, the user accepted to overwrite
        fs::remove_dir_all(&target_dir)?
    };
    fs::create_dir_all(&target_dir)?;

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
                let managers_list = managers_str.split("-").collect::<Vec<_>>();
                if managers_list.contains(&pkg_manager.to_string().as_str()) {
                    p.parent().unwrap().join(file_name)
                } else {
                    return Ok(());
                }
            }
            _ => p,
        };

        fs::create_dir_all(&target_file.parent().unwrap())?;
        fs::write(target_file, Fragments::get(&*file).unwrap().data)?;
        Ok(())
    };

    // write base files first
    for file in Fragments::iter().filter(|e| {
        path::PathBuf::from(e.to_string())
            .components()
            .nth(0)
            .unwrap()
            .as_os_str()
            == path::PathBuf::from("fragment-base")
    }) {
        write_file(&*file)?;
    }

    // then write template files which can override files from base
    for file in Fragments::iter().filter(|e| {
        path::PathBuf::from(e.to_string())
            .components()
            .nth(0)
            .unwrap()
            .as_os_str()
            == path::PathBuf::from(format!("fragment-{template}"))
    }) {
        write_file(&*file)?;
    }

    // update package.json
    let pkg_json = target_dir.join("package.json");
    update_file_content(&pkg_json, |f| f.replace("{{package_name}}", &package_name))?;

    // update tauri.conf.json
    let tauri_conf = target_dir.join("src-tauri").join("tauri.conf.json");
    update_file_content(&tauri_conf, |f| {
        f.replace("{{package_name}}", &package_name)
            .replace("{{pkg_manager_run_command}}", pkg_manager.run_cmd())
    })?;

    // update Cargo.toml
    let cargo_toml = target_dir.join("src-tauri").join("Cargo.toml");
    update_file_content(&cargo_toml, |f| {
        f.replace("{{package_name}}", &package_name)
    })?;

    println!("");
    println!("Done. Please follow https://tauri.app/v1/guides/getting-started/prerequisites to install the needed prerequisites, if you haven't already.");
    println!("Now run:");
    println!("  cd {}", project_name);
    if !pkg_manager.install_cmd().is_empty() {
        println!("  {}", pkg_manager.install_cmd());
    }
    println!("  {} tauri dev", pkg_manager.run_cmd());
    println!("");

    Ok(())
}

fn update_file_content<P: AsRef<path::Path>, F: FnMut(String) -> String>(
    p: P,
    mut f: F,
) -> anyhow::Result<()> {
    if p.as_ref().exists() {
        let file = fs::read_to_string(&p)?;
        let file = f(file);
        fs::write(&p, file)?;
    }
    Ok(())
}

fn is_valid_pkg_name(project_name: &str) -> bool {
    !project_name
        .chars()
        .next()
        .map(|c| c.is_digit(10))
        .unwrap_or_default()
        && !project_name
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_'))
}

fn to_valid_pkg_name(project_name: &str) -> String {
    let mut ret = project_name
        .trim()
        .to_lowercase()
        .replace(".", "-")
        .replace(":", "-")
        .replace(";", "-")
        .replace(" ", "-")
        .replace("\\", "-")
        .replace("/", "-")
        .replace("~", "-");

    if let Some(ch) = ret.chars().next() {
        if ch.is_digit(10) || ch == '-' {
            ret.remove(0);
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valiadtes_pkg_name() {
        assert_eq!(is_valid_pkg_name("tauri-app"), true);
        assert_eq!(is_valid_pkg_name("tauri_app"), true);
        assert_eq!(is_valid_pkg_name("t2auriapp"), true);
        assert_eq!(is_valid_pkg_name("1tauriapp"), false);
        assert_eq!(is_valid_pkg_name("tauri app"), false);
        assert_eq!(is_valid_pkg_name("tauri:app"), false);
        assert_eq!(is_valid_pkg_name("tauri.app"), false);
        assert_eq!(is_valid_pkg_name("tauri/app"), false);
        assert_eq!(is_valid_pkg_name("tauri\\app"), false);
        assert_eq!(is_valid_pkg_name("tauri~app"), false);
    }

    #[test]
    fn converts_to_valid_pkg_name() {
        assert_eq!(to_valid_pkg_name("tauri-app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri_app"), "tauri_app");
        assert_eq!(to_valid_pkg_name("t2auriapp"), "t2auriapp");
        assert_eq!(to_valid_pkg_name("1tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri:app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri;app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri.app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri/app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri\\app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri~app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("-tauri.app"), "tauri-app");
    }
}
