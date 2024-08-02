// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::Context;
use dialoguer::{Confirm, Input, Select};
use std::{ffi::OsString, fs, process::exit};

use crate::{
    category::Category,
    deps::print_missing_deps,
    package_manager::PackageManager,
    utils::{colors::*, theme::ColorfulTheme},
};

mod args;
mod category;
mod deps;
mod manifest;
mod package_manager;
mod template;
mod utils;

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

pub fn run<I, A>(args: I, bin_name: Option<String>, detected_manager: Option<String>)
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    let _ = ctrlc::set_handler(move || {
        eprint!("\x1b[?25h");
        exit(0);
    });
    if let Err(e) = try_run(args, bin_name, detected_manager) {
        eprintln!("{BOLD}{RED}error{RESET}: {e:#}");
        exit(1);
    }
}

fn try_run<I, A>(
    args: I,
    bin_name: Option<String>,
    detected_manager: Option<String>,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = A>,
    A: Into<OsString> + Clone,
{
    let detected_manager = detected_manager.and_then(|p| p.parse::<PackageManager>().ok());
    let args = args::parse(args.into_iter().map(Into::into).collect(), bin_name)?;
    let defaults = args::Args::default();
    let args::Args {
        skip,
        rc,
        manager,
        project_name,
        template,
        force,
    } = args;
    let cwd = std::env::current_dir()?;

    // Project name used for the project directory name and productName in tauri.conf.json
    // and if valid, it will also be used in Cargo.toml, Package.json ...etc
    let project_name = match project_name {
        Some(name) => name,
        None => {
            if skip {
                defaults
                    .project_name
                    .context("default project_name not set")?
            } else {
                Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Project name")
                    .default("tauri-app".into())
                    .interact_text()?
                    .trim()
                    .into()
            }
        }
    };

    let target_dir = cwd.join(&project_name);

    // Package name used in Cargo.toml, Package.json ...etc
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
                    Err("Package name should only include lowercase alphanumeric character and hyphens \"-\" and doesn't start with numbers")
                }
            })
            .interact_text()?
            .trim().to_string()
        }
    };

    // Confirm deleting the target project directory if not empty
    if target_dir.exists() && target_dir.read_dir()?.next().is_some() {
        let overwrite = if force {
            true
        } else if skip {
            false
        } else {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "{} directory is not empty, do you want to overwrite?",
                    if target_dir == cwd {
                        "Current".to_string()
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
        if !overwrite {
            eprintln!("{BOLD}{RED}✘{RESET} Directory is not empty, Operation Cancelled");
            exit(1);
        }
    };

    // Detect category if a package manger is not passed on the command line
    let category = if manager.is_none() {
        // Filter managers if a template is passed on the command line
        let managers = PackageManager::ALL.to_vec();
        let managers = template
            .map(|t| {
                managers
                    .iter()
                    .copied()
                    .filter(|p| p.templates_no_flavors().contains(&t.without_flavor()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or(managers);

        // Filter categories based on the detected package mangers
        let categories = Category::ALL.to_vec();
        let mut categories = categories
            .into_iter()
            .filter(|c| c.package_managers().iter().any(|p| managers.contains(p)))
            .collect::<Vec<_>>();

        // sort categories so the most relevant category
        // based on the auto-detected package manager is selected first
        categories.sort_by(|a, b| {
            detected_manager
                .map(|p| b.package_managers().contains(&p))
                .unwrap_or(false)
                .cmp(
                    &detected_manager
                        .map(|p| a.package_managers().contains(&p))
                        .unwrap_or(false),
                )
        });

        // Skip prompt, if only one category is detected or explicit skip requested by `-y/--yes` flag
        if categories.len() == 1 || skip {
            Some(categories[0])
        } else {
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose which language to use for your frontend")
                .items(&categories)
                .default(0)
                .interact()?;
            Some(categories[index])
        }
    } else {
        None
    };

    // Package manager which will be used for rendering the template
    // and the after-render instructions
    let pkg_manager = match manager {
        Some(manager) => manager,
        None => {
            if let Some(category) = category {
                let mut managers = category.package_managers().to_owned();
                // sort managers so the auto-detected package manager is selected first
                managers.sort_by(|a, b| {
                    detected_manager
                        .map(|p| p == *b)
                        .unwrap_or(false)
                        .cmp(&detected_manager.map(|p| p == *a).unwrap_or(false))
                });
                // Skip prompt, if only one package manager is detected or explicit skip requested by `-y/--yes` flag
                if managers.len() == 1 || skip {
                    managers[0]
                } else {
                    let index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choose your package manager")
                        .items(&managers)
                        .default(0)
                        .interact()?;
                    managers[index]
                }
            } else {
                defaults.manager.context("default manager not set")?
            }
        }
    };

    let templates_no_flavors = pkg_manager.templates_no_flavors();

    // Template to render
    let template = match template {
        Some(template) => template,
        None => {
            if skip {
                defaults.template.context("default template not set")?
            } else {
                let index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose your UI template")
                    .items(
                        &templates_no_flavors
                            .iter()
                            .map(|t| t.select_text())
                            .collect::<Vec<_>>(),
                    )
                    .default(0)
                    .interact()?;

                let template = templates_no_flavors[index];

                // Prompt for flavors if the template has more than one flavor
                let flavors = template.flavors(pkg_manager);
                if let Some(flavors) = flavors {
                    let index = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choose your UI flavor")
                        .items(flavors)
                        .default(0)
                        .interact()?;
                    template.from_flavor(flavors[index])
                } else {
                    template
                }
            }
        }
    };

    // If the package manager and the template are specified on the command line
    // then almost all prompts are skipped so we need to make sure that the combination
    // is valid, otherwise, we error and exit
    if !pkg_manager.templates().contains(&template) {
        eprintln!(
            "{BOLD}{RED}error{RESET}: the {GREEN}{template}{RESET} template is not suppported for the {GREEN}{pkg_manager}{RESET} package manager\n       possible templates for {GREEN}{pkg_manager}{RESET} are: [{}]\n       or maybe you meant to use another package manager\n       possible package managers for {GREEN}{template}{RESET} are: [{}]" ,
            templates_no_flavors.iter().map(|e|format!("{GREEN}{e}{RESET}")).collect::<Vec<_>>().join(", "),
            template.possible_package_managers().iter().map(|e|format!("{GREEN}{e}{RESET}")).collect::<Vec<_>>().join(", "),
        );
        exit(1);
    }

    // Remove the target dir contents before rendering the template
    // SAFETY: Upon reaching this line, the user already accepted to overwrite
    if target_dir.exists() {
        #[inline(always)]
        fn clean_dir(dir: &std::path::PathBuf) -> anyhow::Result<()> {
            for entry in fs::read_dir(dir)?.flatten() {
                let path = entry.path();
                if entry.file_type()?.is_dir() {
                    if entry.file_name() != ".git" {
                        clean_dir(&path)?;
                        std::fs::remove_dir(path)?;
                    }
                } else {
                    fs::remove_file(path)?;
                }
            }
            Ok(())
        }
        clean_dir(&target_dir)?;
    } else {
        let _ = fs::create_dir_all(&target_dir);
    }

    // Render the template
    template.render(&target_dir, pkg_manager, &project_name, &package_name, rc)?;

    // Print post-render instructions
    println!();
    print!("Template created!");
    let has_missing = print_missing_deps(pkg_manager, template, rc);
    if has_missing {
        println!("Make sure you have installed the prerequisites for your OS: {BLUE}{BOLD}https://tauri.app/v1/guides/getting-started/prerequisites{RESET}, then run:");
    } else {
        println!(" To get started run:")
    }
    if target_dir != cwd {
        println!(
            "  cd {}",
            if project_name.contains(' ') {
                format!("\"{project_name}\"")
            } else {
                project_name
            }
        );
    }
    if let Some(cmd) = pkg_manager.install_cmd() {
        println!("  {cmd}");
    }
    if !rc {
        println!("  {} tauri dev", pkg_manager.run_cmd());
    } else {
        println!("  {} tauri android init", pkg_manager.run_cmd());
        #[cfg(target_os = "macos")]
        println!("  {} tauri ios init", pkg_manager.run_cmd());

        println!();
        println!("For Desktop development, run:");
        println!("  {} tauri dev", pkg_manager.run_cmd());
        println!();
        println!("For Android development, run:");
        println!("  {} tauri android dev", pkg_manager.run_cmd());
        #[cfg(target_os = "macos")]
        {
            println!();
            println!("For iOS development, run:");
            println!("  {} tauri ios dev", pkg_manager.run_cmd());
        }
    }
    println!();
    Ok(())
}

fn is_valid_pkg_name(project_name: &str) -> bool {
    let mut chars = project_name.chars().peekable();
    !project_name.is_empty()
        && !chars.peek().map(|c| c.is_ascii_digit()).unwrap_or_default()
        && !chars.any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_') || ch.is_uppercase())
}

fn to_valid_pkg_name(project_name: &str) -> String {
    let ret = project_name
        .trim()
        .to_lowercase()
        .replace([':', ';', ' ', '~'], "-")
        .replace(['.', '\\', '/'], "");

    let ret = ret
        .chars()
        .skip_while(|ch| ch.is_ascii_digit() || *ch == '-')
        .collect::<String>();

    if ret.is_empty() {
        "tauri-app".to_string()
    } else {
        ret
    }
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
        assert!(!is_valid_pkg_name("Tauriapp"));
    }

    #[test]
    fn converts_to_valid_pkg_name() {
        assert_eq!(to_valid_pkg_name("tauri-app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri_app"), "tauri_app");
        assert_eq!(to_valid_pkg_name("t2auriapp"), "t2auriapp");
        assert_eq!(to_valid_pkg_name("1tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("123tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("123-tauriapp"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri:app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri;app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri/app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri\\app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("tauri~app"), "tauri-app");
        assert_eq!(to_valid_pkg_name("-tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("-123tauri.app"), "tauriapp");
        assert_eq!(to_valid_pkg_name("-2-123tau2ri-app-2"), "tau2ri-app-2");
        assert_eq!(to_valid_pkg_name("1-2-3tau2ri-app2-"), "tau2ri-app2-");
        assert_eq!(to_valid_pkg_name("Tauriapp"), "tauriapp");
    }
}
