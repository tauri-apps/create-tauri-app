use template::Template;

use crate::colors::*;
use crate::internal::template;
use crate::package_manager::PackageManager;
use std::process::{Command, Output};

fn is_rustc_installed() -> bool {
    Command::new("rustc")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
fn is_cargo_installed() -> bool {
    Command::new("cargo")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
fn is_node_installed() -> bool {
    Command::new("node")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn is_trunk_installed() -> bool {
    Command::new("trunk")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
fn is_appropriate_tauri_cli_installed(alpha: bool) -> bool {
    let check = |o: Output| match o.status.success() {
        true if alpha => String::from_utf8_lossy(&o.stderr)
            .split_once(' ')
            .map(|(_, v)| v.starts_with('2'))
            .unwrap_or(false),
        s => s,
    };
    Command::new("cargo")
        .args(["tauri", "-V"])
        .output()
        .map(check)
        .or_else(|_| Command::new("tauri").arg("-V").output().map(check))
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
#[cfg(windows)]
fn is_webview2_installed() -> bool {
    let powershell_path = std::env::var("SYSTEMROOT").map_or_else(
        |_| "powershell.exe".to_string(),
        |p| format!("{p}\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"),
    );
    // check 64bit per-system installation
    let output = Command::new(&powershell_path)
          .args(["-NoProfile", "-Command"])
          .arg("Get-ItemProperty -Path 'HKLM:\\SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' | ForEach-Object {$_.pv}")
          .output().map(|o|o.status.success());
    if let Ok(o) = output {
        if o {
            return true;
        }
    }
    // check 32bit per-system installation
    let output = Command::new(&powershell_path)
            .args(["-NoProfile", "-Command"])
            .arg("Get-ItemProperty -Path 'HKLM:\\SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' | ForEach-Object {$_.pv}")
            .output().map(|o|o.status.success());
    if let Ok(o) = output {
        if o {
            return true;
        }
    }
    // check per-user installation
    let output = Command::new(&powershell_path)
          .args(["-NoProfile", "-Command"])
          .arg("Get-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' | ForEach-Object {$_.pv}")
          .output().map(|o|o.status.success());
    if let Ok(o) = output {
        if o {
            return true;
        }
    }

    false
}
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn is_webkit2gtk_installed(alpha: bool) -> bool {
    Command::new("pkg-config")
        .arg(if alpha {
            "webkit2gtk-4.1"
        } else {
            "webkit2gtk-4.0"
        })
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
fn is_rsvg2_installed() -> bool {
    Command::new("pkg-config")
        .arg("librsvg-2.0")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn is_xcode_command_line_tools_installed() -> bool {
    Command::new("xcode-select")
        .arg("-p")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn print_missing_deps(pkg_manager: PackageManager, template: Template, alpha: bool) {
    let rustc_installed = is_rustc_installed();
    let cargo_installed = is_cargo_installed();

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    let (webkit2gtk_installed, rsvg2_installed) =
        (is_webkit2gtk_installed(alpha), is_rsvg2_installed());

    let deps: &[(&str, String, &dyn Fn() -> bool, bool)] = &[
        (
            "Rust",
            format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET}"),
            &|| rustc_installed && cargo_installed,
            rustc_installed || cargo_installed,
        ),
        (
            "rustc",
            format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            &|| rustc_installed,
            !rustc_installed && !cargo_installed,
        ),
        (
            "Cargo",
            format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            &|| cargo_installed,
            !rustc_installed && !cargo_installed,
        ),
        (
            "Tauri CLI",
            if alpha {
                format!("Run `{BLUE}{BOLD}cargo install tauri-cli --version '^2.0.0-alpha'{RESET}`")
            } else {
                format!("Run `{BLUE}{BOLD}cargo install tauri-cli{RESET}`")
            },
            &|| is_appropriate_tauri_cli_installed(alpha),
            pkg_manager.is_node() || !template.needs_tauri_cli(),
        ),
        (
            "Trunk",
            format!("Run `{BLUE}{BOLD}cargo install trunk{RESET}`"),
            &is_trunk_installed,
            pkg_manager.is_node() || !template.needs_trunk(),
        ),
        (
            "wasm32 target",
            format!("Run `{BLUE}{BOLD}rustup target add wasm32-unknown-unknown{RESET}`"),
            &is_wasm32_installed,
            pkg_manager.is_node() || !template.needs_wasm32_target(),
        ),
        (
            "Node.js",
            format!("Visit {BLUE}{BOLD}https://nodejs.org/en/{RESET}"),
            &is_node_installed,
            !pkg_manager.is_node(),
        ),
        #[cfg(windows)]
        (
            "Webview2",
            format!("Visit {BLUE}{BOLD}https://go.microsoft.com/fwlink/p/?LinkId=2124703{RESET}"),
            &is_webview2_installed,
            false,
        ),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        (
            "webkit2gtk & rsvg2",
            format!("Visit {BLUE}{BOLD}{}{RESET}", if alpha {
                "https://next--tauri.netlify.app/next/guides/getting-started/prerequisites/linux#1-system-dependencies"
            } else {
                "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux"
            }),
            &|| webkit2gtk_installed && rsvg2_installed,
            webkit2gtk_installed || rsvg2_installed,
        ),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        (
            "webkit2gtk",
            format!("Visit {BLUE}{BOLD}{}{RESET}", if alpha {
                "https://next--tauri.netlify.app/next/guides/getting-started/prerequisites/linux#1-system-dependencies"
            } else {
                "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux"
            }),
            &|| webkit2gtk_installed,
            !rsvg2_installed && !webkit2gtk_installed,
        ),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        (
            "rsvg2",
            format!("Visit {BLUE}{BOLD}{}{RESET}", if alpha {
                "https://next--tauri.netlify.app/next/guides/getting-started/prerequisites/linux#1-system-dependencies"
            } else {
                "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux"
            }),
            &|| rsvg2_installed,
            !rsvg2_installed && !webkit2gtk_installed,
        ),
        #[cfg(target_os = "macos")]
        (
            "Xcode Command Line Tools",
            format!("Run `{BLUE}{BOLD}xcode-select --install{RESET}`"),
            &is_xcode_command_line_tools_installed,
            false,
        ),
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
        println!("\n\nYour system is {YELLOW}missing dependencies{RESET} (or they do not exist in {YELLOW}$PATH{RESET}):");
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
                "│ {YELLOW}{name}{RESET}{} │ {instruction}{} │",
                " ".repeat(largest_first_cell - name.len()),
                " ".repeat(largest_second_cell - remove_colors(instruction).len()),
            );
        }
        println!(
            "╰{}┴{}╯",
            "─".repeat(largest_first_cell + 2),
            "─".repeat(largest_second_cell + 2),
        );
        println!();
        println!("Make sure you have installed the prerequisites for your OS: {BLUE}{BOLD}https://tauri.app/v1/guides/getting-started/prerequisites{RESET}, then run:");
    } else {
        println!(" To get started run:")
    }
}
