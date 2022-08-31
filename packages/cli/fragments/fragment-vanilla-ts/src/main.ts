import { invoke } from "@tauri-apps/api";

declare global {
  interface Window {
    greet: () => void;
  }
}

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  if (greetMsgEl && greetInputEl) {
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.greet = greet;
