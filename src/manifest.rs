use std::collections::HashMap;

use anyhow::{bail, Context};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Manifest<'a> {
    pub before_dev_command: Option<&'a str>,
    pub before_build_command: Option<&'a str>,
    pub dev_path: Option<&'a str>,
    pub dist_dir: Option<&'a str>,
    pub with_global_tauri: Option<bool>,
    pub files: HashMap<&'a str, &'a str>,
}

impl<'a> Manifest<'a> {
    pub fn parse(s: &'a str, mobile: bool) -> Result<Self, anyhow::Error> {
        let mut manifest = Self::default();

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
                            format!("parsing manifest: key is not found in line {line_number}")
                        })?
                        .trim(),
                    s.next()
                        .with_context(|| {
                            format!("parsing manifest: value is not found in line {line_number}")
                        })?
                        .trim(),
                );

                if k.is_empty() {
                    bail!("parsing manifest: key is empty in line {line_number}");
                }

                if v.is_empty() {
                    bail!("parsing manifest: value is empty in line {line_number}");
                }

                #[allow(clippy::nonminimal_bool)]
                let replace =
                    !in_files_section && (!in_mobile_section || (in_mobile_section && mobile));

                match k {
                    "beforeDevCommand" if replace => manifest.before_dev_command = Some(v),
                    "beforeBuildCommand" if replace => manifest.before_build_command = Some(v),
                    "devPath" if replace => manifest.dev_path = Some(v),
                    "distDir" if replace => manifest.dist_dir = Some(v),
                    "withGlobalTauri" if replace => manifest.with_global_tauri = Some(v.parse()?),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses() {
        let manifest_file = r#"
            # Copyright 2019-2022 Tauri Programme within The Commons Conservancy
            # SPDX-License-Identifier: Apache-2.0
            # SPDX-License-Identifier: MIT

            beforeDevCommand = npm start -- --port 1420
            beforeBuildCommand = {% pkg_manager_run_command %} build # this comment should be stripped
            devPath = http://localhost:1420

            [mobile]
            beforeBuildCommand = {% pkg_manager_run_command %} build mobile

            [files]
            tauri.svg = src/assets/tauri.svg
            styles.css = src/styles.css
        "#;

        assert_eq!(Manifest::parse(manifest_file, false).unwrap(), {
            let mut files = HashMap::new();
            files.insert("tauri.svg", "src/assets/tauri.svg");
            files.insert("styles.css", "src/styles.css");

            Manifest {
                before_dev_command: Some("npm start -- --port 1420"),
                before_build_command: Some("{% pkg_manager_run_command %} build"),
                dev_path: Some("http://localhost:1420"),
                dist_dir: None,
                with_global_tauri: None,
                files,
            }
        });

        assert_eq!(Manifest::parse(manifest_file, true).unwrap(), {
            let mut files = HashMap::new();
            files.insert("tauri.svg", "src/assets/tauri.svg");
            files.insert("styles.css", "src/styles.css");

            Manifest {
                before_dev_command: Some("npm start -- --port 1420"),
                before_build_command: Some("{% pkg_manager_run_command %} build mobile"),
                dev_path: Some("http://localhost:1420"),
                dist_dir: None,
                with_global_tauri: None,
                files,
            }
        });
    }

    #[test]
    #[should_panic]
    fn it_panics_while_parsing() {
        let manifest_file = r#"
            # Copyright 2019-2022 Tauri Programme within The Commons Conservancy
            # SPDX-License-Identifier: Apache-2.0
            # SPDX-License-Identifier: MIT

            beforeDevCommand = npm start -- --port 1420
            beforeBuildCommand =
            devPath = http://localhost:1420

            [mobile]
            beforeBuildCommand = {% pkg_manager_run_command %} build mobile

            [files]
            tauri.svg = src/assets/tauri.svg
            styles.css = src/styles.css
        "#;

        Manifest::parse(manifest_file, false).unwrap();
    }

    #[test]
    fn later_should_override_former() {
        let manifest_file = r#"
        # Copyright 2019-2022 Tauri Programme within The Commons Conservancy
        # SPDX-License-Identifier: Apache-2.0
        # SPDX-License-Identifier: MIT

        beforeDevCommand = npm start -- --port 1420
        beforeBuildCommand = {% pkg_manager_run_command %} build # this comment should be stripped
        devPath = http://localhost:1420
        beforeBuildCommand = {% pkg_manager_run_command %} build mobile

        [files]
        tauri.svg = src/assets/tauri.svg
        styles.css = src/styles.css
    "#;

        assert_eq!(Manifest::parse(manifest_file, false).unwrap(), {
            let mut files = HashMap::new();
            files.insert("tauri.svg", "src/assets/tauri.svg");
            files.insert("styles.css", "src/styles.css");

            Manifest {
                before_dev_command: Some("npm start -- --port 1420"),
                before_build_command: Some("{% pkg_manager_run_command %} build mobile"),
                dev_path: Some("http://localhost:1420"),
                dist_dir: None,
                with_global_tauri: None,
                files,
            }
        });
    }
}
