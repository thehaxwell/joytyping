<script>
  import { listen } from '@tauri-apps/api/event'
  import Keyboard from "../lib/QuickLookup/Keyboard.svelte";

  let stepNum = 1;
  let layerNum = 1;
  listen('update-keyboard', (event) => {
    stepNum = event.payload.step;
    layerNum = event.payload.layer;
  }).then();
//});
</script>

<!-- <AsciiControllerImage labels= -->
<!--                       {["␣","⌫","e","t","a","o","i","n","↵"," ","E","T","A","O","I","N"]} -->
<!-- /> -->

<Keyboard {layerNum} {stepNum} />
