<script>
  import AsciiControllerImage from "../lib/QuickLookup/AsciiControllerImage.svelte";
let labels = {
  l1s1: ["␣","⌫","e","t","a","o","i","n","↵"," ","E","T","A","O","I","N"],
  l1s2: ["s","r","h","l","d","c","u","m","S","R","H","L","D","C","U","M"],
  l1s3: ["f","p","g","w","y","b","v","k","F","P","G","W","Y","B","V","K"],
  l1s4: ["x","j","q","z"," "," "," "," ","X","J","Q","Z"," "," "," "," "],
};

</script>

<AsciiControllerImage labels={labels.l1s1}/>
