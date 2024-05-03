<template>
  <div>
    <h3>
      Welcome to Q3Lab, click the picture to toggle the menu wenn you ready to
      start
    </h3>
    <div class="col">
      <img class="imgagebackground" src="../../assets/Welcome.png"/>
    </div>
  </div>
</template>
<script lang="ts">
 import {defineComponent} from "vue";
 export default defineComponent({
   name: "Welcome",
 });
</script>

<style scoped>
 .imgagebackground {
   width: 100%;
   height: auto;
   max-width: 100%;
   display: block; 
 }
</style>
