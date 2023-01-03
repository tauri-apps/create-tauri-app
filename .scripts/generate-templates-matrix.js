const changedFiles = process.argv.slice(2);

const nodeJsTemplates = [
  "svelte",
  "svelte-ts",
  "vue",
  "vue-ts",
  "solid",
  "solid-ts",
  "react",
  "react-ts",
  "vanilla",
  "vanilla-ts",
];

const matrixConfig = [
  {
    manager: "pnpm",
    install_cmd: "pnpm install",
    run_cmd: "pnpm",
    templates: nodeJsTemplates,
  },
  {
    manager: "yarn",
    install_cmd: "yarn",
    run_cmd: "yarn",
    templates: nodeJsTemplates,
  },
  {
    manager: "npm",
    install_cmd: "npm install",
    run_cmd: "npm run",
    templates: nodeJsTemplates,
  },
  {
    manager: "cargo",
    install_cmd: "",
    run_cmd: "cargo",
    templates: ["vanilla", "yew"],
  },
];

const outMatrix = [];
matrixConfig
  .map((e) => e.templates)
  .forEach((ts, i) => {
    let { templates, ...managerInfo } = matrixConfig[i];
    for (const t of ts) {
      if (
        changedFiles.some(
          (e) =>
            e.includes(`packages/cli/fragments/base`) ||
            e.includes(`packages/cli/fragments/fragment-${t}`) ||
            e.includes("packages/cli/src") ||
            e.includes("packages/cli/Cargo.toml") ||
            e.includes(".github/workflows/templates-test.yml")
        )
      ) {
        outMatrix.push({
          template: t,
          ...managerInfo,
        });
      }
    }
  });

console.log(JSON.stringify(outMatrix));
