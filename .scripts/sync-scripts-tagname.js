#!/usr/bin/env node

// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/*
This script is solely intended to be run as part of the `covector version` step to
keep the tagName in `create-tauri-app.{ps1,sh}` up to date with other version bumps.
*/

const { readFileSync, writeFileSync } = require("fs");
const { join } = require("path");

const bump = process.argv[2];

const inc = (content) => {
  const re = new RegExp(
    /(.*__TAG_NAME__\s*=\s*("|')create-tauri-app-v)([0-9]+)\.([0-9]+)\.([0-9]+)(-([a-zA-z]+\.([0-9]+)))?(("|').*)/,
    "s",
  );
  const [, before, , major, minor, patch, preStr, , pre, after] =
    re.exec(content);

  let ret;
  switch (bump) {
    case "major":
      ret = `${before}${Number(major) + 1}.0.0${after}`;
      break;
    case "minor":
      ret = `${before}${major}.${Number(minor) + 1}.0${after}`;
      break;
    case "patch":
      ret = `${before}${major}.${minor}.${Number(patch) + 1}${after}`;
      break;
    default:
      throw new Error("unexpected bump " + bump);
  }
  return ret;
};

for (let file of [
  join(__dirname, "../create-tauri-app.ps1"),
  join(__dirname, "../create-tauri-app.sh"),
]) {
  const content = readFileSync(file, "utf-8");
  const updatedContent = inc(content);
  writeFileSync(file, updatedContent);
  console.log(`updated __TAG_NAME__  version in "${file}"`);
}
