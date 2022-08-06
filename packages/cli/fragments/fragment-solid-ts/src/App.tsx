import type { Component } from "solid-js";
import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const App: Component = () => {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <div className="App">
      <div>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo" alt="Tauri logo" />
        </a>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} className="logo react" alt="Solid logo" />
        </a>
      </div>
      <div className="card">
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
        <p>{greetMsg}</p>

        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
        <p>
          Edit <code>src-tauri/src/main.rs</code> and save to test app hot
          reload.
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Tauri, Vite and Solid logos to learn more
      </p>
    </div>
  );
};

export default App;
