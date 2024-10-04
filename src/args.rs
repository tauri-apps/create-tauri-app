// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{ffi::OsString, fmt::Display, str::FromStr};

use pico_args::Arguments;

use crate::{package_manager::PackageManager, template::Template, utils::colors::*};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum TauriVersion {
    V1,
    #[default]
    V2,
}

impl TauriVersion {
    pub fn all() -> [TauriVersion; 2] {
        [TauriVersion::V1, TauriVersion::V2]
    }
}

impl Display for TauriVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "1"),
            Self::V2 => write!(f, "2"),
        }
    }
}

impl FromStr for TauriVersion {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::V1),
            "2" => Ok(Self::V1),
            _ => Err("unknown Tauri version"),
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub project_name: Option<String>,
    pub manager: Option<PackageManager>,
    pub template: Option<Template>,
    pub identifier: Option<String>,
    pub skip: bool,
    pub force: bool,
    pub tauri_version: TauriVersion,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            project_name: Some("tauri-app".to_string()),
            identifier: Some("com.tauri.dev".to_string()),
            manager: Some(PackageManager::Npm),
            template: Some(Template::Vanilla),
            skip: false,
            force: false,
            tauri_version: TauriVersion::default(),
        }
    }
}

pub fn parse(argv: Vec<OsString>, bin_name: Option<String>) -> anyhow::Result<Args> {
    let mut pargs = Arguments::from_vec(argv);

    if pargs.contains(["-h", "--help"]) {
        let help = format!(
            r#"
{GREEN}{name}{RESET} {version}
{authors}
{desc}

{YELLOW}USAGE:{RESET}
  {name} [OPTIONS] [PROJECTNAME]

{YELLOW}ARGS:{RESET}
  {GREEN}<PROJECTNAME>{RESET}                 Specify project name which is used for the directory, package.json and Cargo.toml

{YELLOW}OPTIONS:{RESET}
  {GREEN}-m{RESET}, {GREEN}--manager <MANAGER>{RESET}       Specify preferred package manager [{managers}]
  {GREEN}-t{RESET}, {GREEN}--template <TEMPLATE>{RESET}     Specify the UI template to use [{templates}]
                    {GREEN}--identifier <identifier>{RESET} Specify a unique identifier for your application
  {GREEN}-y{RESET}, {GREEN}--yes{RESET}                     Skip prompts and use defaults where applicable
  {GREEN}-f{RESET}, {GREEN}--force{RESET}                   Force create the directory even if it is not empty.
                    {GREEN}--tauri-version [1 | 2]{RESET}   Bootstrap a project using the provided Tauri version. Defaults to the latest stable release.
  {GREEN}-h{RESET}, {GREEN}--help{RESET}                    Prints help information
  {GREEN}-v{RESET}, {GREEN}--version{RESET}                 Prints version information
"#,
            name = bin_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string()),
            version = env!("CARGO_PKG_VERSION"),
            authors = env!("CARGO_PKG_AUTHORS"),
            desc = env!("CARGO_PKG_DESCRIPTION"),
            managers = PackageManager::ALL
                .iter()
                .map(|e| format!("{GREEN}{e}{RESET}"))
                .collect::<Vec<_>>()
                .join(", "),
            templates = Template::ALL
                .iter()
                .map(|e| format!("{GREEN}{e}{RESET}"))
                .collect::<Vec<_>>()
                .join(", "),
        );

        println!("{help}");
        std::process::exit(0);
    }
    if pargs.contains(["-v", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let tauri_version: Option<TauriVersion> = pargs.opt_value_from_str("--tauri-version")?;

    let args = Args {
        manager: pargs.opt_value_from_str(["-m", "--manager"])?,
        template: pargs.opt_value_from_str(["-t", "--template"])?,
        skip: pargs.contains(["-y", "--yes"]),
        force: pargs.contains(["-f", "--force"]),
        tauri_version: tauri_version.unwrap_or_default(),
        identifier: pargs.opt_value_from_str("--identifier")?,
        project_name: pargs.opt_free_from_str()?,
    };

    Ok(args)
}
