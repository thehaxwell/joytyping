<script lang=ts>
  export let labels: string[] = [];
  let x = labels.map(str => `<b>${str}</b>`);

  // Original art: https://www.asciiart.eu/computers/game-consoles
  let asciiController = `
      _=====_                             _=====_
     / _____ \\                           / _____ \\
   +.-'_____'-.-------------------------.-'_____'-.+
  /   |     |  '.                     .'  |     |   \\
 / ___| (${x[11]}) |___ \\                   / ___| (${x[10]}) |___ \\
/ |             | ;                 ; |             | ;
| | (${x[9]})     (${x[13]}) | |                 | |(${x[12]})       (${x[8]})| |
| |___       ___| ;                 ; |___       ___| ;
 \\    | (${x[15]}) |    /                   \\    | (${x[14]}) |    /
  \\   |_____|  .','(${x[3]})',       ,'(${x[2]})','.  |_____|  .'
   '-.______.-' /       \\     /       \\ '-._____.-'
               (${x[1]})     (${x[5]})---(${x[4]})     (${x[0]})
                \\       /     \\       /
                 '.(${x[7]}).'       '.(${x[6]}).'
  `;
</script>
<div id="controller">{@html asciiController}</div>
<style>
  #controller {
    font-family: monospace;
    font-size: 1rem;
    line-height: 1rem;
    white-space: pre;
  }
</style>
