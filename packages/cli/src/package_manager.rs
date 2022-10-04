// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{fmt::Display, str::FromStr};

use crate::template::Template;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageManager {
    Cargo,
    Pnpm,
    Yarn,
    Npm,
}

impl Default for PackageManager {
    fn default() -> Self {
        PackageManager::Cargo
    }
}

impl<'a> PackageManager {
    pub const ALL: &'a [PackageManager] = &[
        PackageManager::Cargo,
        PackageManager::Pnpm,
        PackageManager::Yarn,
        PackageManager::Npm,
    ];
}
impl PackageManager {
    pub const fn templates(&self) -> &[Template] {
        match self {
            PackageManager::Cargo => &[Template::Vanilla, Template::Yew],
            PackageManager::Pnpm | PackageManager::Yarn | PackageManager::Npm => &[
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
                Template::Next,
                Template::NextTs,
                Template::Preact,
                Template::PreactTs,
                Template::Angular,
                Template::ClojureScript,
                Template::SvelteKit,
                Template::SvelteKitTs,
            ],
        }
    }
    pub const fn install_cmd(&self) -> Option<&str> {
        match self {
            PackageManager::Pnpm => Some("pnpm install"),
            PackageManager::Yarn => Some("yarn"),
            PackageManager::Npm => Some("npm install"),
            _ => None,
        }
    }

    pub const fn run_cmd(&self) -> &str {
        match self {
            PackageManager::Cargo => "cargo",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Npm => "npm run",
        }
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Cargo => write!(f, "cargo"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Npm => write!(f, "npm"),
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
            _ => Err("Invalid package manager".to_string()),
        }
    }
}
