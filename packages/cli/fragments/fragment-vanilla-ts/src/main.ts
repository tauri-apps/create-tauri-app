import { invoke } from "@tauri-apps/api/tauri";

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

function isTauri() {
  return typeof window !== "undefined" && "__TAURI__" in window;
}

async function greet() {
  if (greetMsgEl && greetInputEl && greetInputEl.value.length > 0) {
    if (isTauri()) {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      greetMsgEl.textContent = await invoke("greet", {
        name: greetInputEl.value,
      });
      return;
    }

    greetMsgEl.textContent = `Hello, ${greetInputEl.value}! You've been greeted from TypeScript!`;
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
