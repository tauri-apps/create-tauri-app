import { shell } from '../shell'
import { Recipe } from '../types/recipe'

export const ngcli: Recipe = {
  shortName: 'ngcli',
  descriptiveName: {
    name: 'Angular CLI (https://angular.io/cli)',
    value: 'ng-cli'
  },
  configUpdate: ({ cfg, packageManager }) => ({
    ...cfg,
    distDir: `../dist/${cfg.appName}`,
    devPath: 'http://localhost:4200',
    beforeDevCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } start`,
    beforeBuildCommand: `${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } build`
  }),
  preInit: async ({ cwd, cfg, packageManager, ci }) => {
    await shell(
      'npx',
      [
        ci ? '--yes' : '',
        '-p',
        '@angular/cli',
        'ng',
        'new',
        cfg.appName,
        `--package-manager=${packageManager.name}`
      ],
      {
        cwd
      }
    )
  },
  postInit: async ({ packageManager, cfg }) => {
    console.log(`
    Your installation completed.

    $ cd ${cfg.appName}
    $ ${
      packageManager.name === 'npm' ? 'npm run' : packageManager.name
    } tauri dev
    `)

    return await Promise.resolve()
  }
}
