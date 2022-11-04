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

pub mod internal {
    //! Re-export of create-tauri-app internals
    //!
    //! ## Warning
    //!
    //! This is meant to be used internally only so use at your own risk
    //! and expect APIs to break without a prior notice.
    pub mod package_manager {
        pub use crate::package_manager::*;
    }

    pub mod template {
        pub use crate::template::*;
    }
}

pub fn run<I, A>(args: I, bin_name: Option<String>)
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    if let Err(e) = try_run(args, bin_name) {
        eprintln!(
            "{BOLD}{RED}error{RESET}: {:#}",
            e,
            BOLD = BOLD,
            RED = RED,
            RESET = RESET
        );
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

    // when invoked from pnpm, it seems like pnpm forgets to end its output with a new line
    // and it obscures the first question
    // this ensures we are on a new line before presenting our prompts
    println!();

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
            eprintln!(
                "{BOLD}{RED}✘{RESET} Operation Cancelled",
                BOLD = BOLD,
                RED = RED,
                RESET = RESET
            );
            exit(1);
        }
    };

    let pkg_manager = args.manager.unwrap_or_else(|| {
        if skip {
            defaults.manager.unwrap()
        } else {
            let managers = PackageManager::ALL.to_vec();
            let managers = args
                .template
                .map(|t| {
                    managers
                        .iter()
                        .copied()
                        .filter(|p| p.templates().contains(&t))
                        .collect::<Vec<_>>()
                })
                .unwrap_or(managers);
            if managers.len() == 1 {
                managers[0]
            } else {
                let index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose your package manager")
                    .items(&managers)
                    .default(0)
                    .interact()
                    .unwrap();
                managers[index]
            }
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
            "{BOLD}{RED}error{RESET}: the {GREEN}{}{RESET} template is not suppported for the {GREEN}{pkg_manager}{RESET} package manager\n       possible templates for {GREEN}{pkg_manager}{RESET} are: [{}]",
            template,
            templates.iter().map(|e|format!("{GREEN}{}{RESET}", e, GREEN = GREEN, RESET = RESET)).collect::<Vec<_>>().join(", "),
            pkg_manager = pkg_manager,
            BOLD = BOLD,
            RED = RED,
            RESET = RESET,
            GREEN = GREEN,
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
    println!(
        "{ITALIC}{DIM}Please follow{DIMRESET} {BLUE}https://tauri.app/v1/guides/getting-started/prerequisites{WHITE} {DIM}to install the needed prerequisites, if you haven't already.{DIMRESET}{RESET}",
        ITALIC = ITALIC,
        DIM = DIM,
        DIMRESET = DIMRESET,
        WHITE = WHITE,
        BLUE = BLUE,
        RESET=RESET
    );
    if let Some(info) = template.post_init_info(pkg_manager) {
        println!("{}", info);
    }
    println!();
    println!("Done, Now run:");
    println!("  cd {}", project_name);
    if let Some(cmd) = pkg_manager.install_cmd() {
        println!("  {}", cmd);
    }
    println!("  {} tauri dev", pkg_manager.run_cmd());
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
    #[allow(clippy::collapsible_str_replace)]
    let mut ret = project_name
        .trim()
        .to_lowercase()
        .replace(':', "-")
        .replace(';', "-")
        .replace(' ', "-")
        .replace('~', "-")
        .replace('.', "")
        .replace('\\', "")
        .replace('/', "");

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
        assert_eq!(to_valid_pkg_name("tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri/app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri\\app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri~app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("-tauri.app"), "tauriapp");
    }
}
