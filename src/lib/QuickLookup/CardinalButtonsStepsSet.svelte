<script>
  import CardinalButtonsCircle from "./CardinalButtonsCircle.svelte";
  export let activeStepNum = 1;
  export let layerIsActive = false;
  export let labels = [];
</script>

<div class="flex flex-col items-center">
<div class="flex">
      <CardinalButtonsCircle buttons={labels.slice(8,12)}
      active={layerIsActive && activeStepNum==3}/>
        <CardinalButtonsCircle buttons={labels.slice(0,4)}
        active={layerIsActive && activeStepNum==1}/>
      <CardinalButtonsCircle buttons={labels.slice(4,8)}
      active={layerIsActive && activeStepNum==2}/>
</div>
      <CardinalButtonsCircle buttons={labels.slice(12,16)}
      active={layerIsActive && activeStepNum==4}/>
</div>

      <style>
.flex {
  display: flex;
}
        .flex-col {
flex-direction: column;
        }
        .items-center{
align-items: center;
        }
      </style>
