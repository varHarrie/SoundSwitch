<script setup lang="ts">
import DeviceList from "./components/DeviceList.vue";
</script>

<template>
  <main>
    <DeviceList />
  </main>
</template>

<style>
/* Reset some basics is sufficient as Tailwind handles normalization */
body {
  margin: 0;
  padding: 0;
  background-color: #f9fafb; /* Light gray background */
}
</style>
