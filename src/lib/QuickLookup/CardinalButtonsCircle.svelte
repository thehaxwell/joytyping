<script>
  export let buttons = [];
  export let active = false;
</script>

<div class="circle flex-col justify-between p-2"
  style={active ? "border-color: blue" : "border-color: black"}>
  <div class="flex justify-center">
    {buttons[0]}
  </div>
  <div class="flex justify-between items-center">
    <div>{buttons[3]}</div>
    <div>{buttons[1]}</div>
  </div>
  <div class="flex justify-center">
    {buttons[2]}
  </div>
</div>

<style>
.circle {
  height: 4.5rem;
  width: 4.5rem;
  border-width: 4px;
  border-style: solid;
  border-radius: 100%;
}
.flex-col {
  display: flex;
  flex-direction: column;
  /* justify-content: center;
  align-items: center; */
}
.flex {
  display: flex;
}
.justify-center{
  justify-content: center;
}
.items-center{
  align-items: center;
}
.justify-between{
  justify-content: space-between;
}
.m-2 {
  margin: 0.5rem;
}
.p-2 {
  padding: 0.5rem;
}
</style>
