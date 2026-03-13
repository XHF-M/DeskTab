<template>
  <div id="app">
    <HotkeyHandler />
    <TopBar />
  </div>
</template>

<script setup lang="ts">
import TopBar from './components/TopBar.vue';
import HotkeyHandler from './components/HotkeyHandler.vue';
</script>

<style>
@import './styles/global.css';

/* 隐藏滚动条 */
#app {
  overflow: hidden;
}

/* 全局隐藏滚动条 */
::-webkit-scrollbar {
  width: 0;
  height: 0;
}

* {
  scrollbar-width: none;
  -ms-overflow-style: none;
}
</style>
