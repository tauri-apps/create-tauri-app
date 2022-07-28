use dialoguer::console::style;
use pico_args::Arguments;

use crate::{package_manager::PackageManager, template::Template};

const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub struct Args {
    pub project_name: Option<String>,
    pub manager: Option<PackageManager>,
    pub template: Option<Template>,
    pub skip_prompts: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            project_name: Some("tauri-app".to_string()),
            manager: Some(PackageManager::Npm),
            template: Some(Template::Vanilla),
            skip_prompts: false,
        }
    }
}

pub fn args() -> anyhow::Result<Args> {
    let mut pargs = Arguments::from_env();

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
  {GREEN}-h{RESET}, {GREEN}--help{RESET}                    Prints help information
  {GREEN}-v{RESET}, {GREEN}--version{RESET}                 Prints version information
"#,
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            authors = env!("CARGO_PKG_AUTHORS"),
            desc = env!("CARGO_PKG_DESCRIPTION"),
            managers = PackageManager::ALL
                .into_iter()
                .map(|e| style(e).green().to_string())
                .collect::<Vec<_>>()
                .join(", "),
            fragments = Template::ALL
                .into_iter()
                .map(|e| style(e).green().to_string())
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
        skip_prompts: pargs.contains(["-y", "--yes"]),
        project_name: pargs.opt_free_from_str()?,
    };

    Ok(args)
}
