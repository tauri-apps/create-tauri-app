use template::Template;

use crate::colors::*;
use crate::internal::template;
use crate::package_manager::PackageManager;
use std::process::Command;

fn is_rustc_installed() -> bool {
    Command::new("rustc").arg("-V").output().is_ok()
}
fn is_cargo_installed() -> bool {
    Command::new("cargo").arg("-V").output().is_ok()
}
fn is_node_installed() -> bool {
    Command::new("node").arg("-v").output().is_ok()
}

fn is_trunk_installed() -> bool {
    Command::new("trunk").arg("-V").output().is_ok()
}
fn is_tauri_cli_installed() -> bool {
    Command::new("cargo")
        .arg("tauri")
        .arg("-V")
        .output()
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stderr);
            !s.starts_with("error:")
        })
        .unwrap_or(false)
}
fn is_wasm32_installed() -> bool {
    Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|o| {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains("wasm32-unknown-unknown")
        })
        .unwrap_or(false)
}
// fn is_pnpm_installed() -> bool {
//     Command::new("pnpm").arg("-v").output().is_ok()
// }
// fn is_yarn_installed() -> bool {
//     Command::new("yarn").arg("-v").output().is_ok()
// }
// fn is_npm_installed() -> bool {
//     Command::new("npm").arg("-v").output().is_ok()
// }
// fn is_fnm_installed() -> bool {
//     Command::new("fnm").arg("-V").output().is_ok()
// }
// fn is_nvm_installed() -> bool {
//     #[cfg(windows)]
//     return Command::new("nvm").arg("version").output().is_ok();
//     #[cfg(not(windows))]
//     Command::new("nvm").arg("-v").output().is_ok()
// }

pub fn print_missing_deps(pkg_manager: PackageManager, template: Template, alpha: bool) {
    let rustc_installed = is_rustc_installed();
    let cargo_installed = is_cargo_installed();
    let deps: &[(&str, String, &dyn Fn() -> bool, bool)] = &[
        (
            "Rust",
            format!("Visit {BLUE}https://www.rust-lang.org/learn/get-started#installing-rust{RESET}"),
            &|| rustc_installed && cargo_installed,
            rustc_installed || cargo_installed,
        ),
        (
            "rustc",
            format!("Visit {BLUE}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            &|| rustc_installed,
            !rustc_installed && !cargo_installed,
        ),
        (
            "Cargo",
            format!("Visit {BLUE}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            &|| cargo_installed,
            !rustc_installed && !cargo_installed,
        ),
        (
            "Tauri CLI",
            if alpha {
                format!("Run `{BLUE}cargo install tauri-cli --version 2.0.0-alpha.2{RESET}`")
            } else {
                format!("Run `{BLUE}cargo install tauri-cli{RESET}`")
            },
            &is_tauri_cli_installed,
            pkg_manager.is_node() || !template.needs_tauri_cli(),
        ),
        (
            "Trunk",
            if alpha {
                format!("Run `{BLUE}cargo install trunk --git https://github.com/amrbashir/trunk{RESET}`")
            } else {
                format!("Visit {BLUE}https://trunkrs.dev/#install{RESET}")
            },
            &is_trunk_installed,
            pkg_manager.is_node() || !template.needs_trunk(),
        ),
        (
            "wasm32 target",
            format!("Run `{BLUE}rustup target add wasm32-unknown-unknown{RESET}`"),
            &is_wasm32_installed,
            pkg_manager.is_node() || !template.needs_wasm32_target(),
        ),
        (
            "Node.js",
            format!("Visit {BLUE}https://nodejs.org/en/{RESET}"),
            &is_node_installed,
            !pkg_manager.is_node(),
        ),
        // (
        //     "pnpm",
        //     format!("Visit {BLUE}https://pnpm.io/{RESET}"),
        //     &|| is_pnpm_installed(),
        //      !pkg_manager.is_node() || pkg_manager != PackageManager::Pnpm || is_fnm_installed() || is_nvm_installed(),
        // ),
        // (
        //     "yarn",
        //     format!("Visit {BLUE}https://yarnpkg.com/getting-started/install{RESET}."),
        //     &|| is_yarn_installed(),
        //      !pkg_manager.is_node() || pkg_manager != PackageManager::Yarn || is_fnm_installed() || is_nvm_installed(),
        // ),
        // (
        //     "npm",
        //     format!("Visit {BLUE}https://nodejs.org/en/{RESET} to install Node.js"),
        //     &|| is_npm_installed(),
        //      !pkg_manager.is_node() || pkg_manager != PackageManager::Npm || is_fnm_installed() || is_nvm_installed(),
        // ),
    ];

    let missing_deps: Vec<(String, String)> = deps
        .iter()
        .filter(|(_, _, exists, skip)| !skip && !exists())
        .map(|(s, d, _, _)| (s.to_string(), d.clone()))
        .collect();

    let (largest_first_cell, largest_second_cell) =
        missing_deps
            .iter()
            .fold((0, 0), |(mut prev_f, mut prev_s), (f, s)| {
                let f_len = f.len();
                if f_len > prev_f {
                    prev_f = f_len;
                }

                let s_len = remove_colors(s).len();
                if s_len > prev_s {
                    prev_s = s_len;
                }

                (prev_f, prev_s)
            });

    if !missing_deps.is_empty() {
        println!(
            " but your system is missing \nsome dependencies or they do not exist in {YELLOW}PATH{RESET}"
        );
        for (index, (name, instruction)) in missing_deps.iter().enumerate() {
            if index == 0 {
                println!(
                    "╭{}┬{}╮",
                    "─".repeat(largest_first_cell + 2),
                    "─".repeat(largest_second_cell + 2)
                );
            } else {
                println!(
                    "├{}┼{}┤",
                    "─".repeat(largest_first_cell + 2),
                    "─".repeat(largest_second_cell + 2)
                );
            }
            println!(
                "│ {YELLOW}{BOLD}{name}{RESET}{} │ {instruction}{} │",
                " ".repeat(largest_first_cell - name.len()),
                " ".repeat(largest_second_cell - remove_colors(instruction).len()),
            );
        }
        println!(
            "╰{}┴{}╯",
            "─".repeat(largest_first_cell + 2),
            "─".repeat(largest_second_cell + 2),
        );
        println!("Follow {BLUE}https://tauri.app/v1/guides/getting-started/prerequisites{RESET} to learn more about Tauri prerequisites");
        println!("then run:")
    } else {
        println!(" now run:")
    }
}
