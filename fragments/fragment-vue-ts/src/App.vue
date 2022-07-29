<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from './components/Greet.vue'
</script>

<template>
  <div>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo" alt="Tauri logo" />
    </a>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <Greet />
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
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
