import { useState } from "preact/hooks";
import preactLogo from "./assets/preact.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./app.css";

export function App<FC>() {
  const [greetMsg, setGreetMsg] = useState<string>("");
  const [name, setName] = useState<string>("");

  const greet = async () => {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  };

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>
      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://preactjs.com" target="_blank">
          <img src={preactLogo} class="logo preact" alt="Preact logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Preact logos to learn more.</p>

      <div class="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>
      <p>{greetMsg}</p>
    </div>
  );
}
