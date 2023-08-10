<script lang="ts">
  import ButtonSet from "./ButtonSet.svelte";
  export let activeStepNum = 1;
  export let layerIsActive = false;
  export let labels = [];

  const SQ_LENGTH = 20;
  const SPACING = 10;
  const doubleCrossCoordinates = [
    {
      x: 5*SQ_LENGTH + SPACING/2,
      y: 0,
    },
    {
      x: 8*SQ_LENGTH + 1.5*SPACING,
      y: SQ_LENGTH + SPACING/2,
    },
    {
      x: SQ_LENGTH,
      y: 2*SQ_LENGTH + SPACING/2,
    },
    {
      x: 4*SQ_LENGTH + SPACING,
      y: 3*SQ_LENGTH + SPACING,
    }
  ];
</script>

<ButtonSet {activeStepNum} {layerIsActive} {labels}
        isLeftButtonSet={false} {doubleCrossCoordinates} />
