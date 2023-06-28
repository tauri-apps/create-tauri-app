// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dialoguer::{Confirm, Input, Select};
use std::{ffi::OsString, fs, process::exit};

use crate::{
    category::Category, colors::*, deps::print_missing_deps, package_manager::PackageManager,
    theme::ColorfulTheme,
};

mod category;
mod cli;
mod colors;
mod deps;
mod manifest;
mod package_manager;
mod template;
mod theme;

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
    let cli::Args {
        skip,
        mobile,
        alpha,
        ..
    } = args;
    let cwd = std::env::current_dir()?;

    // Allow `--mobile` to only be used with `--alpha` for now
    // TODO: remove this limitation once tauri@v2 is stable
    if let Some(mobile) = mobile {
        if mobile && !alpha {
            eprintln!(
                "{BOLD}{RED}error{RESET}: `{GREEN}--mobile{RESET}` option is only available if `{GREEN}--alpha{RESET}` option is also used"
            );
            exit(1);
        }
    }

    // Project name used for the project directory name
    // and if valid, it will also be used in Cargo.toml, Package.json ...etc
    let project_name = args.project_name.unwrap_or_else(|| {
        if skip {
            defaults.project_name.unwrap()
        } else {
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default("tauri-app".into())
                .interact_text()
                .unwrap()
                .trim()
                .to_string()
        }
    });

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
                    Err("Package name should only include alphanumeric character and hyphens \"-\" and doesn't start with numbers")
                }
            })
            .interact_text()?
            .trim().to_string()
        }
    };

    // Confirm deleting the target project directory if not empty
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

    // Prompt for category if a package manger is not passed on the command line
    let category = if args.manager.is_none() && !skip {
        // Filter managers if a template is passed on the command line
        let managers = PackageManager::ALL.to_vec();
        let managers = args
            .template
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
        let categories = categories
            .into_iter()
            .filter(|c| c.package_managers().iter().any(|p| managers.contains(p)))
            .collect::<Vec<_>>();

        // If only one category is detected, skip prompt
        if categories.len() == 1 {
            Some(categories[0])
        } else {
            let index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose which language to use for your frontend")
                .items(&categories)
                .default(0)
                .interact()
                .unwrap();
            Some(categories[index])
        }
    } else {
        None
    };

    // Package manager which will be used for rendering the template
    // and the after-render instructions
    let pkg_manager = args.manager.unwrap_or_else(|| {
        if skip {
            defaults.manager.unwrap()
        } else {
            let category = category.unwrap();
            let managers = category.package_managers();

            // If only one package manager is detected, skip prompt
            if managers.len() == 1 {
                managers[0]
            } else {
                let index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose your package manager")
                    .items(managers)
                    .default(0)
                    .interact()
                    .unwrap();
                managers[index]
            }
        }
    });

    let templates_no_flavors = pkg_manager.templates_no_flavors();

    // Template to render
    let template = args.template.unwrap_or_else(|| {
        if skip {
            defaults.template.unwrap()
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
                .interact()
                .unwrap();
            let template = templates_no_flavors[index];

            // Prompt for flavors if the template has more than one flavor
            let flavors = template.flavors(pkg_manager);
            if let Some(flavors) = flavors {
                let index = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose your UI flavor")
                    .items(flavors)
                    .default(0)
                    .interact()
                    .unwrap();
                template.from_flavor(flavors[index])
            } else {
                template
            }
        }
    });

    // Prompt for wether to bootstrap a mobile-friendly tauri project
    // This should only be prompted if `--alpha` is used on the command line and `--mobile` wasn't.
    // TODO: remove this limitation once tauri@v2 is stable
    let mobile = mobile.unwrap_or_else(|| {
        if skip || !alpha {
            defaults.mobile.unwrap()
        } else {
            Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Would you like to setup the project for mobile as well?")
                .default(false)
                .interact()
                .unwrap()
        }
    });

    // If the package manager and the template are specified on the command line
    // then almost all prompts are skipped so we need to make sure that the combination
    // is valid, otherwise, we error and exit
    if !pkg_manager.templates().contains(&template) {
        eprintln!(
            "{BOLD}{RED}error{RESET}: the {GREEN}{}{RESET} template is not suppported for the {GREEN}{pkg_manager}{RESET} package manager\n       possible templates for {GREEN}{pkg_manager}{RESET} are: [{}]",
            template,
            templates_no_flavors.iter().map(|e|format!("{GREEN}{e}{RESET}")).collect::<Vec<_>>().join(", ")
        );
        exit(1);
    }

    // Remove the target dir contents before rendering the template
    // SAFETY: Upon reaching this line, the user already accepted to overwrite
    if target_dir.exists() {
        fn clean_dir(dir: &std::path::PathBuf) -> anyhow::Result<()> {
            for entry in fs::read_dir(dir)?.flatten() {
                let path = entry.path();
                if entry.file_type()?.is_dir() {
                    clean_dir(&path)?;
                    std::fs::remove_dir(path)?;
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
    template.render(&target_dir, pkg_manager, &package_name, alpha, mobile)?;

    // Print post-render instructions
    println!();
    print!("Template created!");
    print_missing_deps(pkg_manager, template, alpha);
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
    if !mobile {
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
    !project_name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or_default()
        && !project_name
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-' || ch == '_'))
        && !project_name.is_empty()
}

fn to_valid_pkg_name(project_name: &str) -> String {
    #[allow(clippy::collapsible_str_replace)]
    let ret = project_name
        .trim()
        .to_lowercase()
        .replace(':', "-")
        .replace(';', "-")
        .replace(' ', "-")
        .replace('~', "-")
        .replace('.', "")
        .replace('\\', "")
        .replace('/', "");

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
    }
}
