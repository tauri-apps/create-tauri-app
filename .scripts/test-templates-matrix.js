const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");

function matrix(files) {
  return JSON.parse(
    execFileSync(process.execPath, [
      ".scripts/generate-templates-matrix.js",
      "pull_request",
      JSON.stringify(files),
    ]),
  );
}

const base = matrix(["templates/_base_/src-tauri/src/%(v2)%lib.rs.lte"]);
assert(base.some((job) => job.template === "react-ts" && job.specta));
assert(base.some((job) => job.template === "yew"));
const bindings = matrix(["templates/_specta_/bindings.js"]);
assert.equal(bindings.filter((job) => job.specta).length, 13);
assert.deepEqual(matrix(["unrelated.txt"]), []);
