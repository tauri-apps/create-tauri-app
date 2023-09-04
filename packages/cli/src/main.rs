// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{env::args_os, ffi::OsStr, path::Path};

fn main() {
    let mut args = args_os().peekable();
    let mut is_cargo = false;
    let bin_name = match args
        .next()
        .as_deref()
        .map(Path::new)
        .and_then(Path::file_stem)
        .and_then(OsStr::to_str)
    {
        Some("cargo-create-tauri-app") => {
            is_cargo = true;
            if args.peek().and_then(|s| s.to_str()) == Some("create-tauri-app") {
                // remove the extra cargo subcommand
                args.next();
                Some("cargo create-tauri-app".into())
            } else {
                Some("cargo-create-tauri-app".into())
            }
        }
        Some(stem) => Some(stem.to_string()),
        None => None,
    };

    create_tauri_app::run(
        args,
        bin_name,
        if is_cargo { Some("cargo".into()) } else { None },
    );
}
