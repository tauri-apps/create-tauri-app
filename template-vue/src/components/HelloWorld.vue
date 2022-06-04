<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

defineProps({
  msg: String
})

const greetMsg = ref('')
const name = ref('')

async function greet() {
  greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
  <h1>{{ msg }}</h1>

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

  <p>
    <a href="https://tauri.studio/" target="_blank">Tauri Documentation</a>
    |
    <a href="https://v3.vuejs.org/" target="_blank">Vue 3 Documentation</a>
  </p>

  <div>
    <input v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>
  <p>{{ greetMsg }}</p>

  <p>
    Edit
    <code>src/components/HelloWorld.vue</code> to test hot module replacement.
  </p>
  <p>
    Edit
    <code>src-tauri/src/main.rs</code> to test app hot reload.
  </p>
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
