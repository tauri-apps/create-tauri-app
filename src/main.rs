use dialoguer::{console::style, theme::ColorfulTheme, Confirm, Input, Select};
use rust_embed::RustEmbed;
use std::{fs, path};

// order is important for the 3 variables below
const PKG_MANAGERS: &[&str] = &["cargo", "pnpm", "yarn", "npm"];
const PKG_MANAGERS_RUN_COMAND: &[&str] = &[
    "",        // placeholder
    "pnpm",    // PNPM
    "yarn",    // YARN
    "npm run", // NPM
];
const PKG_MANAGER_UI_FRAGMENTS: &[&[&str]] = &[
    &["vanilla"],         //CARGO
    NODE_JS_UI_FRAGMENTS, // PNPM
    NODE_JS_UI_FRAGMENTS, // YARN
    NODE_JS_UI_FRAGMENTS, // NPM
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

    let package_name = if is_valid_crate_name(&project_name) {
        project_name.clone()
    } else {
        Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Package/Crate name")
            .default(to_valid_pkg_name(&project_name))
            .with_initial_text(to_valid_pkg_name(&project_name))
            .validate_with(|input: &String| {
                if is_valid_crate_name(&input) {
                    Ok(())
                } else {
                    Err("Package/Crate name should only include alphanumeric character and hyphens \"-\" and doesn't start with numbers")
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
    let ui_fragments = PKG_MANAGER_UI_FRAGMENTS[index];
    let pkg_manager_run_command = PKG_MANAGERS_RUN_COMAND[index];

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
        // remove the first component, which is certainly the fragment directory they were in before getting embeded
        let p = path::PathBuf::from(file)
            .components()
            .skip(1)
            .collect::<Vec<_>>()
            .iter()
            .collect::<path::PathBuf>();
        let target_file = target_dir.join(&p);
        fs::create_dir_all(&target_file.parent().unwrap())?;
        fs::write(
            if target_file.file_name().unwrap() == "_gitignore" {
                target_file.parent().unwrap().join(".gitignore")
            } else {
                target_file
            },
            Templates::get(&*file).unwrap().data,
        )?;
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

fn is_valid_crate_name(project_name: &str) -> bool {
    !project_name
        .chars()
        .next()
        .map(|c| c.is_digit(10))
        .is_some()
        && !project_name
            .chars()
            .any(|ch| !(ch.is_alphanumeric() || ch == '-'))
}

fn to_valid_pkg_name(project_name: &str) -> String {
    project_name
        .trim()
        .to_lowercase()
        .replace(".", "")
        .replace(" ", "-")
        .replace("\\", "-")
        .replace("/", "-")
        .replace("~", "-")
}
