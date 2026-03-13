<template>
  <button
    v-for="drive in drives"
    :key="drive"
    class="disk-tab"
    :class="{ active: drive === selectedDrive }"
    @click="handleClick(drive)"
  >
    {{ drive }}
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useDiskStore } from '../stores/diskStore';

const diskStore = useDiskStore();
const drives = computed(() => diskStore.drives);
const selectedDrive = computed(() => diskStore.selectedDrive);

const handleClick = (drive: string) => {
  diskStore.selectDrive(drive);
};
</script>

<style scoped>
.disk-tab {
  position: relative;
  padding: 8px 16px;
  background: var(--background-primary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  margin: 0 4px;
}

.disk-tab::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%) scaleX(0);
  width: 80%;
  height: 2px;
  background: var(--accent-color);
  transition: transform 0.2s ease;
  border-radius: 1px;
}

.disk-tab:hover {
  background: var(--background-hover);
  border-color: var(--accent-color);
  transform: translateY(-1px);
}

.disk-tab.active {
  background: var(--accent-color);
  border-color: var(--accent-color);
  color: #fff;
  font-weight: 500;
}

.disk-tab.active::after {
  transform: translateX(-50%) scaleX(1);
  background: #fff;
}
</style>
