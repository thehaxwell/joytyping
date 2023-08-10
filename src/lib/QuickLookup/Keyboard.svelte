<script>
  import LeftButtonsSet from "./LeftButtonsSet.svelte";
  import RightButtonsSet from "./RightButtonsSet.svelte";
  export let layerNum = 1;
  export let stepNum = 1;
</script>

<div class="flex">
        <LeftButtonsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "t","o","n","⌫",  "T","O","N","",
            "l","c","m","r",  "L","C","M","R",
            "w","b","k","p",  "W","B","K","P",
            "z","","","j",    "Z","","","J",
          ]}
        />
        <RightButtonsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "e","␣","i","a",  "E","↵","I","A",
            "h","s","u","d",  "H","S","U","D",
            "g","f","v","y",  "G","F","V","Y",
            "q","x","","",    "Q","X","","",
          ]}
        />
</div>
<div class="flex">
        <LeftButtonsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "1","2","3","4",  "!","@","#","$",
            "9","0",".",",",  "(",")",">","<",
            "-","/","=",      "`","_","?","+","~",
            "","","","",      "","","","",
          ]}
        />

        <RightButtonsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "5","6","7","8",  "%","^","&","*",
            "'",";","]","[",  "\"",":","}","{",
            "Tab","\\","","", "Tab","|","","",
            "","","","",      "","","","",
          ]}
        />
</div>


<style>
.flex {
  display: flex;
}
.m-2 {
  margin: 0.5rem;
}
</style>
