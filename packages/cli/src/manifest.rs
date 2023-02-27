use std::collections::HashMap;

use anyhow::{bail, Context};

#[derive(Default, Clone)]
pub struct Manifest<'a> {
    pub before_dev_command: Option<&'a str>,
    pub before_build_command: Option<&'a str>,
    pub dev_path: Option<&'a str>,
    pub dist_dir: Option<&'a str>,
    pub with_global_tauri: bool,
    pub files: HashMap<&'a str, &'a str>,
}

impl<'a> Manifest<'a> {
    pub fn parse(s: &'a str, mobile: bool) -> Result<Self, anyhow::Error> {
        let mut manifest = Manifest::default();
        let mut in_files_section = false;
        let mut in_mobile_section = false;
        for (i, line) in s.split('\n').enumerate() {
            let line_number = i + 1;

            // ignore the comment portion of the line
            let line = line.split('#').next().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            if line == "[files]" {
                in_files_section = true;
                in_mobile_section = false;
                continue;
            }

            if line == "[mobile]" {
                in_mobile_section = true;
                in_files_section = false;
                continue;
            }

            if line.contains('=') {
                let mut s = line.split('=');
                let (k, v) = (
                    s.next()
                        .with_context(|| {
                            format!("parsing manifest: key is not found in line {}", line_number)
                        })?
                        .trim(),
                    s.next()
                        .with_context(|| {
                            format!(
                                "parsing manifest: value is not found in line {}",
                                line_number
                            )
                        })?
                        .trim(),
                );

                if k.is_empty() {
                    bail!("parsing manifest: key is empty in line {}", line_number);
                }

                if v.is_empty() {
                    bail!("parsing manifest: value is empty in line {}", line_number);
                }

                let replace =
                    !in_files_section && (!in_mobile_section || (in_mobile_section && mobile));

                match k {
                    "beforeDevCommand" if replace => manifest.before_dev_command = Some(v),
                    "beforeBuildCommand" if replace => manifest.before_build_command = Some(v),
                    "devPath" if replace => manifest.dev_path = Some(v),
                    "distDir" if replace => manifest.dist_dir = Some(v),
                    "withGlobalTauri" if replace => manifest.with_global_tauri = v.parse()?,
                    _ if in_files_section => {
                        manifest.files.insert(k, v);
                    }
                    _ => {}
                }
            }
        }
        Ok(manifest)
    }
}
