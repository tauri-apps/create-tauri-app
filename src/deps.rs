use template::Template;

use crate::package_manager::PackageManager;
use crate::utils::colors::*;
use crate::{args::TauriVersion, internal::template};
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

fn is_dioxus_cli_installed() -> bool {
    Command::new("dx")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn is_appropriate_tauri_cli_installed(tauri_version: TauriVersion) -> bool {
    let check = |o: Output| match o.status.success() {
        true => String::from_utf8_lossy(&o.stdout)
            .split_once(' ')
            .map(|(_, v)| v.starts_with(&tauri_version.to_string()))
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
fn is_webkit2gtk_installed(tauri_version: TauriVersion) -> bool {
    Command::new("pkg-config")
        .arg(match tauri_version {
            TauriVersion::V1 => "webkit2gtk-4.0",
            TauriVersion::V2 => "webkit2gtk-4.1",
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

fn is_dotnet_installed() -> bool {
    Command::new("dotnet")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

struct Dep<'a> {
    name: &'a str,
    instruction: String,
    exists: &'a dyn Fn() -> bool,
    skip: bool,
}

/// Print missing deps in a table and returns whether there was any missing deps.
pub fn print_missing_deps(
    pkg_manager: PackageManager,
    template: Template,
    tauri_version: TauriVersion,
) -> bool {
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
        (is_webkit2gtk_installed(tauri_version), is_rsvg2_installed());

    let deps: &[Dep<'_>] = &[
        Dep {
            name: "Rust",
            instruction: format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET}"),
            exists: &|| rustc_installed && cargo_installed,
            skip: rustc_installed || cargo_installed,
        },
        Dep  {
            name: "rustc",
            instruction: format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            exists: &|| rustc_installed,
            skip: !rustc_installed && !cargo_installed,
        },
        Dep {
            name: "Cargo",
            instruction: format!("Visit {BLUE}{BOLD}https://www.rust-lang.org/learn/get-started#installing-rust{RESET} to install Rust"),
            exists: &|| cargo_installed,
            skip: !rustc_installed && !cargo_installed,
        },
        Dep {
            name: "Tauri CLI",
            instruction: match tauri_version {
                TauriVersion::V1 => format!("Run `{BLUE}{BOLD}cargo install tauri-cli --version '^1.0.0' --locked{RESET}`"),
                TauriVersion::V2 => format!("Run `{BLUE}{BOLD}cargo install tauri-cli --version '^2.0.0' --locked{RESET}`"),
            },
            exists: &|| is_appropriate_tauri_cli_installed(tauri_version),
            skip: pkg_manager.is_node() || !template.needs_tauri_cli(),
        },
        Dep {
            name: "Trunk",
            instruction: format!("Run `{BLUE}{BOLD}cargo install trunk --locked{RESET}`"),
            exists: &is_trunk_installed,
            skip: pkg_manager.is_node() || !template.needs_trunk(),
        },
        Dep {
            name: "Dioxus CLI",
            instruction: format!("Run `{BLUE}{BOLD}cargo install dioxus-cli --locked{RESET}`"),
            exists: &is_dioxus_cli_installed,
            skip: pkg_manager.is_node() || !template.needs_dioxus_cli(),
        },
        Dep {
            name: "wasm32 target",
            instruction: format!("Run `{BLUE}{BOLD}rustup target add wasm32-unknown-unknown{RESET}`"),
            exists: &is_wasm32_installed,
            skip: pkg_manager.is_node() || !template.needs_wasm32_target(),
        },
        Dep {
            name: "Node.js",
            instruction: format!("Visit {BLUE}{BOLD}https://nodejs.org/en/{RESET}"),
            exists: &is_node_installed,
            skip: !pkg_manager.is_node(),
        },
        #[cfg(windows)]
        Dep {
            name: "Webview2",
            instruction: format!("Visit {BLUE}{BOLD}https://go.microsoft.com/fwlink/p/?LinkId=2124703{RESET}"),
            exists: &is_webview2_installed,
            skip: false,
        },
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        Dep {
            name: "webkit2gtk & rsvg2",
            instruction: format!("Visit {BLUE}{BOLD}{}{RESET}", match tauri_version {
                TauriVersion::V1 => "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux",
                TauriVersion::V2 => "https://v2.tauri.app/guides/prerequisites/#linux",
            }),
            exists: &|| webkit2gtk_installed && rsvg2_installed,
            skip: webkit2gtk_installed || rsvg2_installed,
        },
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        Dep {
            name: "webkit2gtk",
            instruction: format!("Visit {BLUE}{BOLD}{}{RESET}", match tauri_version {
                TauriVersion::V1 => "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux",
                TauriVersion::V2 => "https://v2.tauri.app/guides/prerequisites/#linux",
            }),
            exists: &|| webkit2gtk_installed,
            skip: !rsvg2_installed && !webkit2gtk_installed,
        },
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        Dep {
            name: "rsvg2",
            instruction: format!("Visit {BLUE}{BOLD}{}{RESET}", match tauri_version {
                TauriVersion::V2 => "https://v2.tauri.app/guides/prerequisites/#linux",
                TauriVersion::V1 => "https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux"
            }),
            exists: &|| rsvg2_installed,
            skip: !rsvg2_installed && !webkit2gtk_installed,
        },
        #[cfg(target_os = "macos")]
        Dep {
            name: "Xcode Command Line Tools",
            instruction: format!("Run `{BLUE}{BOLD}xcode-select --install{RESET}`"),
            exists: &is_xcode_command_line_tools_installed,
            skip: false,
        },
        Dep {
            name: ".NET",
            instruction: format!("Visit {BLUE}{BOLD}https://dotnet.microsoft.com/download{RESET}"),
            exists: &is_dotnet_installed,
            skip: !template.needs_dotnet() || pkg_manager.is_node(),
        }
    ];

    let missing_deps: Vec<(&str, &str)> = deps
        .iter()
        .filter(|dep| !dep.skip && !(dep.exists)())
        .map(|dep| (dep.name, dep.instruction.as_str()))
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

        true
    } else {
        false
    }
}
