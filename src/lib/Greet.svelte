<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }

  async function openDocs(){
    await invoke("open_quick_lookup_window");
  }
  async function close(){

    await invoke("close_quick_lookup_window");
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <button on:click={openDocs}>Open docs</button>
  <button on:click={close}>Close</button>

  <p>{greetMsg}</p>
</div>
