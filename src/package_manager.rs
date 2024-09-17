// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{fmt::Display, str::FromStr};

use crate::{template::Template, utils::colors::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[derive(Default)]
pub enum PackageManager {
    #[default]
    Cargo,
    Pnpm,
    Yarn,
    Npm,
    Bun,
    Dotnet,
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Cargo => write!(f, "cargo"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Bun => write!(f, "bun"),
            PackageManager::Dotnet => write!(f, "dotnet"),
        }
    }
}

impl FromStr for PackageManager {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(PackageManager::Cargo),
            "pnpm" => Ok(PackageManager::Pnpm),
            "yarn" => Ok(PackageManager::Yarn),
            "npm" => Ok(PackageManager::Npm),
            "bun" => Ok(PackageManager::Bun),
            "dotnet" => Ok(PackageManager::Dotnet),
            _ => Err(format!(
                "{YELLOW}{s}{RESET} is not a valid package manager. Valid package mangers are [{}]",
                PackageManager::ALL
                    .iter()
                    .map(|e| format!("{GREEN}{e}{RESET}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
}

impl<'a> PackageManager {
    pub const ALL: &'a [PackageManager] = &[
        PackageManager::Cargo,
        PackageManager::Pnpm,
        PackageManager::Yarn,
        PackageManager::Npm,
        PackageManager::Bun,
        PackageManager::Dotnet,
    ];

    /// Node.js managers
    pub const NODE: &'a [PackageManager] = &[
        PackageManager::Pnpm,
        PackageManager::Yarn,
        PackageManager::Npm,
        PackageManager::Bun,
    ];
}
impl PackageManager {
    /// Returns templates without flavors
    pub const fn templates_no_flavors(&self) -> &[Template] {
        match self {
            PackageManager::Cargo => &[
                Template::Vanilla,
                Template::Yew,
                Template::Leptos,
                Template::Sycamore,
                Template::Dioxus,
            ],
            PackageManager::Pnpm
            | PackageManager::Yarn
            | PackageManager::Npm
            | PackageManager::Bun => &[
                Template::Vanilla,
                Template::Vue,
                Template::Svelte,
                Template::React,
                Template::Solid,
                Template::Angular,
                Template::Preact,
            ],
            PackageManager::Dotnet => &[Template::Blazor],
        }
    }

    pub const fn templates(&self) -> &[Template] {
        match self {
            PackageManager::Cargo => &[
                Template::Vanilla,
                Template::Yew,
                Template::Leptos,
                Template::Sycamore,
                Template::Dioxus,
            ],
            PackageManager::Pnpm
            | PackageManager::Yarn
            | PackageManager::Npm
            | PackageManager::Bun => &[
                Template::Vanilla,
                Template::VanillaTs,
                Template::Vue,
                Template::VueTs,
                Template::Svelte,
                Template::SvelteTs,
                Template::React,
                Template::ReactTs,
                Template::Solid,
                Template::SolidTs,
                Template::Angular,
                Template::Preact,
                Template::PreactTs,
            ],
            PackageManager::Dotnet => &[Template::Blazor],
        }
    }

    pub const fn install_cmd(&self) -> Option<&str> {
        match self {
            PackageManager::Pnpm => Some("pnpm install"),
            PackageManager::Yarn => Some("yarn"),
            PackageManager::Npm => Some("npm install"),
            PackageManager::Bun => Some("bun install"),
            _ => None,
        }
    }

    pub const fn run_cmd(&self) -> &str {
        match self {
            PackageManager::Cargo => "cargo",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Npm => "npm run",
            PackageManager::Bun => "bun run",
            PackageManager::Dotnet => "cargo",
        }
    }

    pub const fn is_node(&self) -> bool {
        matches!(
            self,
            PackageManager::Pnpm | PackageManager::Yarn | PackageManager::Npm | PackageManager::Bun,
        )
    }
}
