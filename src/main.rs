// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input, Select};
use rust_embed::RustEmbed;
use std::{fs, path};

// order of the two variables below must match
const PKG_MANAGERS: &[&str] = &["cargo", "pnpm", "yarn", "npm"];
const PKG_MANAGERS_CONFIG_MAP: &[(&str, &str, &[&str])] = &[
    // <RUN_COMMAND> <INSTALL_COMMAND> <UI FRAGMENTS>
    ("cargo", "", &["vanilla"]),                      // CARGO
    ("pnpm", "pnpm install", NODE_JS_UI_FRAGMENTS),   // PNPM
    ("yarn", "yarn", NODE_JS_UI_FRAGMENTS),           // YARN
    ("npm run", "npm install", NODE_JS_UI_FRAGMENTS), // NPM
];
const NODE_JS_UI_FRAGMENTS: &[&str] = &[
    "vanilla",
    "vue",
    "vue-ts",
    "svelte",
    "svelte-ts",
    "react",
    "react-ts",
    "solid",
    "solid-ts",
];

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/fragments"]
struct Templates;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    let project_name = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("tauri-app".into())
        .interact_text()?;

    let target_dir = cwd.join(&project_name);

    let package_name = if is_valid_pkg_name(&project_name) {
        project_name.clone()
    } else {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Package name")
            .default(to_valid_pkg_name(&project_name))
            .with_initial_text(to_valid_pkg_name(&project_name))
            .validate_with(|input: &String| {
                if is_valid_pkg_name(&input) {
                    Ok(())
                } else {
                    Err("Package name should only include alphanumeric character and hyphens \"-\" and doesn't start with numbers")
                }
            })
            .interact_text()?
    };

    if target_dir.exists() && !target_dir.read_dir()?.next().is_none() {
        let overrwite = Confirm::with_theme(&ColorfulTheme::default())
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
            .interact()?;
        if !overrwite {
            eprintln!("{} Operation Cancelled", style("✘").red());
            std::process::exit(1);
        }
    };

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your package manager")
        .items(PKG_MANAGERS)
        .default(0)
        .interact()?;
    let (pkg_manager_run_command, pkg_manager_install_command, ui_fragments) =
        PKG_MANAGERS_CONFIG_MAP[index];

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your UI template")
        .items(ui_fragments)
        .default(0)
        .interact()?;
    let fragment = ui_fragments[index];

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
        let target_file = if p.file_name().unwrap() == "_gitignore" {
            p.parent().unwrap().join(".gitignore")
        } else {
            p
        };

        fs::create_dir_all(&target_file.parent().unwrap())?;
        fs::write(target_file, Templates::get(&*file).unwrap().data)?;
        Ok(())
    };

    // write base files first
    for file in Templates::iter().filter(|e| {
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
    for file in Templates::iter().filter(|e| {
        path::PathBuf::from(e.to_string())
            .components()
            .nth(0)
            .unwrap()
            .as_os_str()
            == path::PathBuf::from(format!("fragment-{fragment}"))
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
            .replace("{{pkg_manager_run_command}}", &pkg_manager_run_command)
    })?;

    // update Cargo.toml
    let cargo_toml = target_dir.join("src-tauri").join("Cargo.toml");
    update_file_content(&cargo_toml, |f| {
        f.replace("{{package_name}}", &package_name)
    })?;

    println!("{} Done. If you haven't already, please follow https://tauri.app/v1/guides/getting-started/prerequisites to install the needed prerequisites.", style("✔").green());
    println!("Now run:");
    println!("  cd {}", project_name);
    if !pkg_manager_install_command.is_empty() {
        println!("  {}", pkg_manager_install_command);
    }
    println!("  {} tauri dev", pkg_manager_run_command);
    println!("");

    Ok(())
}

fn update_file_content<P: AsRef<path::Path>, F: FnMut(String) -> String>(
    p: P,
    mut f: F,
) -> anyhow::Result<()> {
    let file = fs::read_to_string(&p)?;
    let file = f(file);
    fs::write(&p, file)?;
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
