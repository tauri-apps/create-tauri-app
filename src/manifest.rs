use std::collections::HashMap;

use anyhow::{bail, Context};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Manifest<'a> {
    pub before_dev_command: Option<&'a str>,
    pub before_build_command: Option<&'a str>,
    pub dev_url: Option<&'a str>,
    pub frontend_dist: Option<&'a str>,
    pub with_global_tauri: Option<bool>,
    pub files: HashMap<&'a str, &'a str>,
}

impl<'a> Manifest<'a> {
    pub fn parse(s: &'a str) -> Result<Self, anyhow::Error> {
        let mut manifest = Self::default();

        let mut in_files_section = false;

        for (i, line) in s.split('\n').enumerate() {
            let line_number = i + 1;

            // ignore the comment portion of the line
            let line = line.split('#').next().unwrap().trim();

            if line.is_empty() {
                continue;
            }

            if line == "[files]" {
                in_files_section = true;
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

                match k {
                    "beforeDevCommand" => manifest.before_dev_command = Some(v),
                    "beforeBuildCommand" => manifest.before_build_command = Some(v),
                    "devUrl" => manifest.dev_url = Some(v),
                    "frontendDist" => manifest.frontend_dist = Some(v),
                    "withGlobalTauri" => manifest.with_global_tauri = Some(v.parse()?),
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
            devUrl = http://localhost:1420

            [files]
            tauri.svg = src/assets/tauri.svg
            styles.css = src/styles.css
        "#;

        assert_eq!(Manifest::parse(manifest_file).unwrap(), {
            let mut files = HashMap::new();
            files.insert("tauri.svg", "src/assets/tauri.svg");
            files.insert("styles.css", "src/styles.css");

            Manifest {
                before_dev_command: Some("npm start -- --port 1420"),
                before_build_command: Some("{% pkg_manager_run_command %} build"),
                dev_url: Some("http://localhost:1420"),
                frontend_dist: None,
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
            devUrl = http://localhost:1420

            [files]
            tauri.svg = src/assets/tauri.svg
            styles.css = src/styles.css
        "#;

        Manifest::parse(manifest_file).unwrap();
    }

    #[test]
    fn later_should_override_former() {
        let manifest_file = r#"
        # Copyright 2019-2022 Tauri Programme within The Commons Conservancy
        # SPDX-License-Identifier: Apache-2.0
        # SPDX-License-Identifier: MIT

        beforeDevCommand = npm start -- --port 1420
        beforeBuildCommand = {% pkg_manager_run_command %} build # this comment should be stripped
        devUrl = http://localhost:1420
        beforeBuildCommand = {% pkg_manager_run_command %} build mobile

        [files]
        tauri.svg = src/assets/tauri.svg
        styles.css = src/styles.css
    "#;

        assert_eq!(Manifest::parse(manifest_file).unwrap(), {
            let mut files = HashMap::new();
            files.insert("tauri.svg", "src/assets/tauri.svg");
            files.insert("styles.css", "src/styles.css");

            Manifest {
                before_dev_command: Some("npm start -- --port 1420"),
                before_build_command: Some("{% pkg_manager_run_command %} build mobile"),
                dev_url: Some("http://localhost:1420"),
                frontend_dist: None,
                with_global_tauri: None,
                files,
            }
        });
    }
}
