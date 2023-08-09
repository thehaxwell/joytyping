<script>
  import CardinalButtonsStepsSet from "./CardinalButtonsStepsSet.svelte";
  export let layerNum = 1;
  export let stepNum = 1;
</script>

<div class="flex">
  <div>
    <div class="flex">
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "!","@","#","$",
            "(",")",">","<",
            "_","?","+","~",
            "","","","",
          ]}
        />
      </div>
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "T","O","N","",
            "L","C","M","R",
            "W","B","K","P",
            "Z","","","J",
          ]}
        />
      </div>
    </div>
    <div class="flex">
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "1","2","3","4",
            "9","0",".",",",
            "-","/","=","`",
            "","","","",
          ]}
         />
      </div>
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "t","o","n","⌫",
            "l","c","m","r",
            "w","b","k","p",
            "z","","","j",
          ]}
         />
      </div>
    </div>
  </div>
  <div>
    <div class="flex">
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "E","↵","I","A",
            "H","S","U","D",
            "G","F","V","Y",
            "Q","X","","",
          ]}
         />
      </div>
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "%","^","&","*",
            "\"",":","}","{",
            "Tab","|","","",
            "","","","",
          ]}
         />
      </div>
    </div>
    <div class="flex">
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==1} 
          activeStepNum={stepNum} 
          labels={[
            "e","␣","i","a",
            "h","s","u","d",
            "g","f","v","y",
            "q","x","",""
          ]}
         />
      </div>
      <div class="m-2">
        <CardinalButtonsStepsSet 
          layerIsActive={layerNum==2} 
          activeStepNum={stepNum} 
          labels={[
            "5","6","7","8",
            "'",";","]","[",
            "Tab","\\","","",
            "","","","",
          ]}
         />
      
      </div>
    </div>
  </div>
</div>

<style>
.flex {
  display: flex;
}
.m-2 {
  margin: 0.5rem;
}
</style>
