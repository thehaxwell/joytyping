<script>
  import CardinalButtonsRow from "./CardinalButtonsRow.svelte";
  export let layerNum = 1;
  export let stepNum = 1;
</script>

<div class="flex">
  <div>
    <div class="flex">
      <div class="flex-col m-2">
        <CardinalButtonsRow buttons={["!","@","#","$"]}
        active={layerNum==2 && stepNum==1}/>
      <CardinalButtonsRow buttons={["(",")",">","<"]}
      active={layerNum==2 && stepNum==2}/>
      <CardinalButtonsRow buttons={["_","?","+","~"]}
      active={layerNum==2 && stepNum==3}/>
      <CardinalButtonsRow buttons={["","","",""]}
      active={layerNum==2 && stepNum==4}/>
      </div>
      <div class="flex-col m-2">
        <CardinalButtonsRow buttons={["T","O","N",""]}
        active={layerNum==1 && stepNum==1}/>
      <CardinalButtonsRow buttons={["L","C","M","R"]}
      active={layerNum==1 && stepNum==2}/>
      <CardinalButtonsRow buttons={["W","B","K","P"]}
      active={layerNum==1 && stepNum==3}/>
      <CardinalButtonsRow buttons={["Z","","","J"]}
      active={layerNum==1 && stepNum==4}/>
      </div>
    </div>
    <div class="flex">
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["1","2","3","4"]}
      active={layerNum==2 && stepNum==1}/>
      <CardinalButtonsRow buttons={["9","0",".",","]}
      active={layerNum==2 && stepNum==2}/>
      <CardinalButtonsRow buttons={["-","/","=","`"]}
      active={layerNum==2 && stepNum==3}/>
      <CardinalButtonsRow buttons={["","","",""]}
      active={layerNum==2 && stepNum==4}/>
      </div>
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["t","o","n","⌫"]}
      active={layerNum==1 && stepNum==1}/>
      <CardinalButtonsRow buttons={["l","c","m","r"]}
      active={layerNum==1 && stepNum==2}/>
      <CardinalButtonsRow buttons={["w","b","k","p"]}
      active={layerNum==1 && stepNum==3}/>
      <CardinalButtonsRow buttons={["z","","","j"]}
      active={layerNum==1 && stepNum==4}/>
      </div>
    </div>
  </div>
  <div>
    <div class="flex">
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["E","↵","I","A"]}
      active={layerNum==1 && stepNum==1}/>
      <CardinalButtonsRow buttons={["H","S","U","D"]}
      active={layerNum==1 && stepNum==2}/>
      <CardinalButtonsRow buttons={["G","F","V","Y"]}
      active={layerNum==1 && stepNum==3}/>
      <CardinalButtonsRow buttons={["Q","X","",""]}
      active={layerNum==1 && stepNum==4}/>
      </div>
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["%","^","&","*"]}
      active={layerNum==2 && stepNum==1}/>
      <CardinalButtonsRow buttons={["\"",":","}","{"]}
      active={layerNum==2 && stepNum==2}/>
      <CardinalButtonsRow buttons={["Tab","|","",""]}
      active={layerNum==2 && stepNum==3}/>
      <CardinalButtonsRow buttons={["","","",""]}
      active={layerNum==2 && stepNum==4}/>
      </div>
    </div>
    <div class="flex">
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["e","␣","i","a"]}
      active={layerNum==1 && stepNum==1}/>
      <CardinalButtonsRow buttons={["h","s","u","d"]}
      active={layerNum==1 && stepNum==2}/>
      <CardinalButtonsRow buttons={["g","f","v","y"]}
      active={layerNum==1 && stepNum==3}/>
      <CardinalButtonsRow buttons={["q","x","",""]}
      active={layerNum==1 && stepNum==4}/>
      </div>
      <div class="flex-col m-2">
      <CardinalButtonsRow buttons={["5","6","7","8"]}
      active={layerNum==2 && stepNum==1}/>
      <CardinalButtonsRow buttons={["'",";","]","["]}
      active={layerNum==2 && stepNum==2}/>
      <CardinalButtonsRow buttons={["Tab","\\","",""]}
      active={layerNum==2 && stepNum==3}/>
      <CardinalButtonsRow buttons={["","","",""]}
      active={layerNum==2 && stepNum==4}/>
      </div>
    </div>
  </div>
</div>

<style>
.flex-col {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.flex {
  display: flex;
}
.m-2 {
  margin: 0.5rem;
}
</style>
