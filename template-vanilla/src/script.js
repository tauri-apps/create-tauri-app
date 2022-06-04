const { invoke } = window.__TAURI__.tauri

const greetInput = document.querySelector('#greetInput')
const greetMsg = document.querySelector('#greetMsg')

async function greet() {
  greetMsg.textContent = await invoke('greet', { name: greetInput.value })
}
