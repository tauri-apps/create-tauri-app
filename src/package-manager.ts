import { shell } from './shell'

/** Gets package manager info from a useragent string,usually from `process.env.npm_config_user_aagent` */
export function getPkgManagerFromUA(userAgent: string | undefined):
  | {
      name: string
      version: string
    }
  | undefined {
  if (!userAgent) return undefined
  const pkgSpec = userAgent.split(' ')[0]
  const pkgSpecArr = pkgSpec.split('/')
  return {
    name: pkgSpecArr[0],
    version: pkgSpecArr[1]
  }
}

export interface PackageManager {
  version: number
  name: 'yarn' | 'npm' | 'pnpm' | string
  log: boolean
  ci: boolean

  /** Install project dependencies */
  install: (options?: { cwd?: string }) => Promise<void>

  /** Add dependencies */
  add: (
    packages: string[],
    options?: { dev?: boolean; cwd?: string }
  ) => Promise<void>

  /**
   * Run a npm package main script or a script from package.json, if the pacakge manager is npm, it is always a script from package.json so you'd have to make sure the script exists
   */
  run: (
    script: string,
    args: string[],
    options?: { cwd?: string }
  ) => Promise<void>

  /**
   * Invokes create-* packages to bootstrap new projects
   *
   * @param createApp the create-* package to invoke without "create-" prefix, ex: "react-app" for "create-react-app"
   */
  create: (
    createApp: string,
    args: string[],
    options?: { cwd?: string }
  ) => Promise<void>

  // TODO:add dlx
}

export class Npm implements PackageManager {
  version: number
  name = 'npm'
  log: boolean
  ci: boolean

  constructor(version: number, options?: { ci?: boolean; log?: boolean }) {
    this.version = version
    this.log = options?.log ?? false
    this.ci = options?.ci ?? false
  }

  async install(options?: { cwd?: string }): Promise<void> {
    await shell('npm', ['install'], { cwd: options?.cwd })
  }

  async add(
    packages: string[],
    options?: { dev?: boolean; cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'npm',
      ['install', options?.dev ? '--save-dev' : '', ...packages],
      { cwd: options?.cwd },
      this.log
    )
  }

  async run(
    script: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'npm',
      ['run', script, '--', ...args],
      { cwd: options?.cwd },
      this.log
    )
  }

  async create(
    createApp: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    if (this.version >= 7) {
      await shell(
        'npm',
        ['init', createApp, this.ci ? '--yes' : '', '--', ...args],
        {
          cwd: options?.cwd
        },
        this.log
      )
    } else {
      await shell(
        'npm',
        ['init', this.ci ? '--yes' : '', createApp, ...args],
        { cwd: options?.cwd },
        this.log
      )
    }
  }
}

export class Yarn implements PackageManager {
  version: number
  name = 'yarn'
  log: boolean
  ci: boolean

  constructor(version: number, options?: { ci?: boolean; log?: boolean }) {
    this.version = version
    this.log = options?.log ?? false
    this.ci = options?.ci ?? false
  }

  async install(options?: { cwd?: string }): Promise<void> {
    await shell('yarn', ['install'], { cwd: options?.cwd })
  }

  async add(
    packages: string[],
    options?: { dev?: boolean; cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'yarn',
      ['add', options?.dev ? '-D' : '', ...packages],
      { cwd: options?.cwd },
      this.log
    )
  }

  async run(
    script: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell('yarn', [script, ...args], { cwd: options?.cwd }, this.log)
  }

  async create(
    createApp: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'yarn',
      ['create', createApp, this.ci ? '--non-interactive' : '', ...args],
      {
        cwd: options?.cwd
      },
      this.log
    )
  }
}

export class Pnpm implements PackageManager {
  version: number
  name = 'pnpm'
  log: boolean
  ci: boolean

  constructor(version: number, options?: { ci?: boolean; log?: boolean }) {
    this.version = version
    this.log = options?.log ?? false
    this.ci = options?.ci ?? false
  }

  async install(options?: { cwd?: string }): Promise<void> {
    await shell('pnpm', ['install'], { cwd: options?.cwd })
  }

  async add(
    packages: string[],
    options?: { dev?: boolean; cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'pnpm',
      ['add', options?.dev ? '-D' : '', ...packages],
      { cwd: options?.cwd },
      this.log
    )
  }

  async run(
    script: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    if (this.version >= 7) {
      await shell('pnpm', [script, ...args], { cwd: options?.cwd }, this.log)
    } else {
      await shell(
        'pnpm',
        [script, '--', ...args],
        { cwd: options?.cwd },
        this.log
      )
    }
  }

  async create(
    createApp: string,
    args: string[],
    options?: { cwd?: string | undefined } | undefined
  ): Promise<void> {
    await shell(
      'pnpm',
      ['create', createApp, '--', ...args],
      {
        cwd: options?.cwd
      },
      this.log
    )
  }
}
