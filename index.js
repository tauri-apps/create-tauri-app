#!/usr/bin/env node

// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// @ts-check
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import minimist from 'minimist'
import prompts from 'prompts'
import { bold, yellow, cyan, green } from 'kolorist'
import { red } from 'kolorist'
import { reset } from 'kolorist'

// Avoids autoconversion to number of the project name by defining that the args
// non associated with an option ( _ ) needs to be parsed as a string. See https://github.com/vitejs/vite/pull/4606
const argv = minimist(process.argv.slice(2), { string: ['_'] })
const cwd = process.cwd()

const TEMPLATES = [
  'vanilla',
  'vue',
  'vue-ts',
  'react',
  'react-ts',
  'svelte',
  'svelte-ts',
  'solid',
  'solid-ts'
]

const RENAME_FILES = {
  _gitignore: '.gitignore',
  'src-tauri/_gitignore': 'src-tauri/.gitignore'
}

async function init() {
  // prettier-ignore
  console.log(
      `
We hope to help you create something special with ${bold(yellow('Tauri'))}!
You will have a choice of one of the UI frameworks supported by the greater web tech community.
This tool should get you quickly started. See our docs at ${cyan('https://tauri.studio/')}

If you haven't already, please take a moment to setup your system.
You may find the requirements here: ${cyan('https://tauri.studio/v1/guides/getting-started/prerequisites')}
      `
    )
  await keypress(argv.ci)

  let targetDir = argv._[0]
  let template = argv.template || argv.t

  const defaultProjectName = !targetDir
    ? 'tauri-app'
    : targetDir.trim().replace(/\/+$/g, '')

  let promptsResult = {}

  try {
    promptsResult = await prompts(
      [
        {
          type: targetDir ? null : 'text',
          name: 'projectName',
          message: reset('Project name:'),
          initial: defaultProjectName,
          onState: (state) =>
            (targetDir =
              state.value.trim().replace(/\/+$/g, '') || defaultProjectName)
        },
        {
          type: () =>
            !fs.existsSync(targetDir) || isEmpty(targetDir) ? null : 'confirm',
          name: 'overwrite',
          message: () =>
            (targetDir === '.'
              ? 'Current directory'
              : `Target directory "${targetDir}"`) +
            ` is not empty. Remove existing files and continue?`
        },
        {
          type: (_prev, { overwrite }) => {
            if (overwrite === false) {
              throw new Error(red('✖') + ' Operation cancelled')
            }
            return null
          },
          name: 'overwriteChecker'
        },
        {
          type: () => (isValidPackageName(targetDir) ? null : 'text'),
          name: 'appName',
          message: reset('App name:'),
          initial: () => toValidPackageName(targetDir),
          validate: (dir) =>
            isValidPackageName(dir) || 'Invalid package.json name'
        },
        {
          type: template && TEMPLATES.includes(template) ? null : 'select',
          name: 'template',
          message:
            typeof template === 'string' && !TEMPLATES.includes(template)
              ? reset(
                  `"${template}" isn't a valid template. Please choose from below: `
                )
              : reset('Select a template:'),
          initial: 0,
          choices: TEMPLATES.map((template) => green(template))
        }
      ],
      {
        onCancel: () => {
          throw new Error(red('✖') + ' Operation cancelled')
        }
      }
    )
  } catch (cancelled) {
    console.log(cancelled.message)
    return
  }

  // user choice associated with prompts
  const { overwrite, appName } = promptsResult

  const root = path.join(cwd, targetDir)

  if (overwrite) {
    emptyDir(root)
  } else if (!fs.existsSync(root)) {
    fs.mkdirSync(root, { recursive: true })
  }

  // determine template
  template = TEMPLATES[promptsResult.template] || template

  console.log(`\nScaffolding project in ${root}...`)

  const templateDir = path.resolve(
    fileURLToPath(import.meta.url),
    '..',
    `template-${template}`
  )

  const write = (file, content) => {
    const targetPath = path.join(root, file)
    if (content) {
      fs.writeFileSync(targetPath, content)
    } else {
      copy(path.join(templateDir, file), targetPath)
    }
  }

  const files = fs.readdirSync(templateDir)
  for (const file of files.filter((f) => f !== 'package.json')) {
    write(file)
  }

  // rename files
  for (const [file, newName] of Object.entries(RENAME_FILES)) {
    fs.renameSync(path.join(root, file), path.join(root, newName))
  }

  const pkg = JSON.parse(
    fs.readFileSync(path.join(templateDir, 'package.json'), 'utf-8')
  )
  pkg.name = appName || targetDir
  write('package.json', JSON.stringify(pkg, null, 2))

  const pkgInfo = pkgFromUserAgent(process.env.npm_config_user_agent)
  const pkgManager = pkgInfo ? pkgInfo.name : 'npm'
  const pkgManagerRunCommand = ['yarn', 'pnpm'].includes(pkgManager)
    ? pkgManager
    : 'npm run'

  const tauriConf = JSON.parse(
    fs.readFileSync(
      path.join(templateDir, 'src-tauri', 'tauri.conf.json'),
      'utf-8'
    )
  )
  tauriConf.build.beforeDevCommand = tauriConf.build.beforeDevCommand.replace(
    '{{pkgManagerRunCommand}}',
    pkgManagerRunCommand
  )
  tauriConf.build.beforeBuildCommand =
    tauriConf.build.beforeBuildCommand.replace(
      '{{pkgManagerRunCommand}}',
      pkgManagerRunCommand
    )
  tauriConf.package.productName = appName || targetDir
  write(
    path.join('src-tauri', 'tauri.conf.json'),
    JSON.stringify(tauriConf, null, 2)
  )

  console.log(`\nDone. Now run:\n`)
  if (root !== cwd) {
    console.log(`  cd ${path.relative(cwd, root)}`)
  }
  switch (pkgManager) {
    case 'yarn':
      console.log('  yarn')
      console.log('  yarn tauri dev')
      break
    case 'pnpm':
      console.log('  pnpm install')
      console.log('  pnpm tauri dev')
      break
    default:
      console.log(`  ${pkgManager} install`)
      console.log(`  ${pkgManager} run tauri dev`)
      break
  }
  console.log()
}

/**
 *
 * @param {boolean} skip
 */
async function keypress(skip) {
  if (skip) return
  process.stdin.setRawMode(true)
  return await new Promise((resolve, reject) => {
    console.log('Press any key to continue...')
    process.stdin.once('data', (data) => {
      const byteArray = [...data]
      if (byteArray.length > 0 && byteArray[0] === 3) {
        console.log('^C')
        process.exit(1)
      }
      process.stdin.setRawMode(false)
      resolve()
    })
  })
}

/**
 * @param {string} projectName
 */
function isValidPackageName(projectName) {
  return /^(?:@[a-z0-9-*~][a-z0-9-*._~]*\/)?[a-z0-9-~][a-z0-9-._~]*$/.test(
    projectName
  )
}

/**
 * @param {string} projectName
 */
function toValidPackageName(projectName) {
  return projectName
    .trim()
    .toLowerCase()
    .replace(/\s+/g, '-')
    .replace(/^[._]/, '')
    .replace(/[^a-z0-9-~]+/g, '-')
}

/**
 *
 * @param {string} src
 * @param {string} dest
 */
function copy(src, dest) {
  const stat = fs.statSync(src)
  if (stat.isDirectory()) {
    copyDir(src, dest)
  } else {
    fs.copyFileSync(src, dest)
  }
}

/**
 * @param {string} srcDir
 * @param {string} destDir
 */
function copyDir(srcDir, destDir) {
  fs.mkdirSync(destDir, { recursive: true })
  for (const file of fs.readdirSync(srcDir)) {
    const srcFile = path.resolve(srcDir, file)
    const destFile = path.resolve(destDir, file)
    copy(srcFile, destFile)
  }
}

/**
 * @param {string} path
 */
function isEmpty(path) {
  const files = fs.readdirSync(path)
  return files.length === 0 || (files.length === 1 && files[0] === '.git')
}

/**
 * @param {string} dir
 */
function emptyDir(dir) {
  if (!fs.existsSync(dir)) {
    return
  }
  for (const file of fs.readdirSync(dir)) {
    fs.rmSync(path.resolve(dir, file), { recursive: true, force: true })
  }
}

/**
 * @param {string | undefined} userAgent process.env.npm_config_user_agent
 * @returns object | undefined
 */
function pkgFromUserAgent(userAgent) {
  if (!userAgent) return undefined
  const pkgSpec = userAgent.split(' ')[0]
  const pkgSpecArr = pkgSpec.split('/')
  return {
    name: pkgSpecArr[0],
    version: pkgSpecArr[1]
  }
}

init().catch((e) => {
  console.error(e)
})
