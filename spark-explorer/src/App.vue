<!-- src/App.vue -->
<template>
  <div class="relative w-full h-screen overflow-hidden bg-black">
    <CosmicScene />
    <div class="absolute top-0 left-0 w-full h-full flex items-center justify-center">
      <h1 class="text-white text-4xl z-10">Cosmic Explorer</h1>
    </div>
  </div>
</template>

<script>
import CosmicScene from './components/CosmicScene.vue'

export default {
  name: 'App',
  components: {
    CosmicScene,
  },
}
</script>

<style>
/* 可以移除默认样式 */
</style>
