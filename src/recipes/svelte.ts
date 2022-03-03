// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { join } from 'path'
import { shell } from '../shell'
import { Recipe } from '../types/recipe'

export const svelte: Recipe = {
  shortName: 'svelte',
  descriptiveName: {
    name: 'Svelte (https://github.com/sveltejs/template)',
    value: 'svelte'
  },
  extraQuestions: ({ ci }) => [
    {
      type: 'confirm',
      name: 'typescript',
      message: 'Enable Typescript?',
      default: true,
      loop: false,
      when: !ci
    }
  ],
  configUpdate: ({ cfg, packageManager }) => ({
    ...cfg,
    distDir: `../public`,
    devPath: 'http://localhost:8080',
    beforeDevCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } dev`,
    beforeBuildCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } build`
  }),
  preInit: async ({ cwd, cfg, answers, ci }) => {
    await shell(
      'npx',
      [ci ? '--yes' : '', 'degit', 'sveltejs/template', cfg.appName],
      {
        cwd
      }
    )

    if (answers?.typescript) {
      await shell('node', ['scripts/setupTypeScript.js'], {
        cwd: join(cwd, cfg.appName)
      })
    }
  },
  postInit: async ({ cfg, packageManager }) => {
    console.log(`
    Your installation completed.

    $ cd ${cfg.appName}
    $ ${packageManager.name} install
    $ ${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } tauri dev
    `)

    return await Promise.resolve()
  }
}
