<script>
  import { invoke } from "@tauri-apps/api"

  let name = "";
  let greetMsg = ""

  async function greet(){
    greetMsg = await invoke("greet", { name })
  }
</script>


<div>
  <div>
    <input placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>

<style>
  button {
    font-family: inherit;
    font-size: inherit;
    color: #ff3e00;
    background-color: rgba(255, 62, 0, 0.1);
    border-radius: 2em;
    border: 2px solid rgba(255, 62, 0, 0);
    outline: none;
    cursor: pointer;
  }

  button:focus {
    border: 2px solid #ff3e00;
  }

  button:active {
    background-color: rgba(255, 62, 0, 0.2);
  }
</style>
