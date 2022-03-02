// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import fs from 'fs/promises'
import path from 'path'
import crypto from 'crypto'
import tempDirectory from 'temp-dir'
import execa from 'execa'

let assert
const nodeVersion = process.versions.node.split('.')[0]
if (nodeVersion === '14') {
  assert = await import('assert')
} else {
  assert = await import('assert/strict')
}

const ctaBinary = path.resolve('./bin/create-tauri-app.js')

const manager = process.env.TAURI_RUN_MANAGER || 'yarn'
const recipes = process.env.TAURI_RECIPE
  ? process.env.TAURI_RECIPE.split(',')
  : [
      'vanillajs',
      'vite',
      'cra',
      'svelte',
      'solid',
      'vuecli',
      'ngcli',
      'dominator',
      'cljs'
    ]
const parallelize = process.env.TAURI_RECIPE_PARALLELIZE || false

async function main() {
  const tauriTemp = path.join(
    tempDirectory,
    `tauri_${crypto.randomBytes(16).toString('hex')}`
  )

  try {
    const appName = 'tauri-app'
    let output = {}
    for (let i = 0; i < recipes.length; i++) {
      const recipe = recipes[i]
      console.log(`::group::recipe ${recipe}`)
      console.log(`------------------ ${recipe} started -------------------`)
      const recipeFolder = path.join(tauriTemp, recipe)
      const appFolder = path.join(recipeFolder, appName)
      await fs.mkdir(recipeFolder, { recursive: true })
      console.log(`${recipeFolder} created.`)

      // runs CTA with all args set to avoid any prompts
      const runArgs = [
        ctaBinary,
        '--manager',
        manager,
        '--recipe',
        recipe,
        '--ci'
      ]
      console.log(`[running] node ${runArgs.join(' ')}`)

      let opts = []
      if (manager === 'npm') {
        opts =
          recipe == 'vuecli'
            ? ['run', 'tauri:build']
            : ['run', 'tauri', '--', 'build']
      } else if (manager === 'yarn') {
        opts = recipe == 'vuecli' ? ['tauri:build'] : ['tauri', 'build']
      }

      await execa('node', runArgs, {
        cwd: recipeFolder,
        stdio: 'inherit'
      })
      // now it is finished, assert on some things
      await assertCTAState({ recipe, appFolder, appName })

      await execa(manager, opts, {
        stdio: 'inherit',
        cwd: appFolder
      })
      // build is complete, assert on some things
      await assertTauriBuildState({ appFolder, appName })

      console.log(`------------------ ${recipe} complete -------------------`)
      console.log('::endgroup::')
      // sometimes it takes a moment to flush all of the logs
      // to the console, let things catch up here
      await sleep(1000)
    }
  } catch (e) {
    console.error(e)
    throw Error(e)
  } finally {
    console.log('\nstopping process...')
    // wait a tick for file locks to be release
    await sleep(5000)
    await fs.rm(tauriTemp, { recursive: true, force: true })
    console.log(`${tauriTemp} deleted.`)
  }
}

function sleep(duration) {
  return new Promise((resolve) => {
    setTimeout(() => resolve(), duration)
  })
}

async function assertCTAState({ recipe, appFolder, appName }) {
  const packageFileInitial = JSON.parse(
    await fs.readFile(path.join(appFolder, 'package.json'), 'utf-8')
  )
  assert.strictEqual(
    packageFileInitial.name,
    appName,
    `The package.json did not have the name "${appName}".`
  )
  if (recipe != 'vuecli') {
    assert.strictEqual(
      packageFileInitial.scripts.tauri,
      'tauri',
      `The package.json did not have the tauri script.`
    )
  }
}

async function assertTauriBuildState({ recipe, appFolder, appName }) {
  const packageFileOutput = JSON.parse(
    await fs.readFile(path.join(appFolder, 'package.json'), 'utf-8')
  )
  assert.strictEqual(
    packageFileOutput.name,
    appName,
    `The package.json did not have the name "${appName}".`
  )

  if (recipe != 'vuecli') {
    assert.strictEqual(
      packageFileOutput.scripts.tauri,
      'tauri',
      `The package.json did not have the tauri script.`
    )
  }

  const cargoFileOutput = await fs.readFile(
    path.join(appFolder, 'src-tauri', 'Cargo.toml'),
    'utf-8'
  )
  assert.strictEqual(
    cargoFileOutput.startsWith(`[package]\nname = "app"`),
    true,
    `The Cargo.toml did not have the name "app".`
  )
}

main().catch((e) => {
  console.error(e)
  process.exit(1)
})
