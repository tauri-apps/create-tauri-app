import type { Component } from 'solid-js'
import { createSignal } from 'solid-js'
import { invoke } from '@tauri-apps/api/tauri'
import logo from './logo.svg'
import styles from './App.module.css'

const App: Component = () => {
  const [greetMsg, setGreetMsg] = createSignal('')
  const [name, setName] = createSignal('')

  async function greet() {
    setGreetMsg(await invoke('greet', { name: name() }))
  }

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>Hello Tauri + Solid + Typescript!</p>

        <div>
          <input
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
        <p>{greetMsg}</p>

        <p>
          Edit <code>src/App.jsx</code> and save to test HMR updates.
        </p>
        <p>
          Edit <code>src-tauri/src/main.rs</code> and save to test app hot
          reload.
        </p>

        <p>
          <a
            class={styles.link}
            className="App-link"
            href="https://tauri.studio"
            target="_blank"
            rel="noopener noreferrer"
          >
            Tauri Docs
          </a>
          {' | '}
          <a
            class={styles.link}
            href="https://github.com/solidjs/solid"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn Solid
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
