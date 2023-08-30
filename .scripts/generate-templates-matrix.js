const changedFilesStr = process.argv[2];
const changedFiles = changedFilesStr.split(",");

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
  "preact",
  "preact-ts",
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
    templates: ["vanilla", "yew", "leptos"],
  },
  {
    manager: "bun",
    install_cmd: "bun install",
    run_cmd: "bun run",
    templates: nodeJsTemplates,
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
          install_trunk: ["yew", "leptos"].includes(t),
          ...managerInfo,
        });
      }
    }
  });

console.log(JSON.stringify(outMatrix));
