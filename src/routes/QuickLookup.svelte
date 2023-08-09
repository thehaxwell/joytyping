<script>
  import { listen } from '@tauri-apps/api/event'
  import Keyboard3 from "../lib/QuickLookup/Keyboard3.svelte";

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

<Keyboard3 {layerNum} {stepNum} />
