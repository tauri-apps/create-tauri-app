const isCron = process.argv[2] === "schedule";

const changedFilesStr = process.argv[3];
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
  "angular",
];

const matrixConfig = [
  {
    manager: "cargo",
    install_cmd: "",
    run_cmd: "cargo",
    templates: ["vanilla", "yew", "sycamore", "leptos"],
  },
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
    manager: "bun",
    install_cmd: "bun install",
    run_cmd: "bun run",
    templates: nodeJsTemplates,
  },
  {
    manager: "dotnet",
    install_cmd: "",
    run_cmd: "cargo",
    templates: ["blazor"],
  },
];

const outMatrix = [];
matrixConfig
  .map((e) => e.templates)
  .forEach((ts, i) => {
    let { templates, ...managerInfo } = matrixConfig[i];
    for (const t of ts) {
      if (
        isCron ||
        changedFiles.some(
          (e) =>
            e.startsWith(`templates/base`) ||
            e.startsWith(`templates/template-${t}`) ||
            e.startsWith("src") ||
            e.startsWith("Cargo.toml") ||
            e.startsWith(".github/workflows/templates-test.yml"),
        )
      ) {
        const jobInfo = {
          template: t,
          install_trunk: ["yew", "sycamore", "leptos"].includes(t),
          rc: false,
          no_bundle_flag: "-b none",
          ...managerInfo,
        };
        outMatrix.push(jobInfo);
        outMatrix.push({
          ...jobInfo,
          rc: true,
          no_bundle_flag: "--no-bundle",
          flags: "--rc",
        });
      }
    }
  });

console.log(JSON.stringify(outMatrix));
