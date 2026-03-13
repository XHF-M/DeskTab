<template>
  <!-- 悬浮按钮模式 -->
  <button
    v-if="windowStore.isMinimized"
    class="floating-button"
    @click="handleMinimizeClick"
    @mousedown="handleFloatingMouseDown"
    @mouseup="handleFloatingMouseUp"
    @mousemove="handleFloatingMouseMove"
    title="显示 DeskTab"
  >
    <span class="fab-icon">📁</span>
  </button>

  <!-- 正常窗口模式 -->
  <div
    v-else
    class="top-bar"
    @mousedown="handleMouseDown"
  >
    <div class="disk-tabs-header">
      <DiskTab />
      <button
        class="minimize-button"
        @click.stop="handleMinimizeClick"
        title="最小化到悬浮按钮"
      >
        ➖
      </button>
    </div>

    <div v-if="selectedDrive" class="file-list-container">
      <div class="disk-tabs-divider"></div>
      <FileList :currentDrive="selectedDrive" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useDiskDrives } from '../hooks/useDiskDrives';
import { useWindowStore } from '../stores/windowStore';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import DiskTab from './DiskTab.vue';
import FileList from './FileList.vue';

const { refreshDrives, selectedDrive } = useDiskDrives();
const windowStore = useWindowStore();

// 悬浮按钮拖拽状态
let isDragging = false;
let dragStartX = 0;
let dragStartY = 0;
const DRAG_THRESHOLD = 5; // 移动超过 5px 才算拖拽

const handleMinimizeClick = () => {
  windowStore.toggleMinimize();
};

const handleFloatingMouseDown = (event: MouseEvent) => {
  isDragging = false;
  dragStartX = event.clientX;
  dragStartY = event.clientY;
};

const handleFloatingMouseMove = (event: MouseEvent) => {
  if (isDragging) return; // 已经在拖拽中

  const deltaX = Math.abs(event.clientX - dragStartX);
  const deltaY = Math.abs(event.clientY - dragStartY);

  // 如果移动距离超过阈值，开始拖拽
  if (deltaX > DRAG_THRESHOLD || deltaY > DRAG_THRESHOLD) {
    isDragging = true;
    try {
      appWindow.startDragging();
    } catch (error) {
      console.error('Failed to start dragging:', error);
    }
  }
};

const handleFloatingMouseUp = () => {
  // 如果没有拖拽，允许 click 事件触发展开
  // 如果拖拽了，click 事件会被阻止
  if (!isDragging) {
    // click 事件会自然触发 handleMinimizeClick
  }
};

const handleMouseDown = (event: MouseEvent) => {
  const target = event.target as HTMLElement;

  // 正常窗口模式下，排除特定元素的拖拽
  if (target.closest('.disk-tab') ||
      target.closest('.file-item') ||
      target.closest('button')) {
    return;
  }

  try {
    appWindow.startDragging();
  } catch (error) {
    console.error('Failed to start dragging:', error);
  }
};

onMounted(async () => {
  await refreshDrives();

  try {
    await invoke('set_always_on_top', { alwaysOnTop: true });
  } catch (error) {
    console.error('Failed to set always on top:', error);
  }
});
</script>

<style scoped>
/* 悬浮按钮模式 - 隐藏滚动条 */
*:focus {
  outline: none;
}

/* 悬浮按钮样式 */
.floating-button {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--accent-color), #1976d2);
  border: 2px solid rgba(255, 255, 255, 0.2);
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.3),
    0 0 0 1px rgba(255, 255, 255, 0.1);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  backdrop-filter: blur(10px);
  overflow: hidden;
}

.floating-button:hover {
  transform: scale(1.1);
  box-shadow:
    0 6px 30px rgba(33, 150, 243, 0.4),
    0 0 0 2px rgba(255, 255, 255, 0.2);
  background: linear-gradient(135deg, #42a5f5, #2196f3);
}

.floating-button:active {
  transform: scale(0.95);
}

.fab-icon {
  font-size: 28px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
}

/* 正常窗口模式样式 */
.top-bar {
  width: 100%;
  background: var(--background-primary);
  display: flex;
  flex-direction: column;
  border-radius: 0 0 6px 6px;
  backdrop-filter: blur(10px);
  position: relative;
  padding: 12px;
  height: 350px;
}

.disk-tabs-header {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
  flex-shrink: 0;
  position: sticky;
  top: 0;
  background: var(--background-primary);
  z-index: 10;
  padding: 6px 0;
  margin-bottom: 4px;
  isolation: isolate;
}

.minimize-button {
  margin-left: auto;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
  opacity: 0.6;
}

.minimize-button:hover {
  background: rgba(255, 255, 255, 0.2);
  opacity: 1;
}

.file-list-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
  scroll-behavior: smooth;
  max-height: 260px;
}

.disk-tabs-divider {
  width: 100%;
  height: 1px;
  background: linear-gradient(
    to right,
    transparent,
    rgba(255, 255, 255, 0.1) 20%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.1) 80%,
    transparent
  );
  margin: 12px 0;
  flex-shrink: 0;
}

/* Custom scrollbar for better UX */
.file-list-container::-webkit-scrollbar {
  width: 8px;
}

.file-list-container::-webkit-scrollbar-track {
  background: transparent;
}

.file-list-container::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
}

.file-list-container::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>
