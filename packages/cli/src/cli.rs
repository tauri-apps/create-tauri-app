// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::ffi::OsString;

use pico_args::Arguments;

use crate::{colors::*, package_manager::PackageManager, template::Template};

#[derive(Debug)]
pub struct Args {
    pub project_name: Option<String>,
    pub manager: Option<PackageManager>,
    pub template: Option<Template>,
    pub skip: bool,
    pub alpha: bool,
    pub mobile: Option<bool>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            project_name: Some("tauri-app".to_string()),
            manager: Some(PackageManager::Npm),
            template: Some(Template::Vanilla),
            skip: false,
            alpha: false,
            mobile: Some(false),
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
  {GREEN}-t{RESET}, {GREEN}--template <TEMPLATE>{RESET}     Specify the UI template to use [{fragments}]
  {GREEN}-y{RESET}, {GREEN}--yes{RESET}                     Skip prompts and use defaults where applicable
                    {GREEN}--alpha{RESET}                   Bootstraps a project using tauri@2.0-alpha
                    {GREEN}--mobile{RESET}                  Bootstraps a mobile project too. Only availabe with `--alpha` option.
  {GREEN}-h{RESET}, {GREEN}--help{RESET}                    Prints help information
  {GREEN}-v{RESET}, {GREEN}--version{RESET}                 Prints version information
"#,
            name = bin_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string()),
            version = env!("CARGO_PKG_VERSION"),
            authors = env!("CARGO_PKG_AUTHORS"),
            desc = env!("CARGO_PKG_DESCRIPTION"),
            managers = PackageManager::ALL
                .iter()
                .map(|e| format!("{}{}{}", GREEN, e, RESET))
                .collect::<Vec<_>>()
                .join(", "),
            fragments = Template::ALL
                .iter()
                .map(|e| format!("{}{}{}", GREEN, e, RESET))
                .collect::<Vec<_>>()
                .join(", "),
        );

        println!("{}", help);
        std::process::exit(0);
    }
    if pargs.contains(["-v", "--version"]) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let args = Args {
        manager: pargs.opt_value_from_str(["-m", "--manager"])?,
        template: pargs.opt_value_from_str(["-t", "--template"])?,
        skip: pargs.contains(["-y", "--yes"]),
        alpha: pargs.contains("--alpha"),
        mobile: if pargs.contains("--mobile") {
            Some(true)
        } else {
            None
        },
        project_name: pargs.opt_free_from_str()?,
    };

    Ok(args)
}
