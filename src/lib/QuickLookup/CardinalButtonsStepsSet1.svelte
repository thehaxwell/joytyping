<script>
  import CardinalButtonsCircle1 from "./CardinalButtonsCircle1.svelte";
  export let activeStepNum = 1;
  export let layerIsActive = false;
  export let labels = [];
</script>


<div class="flex flex-col items-center">
<div class="flex">
      <CardinalButtonsCircle1 buttons={labels.slice(24,32)}
      active={layerIsActive && activeStepNum==3}/>
        <CardinalButtonsCircle1 buttons={labels.slice(0,8)}
        active={layerIsActive && activeStepNum==1}/>
      <CardinalButtonsCircle1 buttons={labels.slice(8,16)}
      active={layerIsActive && activeStepNum==2}/>
</div>
      <CardinalButtonsCircle1 buttons={labels.slice(16,24)}
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

