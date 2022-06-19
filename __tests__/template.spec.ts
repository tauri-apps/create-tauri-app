import { expect, test, beforeAll, afterEach } from 'vitest'
import { readFileSync, removeSync, readdirSync } from 'fs-extra'
import type { SyncOptions, ExecaSyncReturnValue } from 'execa'
import { execaCommandSync } from 'execa'
import { join } from 'path'
import { describe } from 'vitest'

const CLI_PATH = join(__dirname, '..')

const projectName = 'template-spec-test-app'
const genPath = join(__dirname, projectName)

const run = (
  args: string[],
  options: SyncOptions<string> = {}
): ExecaSyncReturnValue => {
  return execaCommandSync(`node ${CLI_PATH} ${args.join(' ')} --ci`, options)
}

beforeAll(() => removeSync(genPath))
afterEach(() => removeSync(genPath))

const templates = [
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

readdirSync(CLI_PATH)
  .filter((e) => e.startsWith('template-'))
  .map((e) => e.replace('template-', ''))
  .forEach((template) => {
    describe(template, () => {
      test(`successfully scaffolds a project based on ${template} template`, () => {
        const { stdout } = run([projectName, '--template', template], {
          cwd: __dirname
        })
        expect(stdout).toContain(`Scaffolding project in ${genPath}`)

        const templateFiles = readdirSync(
          join(CLI_PATH, `template-${template}`)
        )
          // _gitignore is renamed to .gitignore
          .map((filePath) =>
            filePath === '_gitignore' ? '.gitignore' : filePath
          )
          .sort()
        const generatedFiles = readdirSync(genPath).sort()
        expect(templateFiles).toEqual(generatedFiles)

        const templateTauriFiles = readdirSync(
          join(CLI_PATH, `template-${template}`, 'src-tauri')
        )
          // _gitignore is renamed to .gitignore
          .map((filePath) =>
            filePath === '_gitignore' ? '.gitignore' : filePath
          )
          .sort()
        const generatedTauriFiles = readdirSync(
          join(genPath, 'src-tauri')
        ).sort()
        expect(templateTauriFiles).toEqual(generatedTauriFiles)
      })

      test('`package.json > scripts > tauri` is set', () => {
        run([projectName, '--template', template], {
          cwd: __dirname
        })

        const {
          scripts: { tauri }
        } = JSON.parse(readFileSync(join(genPath, 'package.json'), 'utf-8'))

        expect(tauri).toEqual('tauri')
      })

      test('`tauri.conf.json > build` is configured', () => {
        run([projectName, '--template', template], {
          cwd: __dirname
        })

        const {
          build: { beforeDevCommand, beforeBuildCommand, distDir, devPath }
        } = JSON.parse(
          readFileSync(join(genPath, 'src-tauri', 'tauri.conf.json'), 'utf-8')
        )

        if (template != 'vanilla') {
          expect(beforeDevCommand).toBeTruthy()
          expect(beforeBuildCommand).toBeTruthy()
        }
        expect(distDir).toBeTruthy()
        expect(devPath).toBeTruthy()
      })

      test('`tauri.conf.json > tauri > bundle > identifier` is the default identifier', () => {
        run([projectName, '--template', template], {
          cwd: __dirname
        })

        const {
          tauri: {
            bundle: { identifier }
          }
        } = JSON.parse(
          readFileSync(join(genPath, 'src-tauri', 'tauri.conf.json'), 'utf-8')
        )

        expect(identifier).toEqual('com.tauri.dev')
      })
    })
  })
