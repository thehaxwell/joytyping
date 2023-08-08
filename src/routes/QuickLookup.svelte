<script>
  import AsciiControllerImage from "../lib/QuickLookup/AsciiControllerImage.svelte";
  import { listen } from '@tauri-apps/api/event'

  let stepNum = 1;
listen('update-keyboard', (event) => {
  console.log('==>',event.payload);
  stepNum = event.payload.step;
 }).then();
//});
</script>

{#if stepNum == 1}
step 1
<AsciiControllerImage labels=
                      {["␣","⌫","e","t","a","o","i","n","↵"," ","E","T","A","O","I","N"]}
/>
{:else if stepNum == 2}
step 2
<AsciiControllerImage labels=
                      {["s","r","h","l","d","c","u","m","S","R","H","L","D","C","U","M"]}
/>
{:else if stepNum == 3}
step 3
<AsciiControllerImage labels=
                      {["f","p","g","w","y","b","v","k","F","P","G","W","Y","B","V","K"]}
/>
{:else}
step 4
<AsciiControllerImage labels=
                      {["x","j","q","z"," "," "," "," ","X","J","Q","Z"," "," "," "," "]}
/>
{/if}
