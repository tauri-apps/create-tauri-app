// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::{ffi::OsString, fs, process::exit};

use crate::{category::Category, colors::*, package_manager::PackageManager};

mod category;
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

    if let Some(mobile) = mobile {
        if mobile && !alpha {
            eprintln!(
                "{BOLD}{RED}error{RESET}: `{GREEN}--mobile{RESET}` option is only available if `{GREEN}--alpha{RESET}` option is also used"
            );
            exit(1);
        }
    }

    // when invoked from pnpm, it seems like pnpm forgets to end its output with a new line
    // and it obscures the first question, this ensures we are on a new line before presenting our prompts
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
                .trim()
                .to_string()
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
            .trim().to_string()
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

    let category = if args.manager.is_none() && !skip {
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

        let categories = Category::ALL.to_vec();
        let categories = categories
            .into_iter()
            .filter(|c| c.package_managers().iter().any(|p| managers.contains(p)))
            .collect::<Vec<_>>();

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

    let pkg_manager = args.manager.unwrap_or_else(|| {
        if skip {
            defaults.manager.unwrap()
        } else {
            let category = category.unwrap();
            let managers = category.package_managers();
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
            let template = templates[index];

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

    if !pkg_manager.templates_all().contains(&template) {
        eprintln!(
            "{BOLD}{RED}error{RESET}: the {GREEN}{}{RESET} template is not suppported for the {GREEN}{pkg_manager}{RESET} package manager\n       possible templates for {GREEN}{pkg_manager}{RESET} are: [{}]",
            template,
            templates.iter().map(|e|format!("{GREEN}{}{RESET}", e, GREEN = GREEN, RESET = RESET)).collect::<Vec<_>>().join(", ")
        );
        exit(1);
    }

    if target_dir.exists() {
        // safe to remove, because upon reaching this line, the user accepted to overwrite
        fs::remove_dir_all(&target_dir)?
    };
    fs::create_dir_all(&target_dir)?;

    template.render(&target_dir, pkg_manager, &package_name, alpha, mobile)?;

    println!();
    println!(
        "{ITALIC}{DIM}Please follow{DIMRESET} {BLUE}https://tauri.app/v1/guides/getting-started/prerequisites{WHITE} {DIM}to install the needed prerequisites, if you haven't already.{DIMRESET}{RESET}",
    );
    if let Some(info) = template.post_init_info(pkg_manager, alpha) {
        println!("{}", info);
    }
    println!();
    println!("Done, Now run:");
    println!(
        "  cd {}",
        if project_name.contains(' ') {
            format!("\"{}\"", project_name)
        } else {
            project_name
        }
    );
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
