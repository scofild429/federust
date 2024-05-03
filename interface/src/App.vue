<script setup lang="ts">
 import { RouterLink, RouterView } from 'vue-router'
 import GlobalHeader from "./components/Header/GlobalHeader.vue";
 import HelloWorld from './components/HelloWorld.vue';
 import Footer from './components/Footer.vue';
 import "bootstrap/dist/css/bootstrap.min.css";
 import "bootstrap/dist/js/bootstrap.bundle.min.js";
</script>

<template>
  <div class="container">
    <GlobalHeader></GlobalHeader>
    <RouterView />
    <br/>
    <br/>
    <Footer/>
  </div>
</template>

<style scoped>

</style>
