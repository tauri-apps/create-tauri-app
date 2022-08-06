<script>
  import svelteLogo from './assets/svelte.svg'
  import Greet from './lib/Greet.svelte'
</script>

<main>
  <div>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo" alt="Tauri Logo" />
    </a>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <div class="card">
    <Greet />
  </div>

  <p class="read-the-docs">
    Click on the Tauri and Vite and Svelte logos to learn more
  </p>
</main>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
