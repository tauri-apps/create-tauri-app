// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::{ffi::OsString, fs, process::exit};

use crate::{colors::*, package_manager::PackageManager};

mod cli;
mod colors;
mod package_manager;
mod template;

pub fn run<I, A>(args: I, bin_name: Option<String>)
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    if let Err(e) = try_run(args, bin_name) {
        eprintln!("{BOLD}{RED}error{RESET}: {e:#}");
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
                if is_valid_pkg_name(input) {
                    Ok(())
                } else {
                    Err("Package name should only include alphanumeric character and hyphens \"-\" and doesn't start with numbers")
                }
            })
            .interact_text()?
        }
    };

    if target_dir.exists() && target_dir.read_dir()?.next().is_some() {
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
            eprintln!("{BOLD}{RED}✘{RESET} Operation Cancelled");
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
            "{BOLD}{RED}error{RESET}: the {GREEN}{template}{RESET} template is not suppported for the {GREEN}{pkg_manager}{RESET} package manager\n       possible templates for {GREEN}{pkg_manager}{RESET} are: [{}]",
            templates.iter().map(|e|format!("{GREEN}{e}{RESET}")).collect::<Vec<_>>().join(", ")
        );
        exit(1);
    }

    if target_dir.exists() {
        // safe to remove, because upon reaching this line, the user accepted to overwrite
        fs::remove_dir_all(&target_dir)?
    };
    fs::create_dir_all(&target_dir)?;

    template.render(&target_dir, pkg_manager, &package_name)?;

    println!();
    println!("Done, now run:");
    println!("  cd {}", project_name);
    if !pkg_manager.install_cmd().is_empty() {
        println!("  {}", pkg_manager.install_cmd());
    }
    println!("  {} tauri dev", pkg_manager.run_cmd());
    println!();
    println!(
       
            "{ITALIC}{DIM}Please follow{DIMRESET} {BLUE}https://tauri.app/v1/guides/getting-started/prerequisites{WHITE} {DIM}to install the needed prerequisites, if you haven't already.{DIMRESET}{RESET}"
    );
    if !template.post_init_info().is_empty() {
        println!("{}", template.post_init_info());
    }
    println!();
    Ok(())
}

fn is_valid_pkg_name(project_name: &str) -> bool {
    !project_name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
        && !project_name
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_'))
}

fn to_valid_pkg_name(project_name: &str) -> String {
    let mut ret = project_name
        .trim()
        .to_lowercase()
        .replace('.', "-")
        .replace(':', "-")
        .replace(';', "-")
        .replace(' ', "-")
        .replace('\\', "-")
        .replace('/', "-")
        .replace('~', "-");

    if let Some(ch) = ret.chars().next() {
        if ch.is_ascii_digit() || ch == '-' {
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
        assert!(is_valid_pkg_name("tauri-app"));
        assert!(is_valid_pkg_name("tauri_app"));
        assert!(is_valid_pkg_name("t2auriapp"));
        assert!(!is_valid_pkg_name("1tauriapp"));
        assert!(!is_valid_pkg_name("tauri app"));
        assert!(!is_valid_pkg_name("tauri:app"));
        assert!(!is_valid_pkg_name("tauri.app"));
        assert!(!is_valid_pkg_name("tauri/app"));
        assert!(!is_valid_pkg_name("tauri\\app"));
        assert!(!is_valid_pkg_name("tauri~app"));
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
