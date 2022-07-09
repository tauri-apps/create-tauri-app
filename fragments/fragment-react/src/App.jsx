import { useState } from 'react'
import logo from './logo.svg'
import { invoke } from '@tauri-apps/api'
import './App.css'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')

  async function greet() {
    setGreetMsg(await invoke('greet', { name }))
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Tauri + React!</p>

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
          Edit <code>App.jsx</code> and save to test HMR updates.
        </p>
        <p>
          Edit <code>src-tauri/src/main.rs</code> and save to test app hot
          reload.
        </p>
        <p>
          <a
            className="App-link"
            href="https://tauri.studio"
            target="_blank"
            rel="noopener noreferrer"
          >
            Tauri Docs
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
