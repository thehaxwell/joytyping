<script lang=ts>
  import { invoke } from '@tauri-apps/api/tauri'
  enum Platform {
    Undetermined,
    Linux,
    Windows,
  }

  let currentPlatform: Platform = Platform.Undetermined;
  const osName = (window as any).__OS_NAME__;
  if (osName == "Linux") 
    currentPlatform = Platform.Linux;
  else if (osName == "Windows")
    currentPlatform = Platform.Windows;

  function setupSettings() {
    invoke('setup_settings_command');
  }
</script>

<div class="flex justify-center">
  <div class="w-full max-w-xl m-4">
    <h1 class="text-3xl font-bold text-center">
      Setup Joytyping to get started
    </h1>
    <p class="mt-4">Joytyping needs settings to know how to work the way you prefer. These settings are normally found at</p>
    <div class="bg-slate-600 text-white p-2 rounded w-fit">
      {#if currentPlatform == Platform.Linux}
        <span class="font-bold">/home/</span>yourusername<span class="font-bold">/.config/joytyping/user.toml</span>
      {:else if currentPlatform == Platform.Windows}
        <span class="font-bold">C:\Users\</span>yourusername<span class="font-bold">\AppData\Roaming\joytyping\user.toml</span>
      {/if}
    </div>
    <p class="my-4">It looks like no settings have been setup on this machine yet.</p>
    <p class="my-4">Press the button below to let Joytyping setup your settings for you. You can change them later.</p>
    <div class="flex justify-center">
      <button class="btn btn-primary" on:click={setupSettings}>Setup settings</button>
    </div>
  </div>
</div>

