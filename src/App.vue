<script setup lang="ts">
// import HelloWorld from './components/HelloWorld.vue'
import { darkTheme } from 'naive-ui';
import EventChronicle from './components/EventChronicle.vue';
</script>

<template>
  <n-config-provider :theme="darkTheme">
    <!-- <HelloWorld /> -->
    <EventChronicle />
  </n-config-provider>
</template>


<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
