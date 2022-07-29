#!/usr/bin/env node

// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/*
This script is solely intended to be run as part of the `covector version` step to
keep the tagName in `scripts/cta.{ps1,sh}` up to date with other version bumps.
*/

const { readFileSync, writeFileSync } = require("fs");

const bump = process.argv[2];

const inc = (content) => {
  const re = new RegExp(
    /(.*__TAG_NAME__\s*=\s*("|')create-tauri-app-v)([0-9])+\.([0-9])+\.([0-9])+(("|').*)/,
    "s"
  );
  const [, , , major, minor, patch] = re.exec(content);

  let replacement;
  switch (bump) {
    case "major":
      replacement = `$1${Number(major) + 1}.$4.$5$6`;
      break;
    case "minor":
      replacement = `$1$3.${Number(minor) + 1}.$5$6`;
      break;
    case "patch":
      replacement = `$1$3.$4.${Number(patch) + 1}$6`;
      break;
    default:
      throw new Error("unexpected bump " + bump);
  }
  return content.replace(re, replacement);
};

for (let file of ["scripts/cta.ps1", "scripts/cta.sh"]) {
  const content = readFileSync(file, "utf-8");
  console.log(content);
  const updatedContent = inc(content);
  writeFileSync(file, updatedContent);
  console.log(`updated __TAG_NAME__  version in "${file}"`);
}
