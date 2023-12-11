<script >
  import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';
  // __ERROR_MESSAGE__ is initialized through
  // https://docs.rs/tauri/latest/tauri/window/struct.WindowBuilder.html#method.initialization_script

//  let message_example = `
//Settings file not parsable: TOML parse error at line 11, column 1
//   |
//11 | [profiles.right_stick]
//   | ^^^^^^^^^^^^^^^^^^^^^^
//missing field \`click_thresholds\`
//  `;

  let message = window.__ERROR_MESSAGE__;
  let lines = message.split("\n");

  function startMainLoop() {
    invoke('start_main_loop_command');
  }
</script>

<h1>Failed to load settings file</h1>

{#if lines.length > 0}
<div class="data">
  {#each lines as line}
  {@html line}<br>
  {/each}
</div>
{/if}

<button on:click={startMainLoop}>Reload</button>

<style>
.data {
  word-wrap: break-word;
  white-space: pre-wrap;
  word-break: break-word;
  background-color: #c7c7c7;
  margin: 1rem;
  padding: 1rem;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
