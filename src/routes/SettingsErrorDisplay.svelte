<script>
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
</script>

<h1>Failed to load settings file</h1>

{#if message != null}
<div class="data">
  {#each lines as line}
  {@html line}<br>
  {/each}
</div>
{/if}

Resorting to <a href="https://docs.joytyping.com/default-settings">the Joytyping Default Settings</a>

<style>
.data {
  white-space: pre;
  background-color: #c7c7c7;
  margin: 1rem;
  padding: 1rem;
}
</style>
