import { commands } from "./bindings.js";

let greetInputEl;
let greetMsgEl;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at {% if v2 %}https://tauri.app/develop/calling-rust/{% else %}https://v1.tauri.app/v1/guides/features/command{% endif %}
    greetMsgEl.textContent = await commands.greet(greetInputEl.value);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
