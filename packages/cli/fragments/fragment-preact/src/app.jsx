import { useState } from "preact/hooks";
import preactLogo from "./assets/preact.svg";
import "./app.css";

export function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <div>
        <div class="row">
          <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
          </a>
          <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo" alt="Vite logo" />
          </a>
          <a href="https://preactjs.com" target="_blank">
            <img src={preactLogo} class="logo preact" alt="Preact logo" />
          </a>
        </div>
      </div>
      <h1>Tauri + Vite + Preact</h1>
      <div class="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/app.jsx</code> and save to test HMR
        </p>
      </div>
      <p class="read-the-docs">
        Click on the Tauri, Vite and Preact logos to learn more
      </p>
    </>
  );
}

