<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  async function greet(){
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>

