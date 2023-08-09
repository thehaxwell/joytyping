<script>
  import { listen } from '@tauri-apps/api/event'
  import Keyboard2 from "../lib/QuickLookup/Keyboard2.svelte";

  let stepNum = 1;
  let layerNum = 1;
  listen('update-keyboard', (event) => {
    console.log('==>',event.payload);
    stepNum = event.payload.step;
    layerNum = event.payload.layer;
  }).then();
//});
</script>

<!-- <AsciiControllerImage labels= -->
<!--                       {["␣","⌫","e","t","a","o","i","n","↵"," ","E","T","A","O","I","N"]} -->
<!-- /> -->

<Keyboard2 {layerNum} {stepNum} />
