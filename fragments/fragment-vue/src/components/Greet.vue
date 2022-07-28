<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref('')
const name = ref('')

async function greet() {
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
    <p>
    Edit
    <code>src/components/Greet.vue</code> to test hot module replacement.
  </p>
  <p>
    Edit
    <code>src-tauri/src/main.rs</code> to test app hot reload.
  </p>

  <p>
    Recommended IDE setup:
    <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
    +
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    +
    <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
      >Tauri</a
    >
    +
    <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
      >rust-analyzer</a
    >
  </p>
  <p class="read-the-docs">Click on the Tauri, Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}

#greet-input {
  margin-right: 5px;
}
</style>
