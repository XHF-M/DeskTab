<template>
  <!-- 这个组件不渲染任何内容，只用于处理快捷键事件 -->
  <div style="display: none;"></div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { useWindowStore } from '../stores/windowStore';

let unlistenToggle: (() => void) | null = null;
let unlistenShow: (() => void) | null = null;

onMounted(async () => {
  const windowStore = useWindowStore();

  // 监听切换快捷键 (Ctrl+Alt+D)
  try {
    unlistenToggle = await listen('global-shortcut-toggle', () => {
      windowStore.toggle();
    });
  } catch (error) {
    console.error('Failed to listen for toggle shortcut:', error);
  }

  // 监听显示快捷键 (Alt+`)
  try {
    unlistenShow = await listen('global-shortcut-show', () => {
      if (!windowStore.isExpanded) {
        windowStore.expand();
      }
    });
  } catch (error) {
    console.error('Failed to listen for show shortcut:', error);
  }
});

onUnmounted(() => {
  // 清理事件监听
  if (unlistenToggle) {
    unlistenToggle();
  }
  if (unlistenShow) {
    unlistenShow();
  }
});
</script>
