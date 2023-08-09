<script>
  export let buttons = [];
  export let active = true;
</script>

<div class="circle outer-circle flex-col justify-between p-2"
  style={active ? "border-color: blue" : "border-color: black"}>
  <div class="flex justify-center">
    {buttons[4]}
  </div>
  <div class="flex justify-between items-center">
    <div>{buttons[7]}</div>

    <div class="circle inner-circle flex-col justify-between p-2"
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

    <div>{buttons[5]}</div>
  </div>
  <div class="flex justify-center">
    {buttons[6]}
  </div>
</div>


<style>
.circle {
  border-width: 4px;
  border-style: solid;
  border-radius: 100%;
}
.outer-circle {
  height: 9rem;
  width: 9rem;
}
.inner-circle {
  height: 4.5rem;
  width: 4.5rem;
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
