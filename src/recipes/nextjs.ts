// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { join } from 'path'
// @ts-expect-error
import scaffe from 'scaffe'
import { Recipe } from '../types/recipe'

export const nextjs: Recipe = {
  shortName: 'nextjs',
  descriptiveName: {
    name: 'Nextjs (https://nextjs.org/)',
    value: 'Nextjs'
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
    distDir: `../dist`,
    devPath: 'http://localhost:3000/',
    beforeDevCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } next:dev`,
    beforeBuildCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } build`
  }),
  preInit: async ({ cwd, cfg, answers }) => {
    const { appName, windowTitle } = cfg
    const templateDir = join(__dirname, answers?.typescript ?
      '../src/templates/nextjs/ts' : '../src/templates/nextjs/js'
    )
    const variables = {
      name: appName,
      title: windowTitle
    }

    try {
      // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access, @typescript-eslint/no-unsafe-call
      await scaffe.generate(templateDir, join(cwd, appName), {
        overwrite: true,
        variables
      })
    } catch (err) {
      console.log(err)
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
