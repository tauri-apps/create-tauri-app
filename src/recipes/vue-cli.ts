// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { join } from 'path'
import { updatePackageJson } from '../helpers/update-package-json'
import { shell } from '../shell'
import { Recipe } from '../types/recipe'

export const vuecli: Recipe = {
  shortName: 'vuecli',
  descriptiveName: {
    name: 'Vue CLI (https://cli.vuejs.org/)',
    value: 'vue-cli'
  },
  configUpdate: ({ cfg }) => cfg,
  preInit: async ({ cwd, cfg, ci, packageManager }) => {
    await shell(
      'npx',
      [
        ci ? '--yes' : '',
        '@vue/cli@latest',
        'create',
        cfg.appName,
        '-m',
        packageManager.name,
        ci ? '--default' : ''
      ],
      { cwd }
    )

    await shell(
      'npx',
      [
        ci ? '--yes' : '',
        '@vue/cli',
        'add',
        'tauri',
        '--appName',
        cfg.appName,
        '--windowTitle',
        `${cfg.windowTitle}`
      ],
      {
        cwd: join(cwd, cfg.appName)
      }
    )

    updatePackageJson((pkg) => {
      return {
        ...pkg,
        scripts: {
          ...pkg.scripts,
          'tauri:build': 'vue-cli-service tauri:build',
          'tauri:dev': 'vue-cli-service tauri:dev'
        }
      }
    }, join(cwd, cfg.appName))
  },
  postInit: async ({ cfg, packageManager }) => {
    console.log(`
    Your installation completed.

    $ cd ${cfg.appName}
    $ ${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } tauri:serve
    `)
    return await Promise.resolve()
  }
}
