<script lang="ts">
  export let activeStepNum = 1;
  export let layerIsActive = false;
  export let labels = [];
  export let isLeftButtonSet = true;
  export let doubleCrossCoordinates = [];

  function buildCrossPoints(startX: number,startY: number,sqLength: number, labels: string[]){
    const pointsSrc = [
        [startX, startY],
        [startX+sqLength, startY],
        [startX+sqLength, startY+sqLength],
        [startX+2*sqLength, startY+sqLength],
        [startX+2*sqLength, startY+2*sqLength],
        [startX+sqLength, startY+2*sqLength],
        [startX+sqLength, startY+3*sqLength],
        [startX, startY+3*sqLength],
        [startX, startY+2*sqLength],
        [startX-sqLength, startY+2*sqLength],
        [startX-sqLength, startY+sqLength],
        [startX, startY+sqLength],
    ];

    let availableSpaceOnOneSideOfLabel = 4
    const labelsSrc = [
      {
        x: pointsSrc[pointsSrc.length-1][0] + availableSpaceOnOneSideOfLabel,
        y: pointsSrc[pointsSrc.length-1][1] - availableSpaceOnOneSideOfLabel,
        label: labels[0],
      },
      {
        x: pointsSrc[5][0] + availableSpaceOnOneSideOfLabel,
        y: pointsSrc[5][1] - availableSpaceOnOneSideOfLabel,
        label: labels[1],
      },
      {
        x: pointsSrc[7][0] + availableSpaceOnOneSideOfLabel,
        y: pointsSrc[7][1] - availableSpaceOnOneSideOfLabel,
        label: labels[2],
      },
      {
        x: pointsSrc[9][0] + availableSpaceOnOneSideOfLabel,
        y: pointsSrc[9][1] - availableSpaceOnOneSideOfLabel,
        label: labels[3],
      },
    ]

    return {
      points: pointsSrc.map((point)=>{
        return point.join(",")
      }).join(" "),
      labels: labelsSrc,
    }
  }

  const SQ_LENGTH = 20;
  function buildDoubleCross(
    startX: number,startY: number,labels: string[]){
    return [
    buildCrossPoints(isLeftButtonSet ? startX : startX+2*SQ_LENGTH,
      startY,SQ_LENGTH,labels.slice(4,8)),
    buildCrossPoints(isLeftButtonSet ? startX+2*SQ_LENGTH : startX,
      startY+SQ_LENGTH,SQ_LENGTH,labels.slice(0,4)),
    ]
  }

  const SPACING = 10;
  let polygons = [
    ...buildDoubleCross(
      doubleCrossCoordinates[0].x,
      doubleCrossCoordinates[0].y,
      labels.slice(0,8)),
    ...buildDoubleCross(
      doubleCrossCoordinates[1].x,
      doubleCrossCoordinates[1].y,
      labels.slice(8,16)),
    ...buildDoubleCross(
      doubleCrossCoordinates[2].x,
      doubleCrossCoordinates[2].y,
      labels.slice(16,24)),
    ...buildDoubleCross(
      doubleCrossCoordinates[3].x,
      doubleCrossCoordinates[3].y,
      labels.slice(24,32)),
  ];

    function stepIsActive(activeStepNum: number,index: number): boolean{
      if(activeStepNum==1 && [0,1].includes(index)){return true}
      else if(activeStepNum==2 && [2,3].includes(index)){return true}
      else if(activeStepNum==3 && [4,5].includes(index)){return true}
      else if(activeStepNum==4 && [6,7].includes(index)){return true}
      else return false;
    }
</script>

<svg viewBox="0 0 260 170" width="260px" height="170px" xmlns="http://www.w3.org/2000/svg">
{#each polygons as {points, labels},index}
  <polygon {points} 
  fill={layerIsActive &&
    stepIsActive(activeStepNum,index) ? "none":"lightgrey"}
  stroke={layerIsActive &&
    stepIsActive(activeStepNum,index) ? "blue" :"grey"}
  />
  {#each labels as {x,y,label}}
    <text x={x+""} y={y+""} fill="black">{label}</text>
  {/each}
{/each}
</svg>
