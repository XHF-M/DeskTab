<template>
  <div class="file-list" ref="fileListRef">
    <div class="title-bar">
      <button
        v-if="canGoBack"
        class="back-button"
        @click="goBack"
        title="返回上一级"
      >
        ⬆️
      </button>
      <h3 class="title">{{ subtitle }}</h3>
      <input
        v-if="props.currentDrive && files.length > 0"
        v-model="searchQuery"
        type="text"
        class="search-input"
        placeholder="搜索..."
        @click.stop
      />
    </div>
    <div class="files">
      <!-- 骨架屏加载状态 -->
      <template v-if="loading">
        <div v-for="i in 8" :key="i" class="skeleton-item">
          <div class="skeleton-icon"></div>
          <div class="skeleton-name"></div>
          <div class="skeleton-size"></div>
        </div>
      </template>
      <!-- 文件列表 -->
      <template v-else-if="filteredFiles.length > 0">
        <div
          v-for="file in filteredFiles"
          :key="file.path"
          class="file-item"
          :class="{ isDirectory: file.is_dir }"
          @click="selectFile(file, $event)"
          @dblclick="openItem(file, $event)"
        >
          <span class="icon">{{ file.is_dir ? '📁' : '📄' }}</span>
          <span class="name">{{ file.name }}</span>
          <span class="size">{{ formatSize(file.size) }}</span>
        </div>
      </template>
      <!-- 空状态 -->
      <div v-else-if="!loading && filteredFiles.length === 0" class="empty-state">
        <span class="empty-icon">📂</span>
        <span class="empty-text">{{ searchQuery ? '未找到匹配文件' : '空目录' }}</span>
      </div>
      <!-- 错误状态 -->
      <div v-if="error" class="error">
        {{ error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/shell';

const props = defineProps<{
  currentDrive: string | null;
}>();

interface FileInfo {
  path: string;
  name: string;
  size: number;
  is_dir: boolean;
}

const files = ref<FileInfo[]>([]);
const loading = ref(false);
const error = ref('');
const currentPath = ref('');
const fileListRef = ref<HTMLElement | null>(null);
const searchQuery = ref('');

// 计算副标题（显示当前路径）
const subtitle = computed(() => {
  if (!props.currentDrive) return '';
  if (files.value.length === 0 && !loading.value) return currentPath.value || '空目录';
  return currentPath.value || `${props.currentDrive} (${files.value.length} 项)`;
});

// 是否可以返回上一级
const canGoBack = computed(() => {
  if (!currentPath.value || !props.currentDrive) return false;
  const pathParts = currentPath.value.replace(/\\+$/, '').split('\\');
  return pathParts.length > 1;
});

// 过滤后的文件列表
const filteredFiles = computed(() => {
  if (!searchQuery.value.trim()) {
    return files.value;
  }
  const query = searchQuery.value.toLowerCase();
  return files.value.filter(file => file.name.toLowerCase().includes(query));
});

// 监听当前盘符变化（immediate 确保首次加载也会触发）
watch(() => props.currentDrive, async (newDrive) => {
  if (newDrive) {
    // 切换盘符时重置到根目录
    currentPath.value = `${newDrive}\\`;
    await loadFiles(currentPath.value);
  } else {
    files.value = [];
    currentPath.value = '';
    error.value = '';
  }
}, { immediate: true });

// 加载文件列表
async function loadFiles(path: string) {
  loading.value = true;
  error.value = '';
  // 滚动到顶部
  if (fileListRef.value) {
    fileListRef.value.scrollTop = 0;
  }
  try {
    const result = await invoke<FileInfo[]>('get_files_in_directory', { path });
    files.value = result;
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载失败';
    console.error('Failed to load files:', err);
  } finally {
    loading.value = false;
  }
}

// 打开文件或文件夹
// async function openItem(file: FileInfo) {
//   try {
//     await open(file.path);
//   } catch (err) {
//     console.error('Failed to open item:', err);
//   }
// }

// 单击选中文件
function selectFile(file: FileInfo, event: MouseEvent) {
  // 阻止事件冒泡，避免触发窗口拖拽
  event.stopPropagation();
  // 可以添加选中逻辑
  console.log('Selected:', file.path);
}

// 打开文件或文件夹
async function openItem(file: FileInfo, event: MouseEvent) {
  // 阻止事件冒泡，避免触发窗口拖拽
  event.stopPropagation();

  if (file.is_dir) {
    // 如果是文件夹，进入该目录
    currentPath.value = file.path;
    await loadFiles(currentPath.value);
  } else {
    // 如果是文件，用系统默认方式打开
    try {
      // 方案1: 使用 Tauri Command API (推荐，更安全)
      await invoke('open_file_with_default_app', { path: file.path });
      console.log('文件打开成功');
    } catch (err) {
      console.error('Failed to open item:', err);
      console.error('错误详情:', err);

      // 方案2: 如果 Command API 失败，尝试直接打开 URL
      try {
        const fileUrl = `file:///${file.path.replace(/\\/g, '/')}`;
        console.log('尝试用 file:// URL 打开:', fileUrl);
        await open(fileUrl);
        console.log('文件通过 URL 打开成功');
      } catch (err2) {
        console.error('URL 方式也失败:', err2);
      }
    }
  }
}

// 返回上一级目录
async function goBack() {
  if (!currentPath.value || !props.currentDrive) return;

  // 计算上一级路径
  const pathParts = currentPath.value.replace(/\\+$/, '').split('\\');
  if (pathParts.length <= 1) {
    // 已经在根目录，无法返回
    return;
  }

  pathParts.pop();
  const parentPath = pathParts.join('\\') + '\\';
  currentPath.value = parentPath;
  await loadFiles(currentPath.value);
}

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}
</script>

<style scoped>
.file-list {
  padding: 8px;
  background: var(--background-primary);
  border-radius: 6px;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

/* 自定义滚动条 */
.file-list::-webkit-scrollbar {
  width: 6px;
}

.file-list::-webkit-scrollbar-track {
  background: transparent;
}

.file-list::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
  border-radius: 3px;
}

.file-list::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.25);
}

.title {
  font-size: 12px;
  color: var(--text-primary);
  margin: 0;
  flex: 1;
}

.title-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.back-button {
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  font-size: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.back-button:hover {
  background: var(--background-hover);
  border-color: var(--accent-color);
}

.search-input {
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: 12px;
  outline: none;
  transition: all 0.2s ease;
  width: 120px;
}

.search-input::placeholder {
  color: var(--text-secondary);
  opacity: 0.6;
}

.search-input:focus {
  background: rgba(255, 255, 255, 0.08);
  border-color: var(--accent-color);
}

.files {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

/* 骨架屏样式 */
.skeleton-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  gap: 8px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.03);
}

.skeleton-icon {
  width: 20px;
  height: 20px;
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.05) 25%,
    rgba(255, 255, 255, 0.1) 50%,
    rgba(255, 255, 255, 0.05) 75%
  );
  background-size: 200% 100%;
  animation: skeleton-shimmer 1.5s ease-in-out infinite;
  border-radius: 4px;
  flex-shrink: 0;
}

.skeleton-name {
  flex: 1;
  height: 13px;
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.05) 25%,
    rgba(255, 255, 255, 0.1) 50%,
    rgba(255, 255, 255, 0.05) 75%
  );
  background-size: 200% 100%;
  animation: skeleton-shimmer 1.5s ease-in-out infinite;
  border-radius: 3px;
}

.skeleton-size {
  width: 60px;
  height: 11px;
  background: linear-gradient(
    90deg,
    rgba(255, 255, 255, 0.05) 25%,
    rgba(255, 255, 255, 0.1) 50%,
    rgba(255, 255, 255, 0.05) 75%
  );
  background-size: 200% 100%;
  animation: skeleton-shimmer 1.5s ease-in-out infinite;
  border-radius: 3px;
  flex-shrink: 0;
}

@keyframes skeleton-shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

/* 文件项样式 */
.file-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.05);
  cursor: pointer;
  transition: all var(--transition-speed);
  gap: 8px;
}

.file-item:hover {
  background: var(--background-hover);
}

.file-item.isDirectory {
  color: var(--accent-color);
}

.file-item .icon {
  font-size: 16px;
  min-width: 20px;
}

.file-item .name {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-item .size {
  font-size: 11px;
  color: var(--text-secondary);
  min-width: 60px;
  text-align: right;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  gap: 8px;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

.empty-text {
  font-size: 13px;
  color: var(--text-secondary);
}

/* 错误状态 */
.error {
  padding: 12px;
  text-align: center;
  font-size: 13px;
  color: #ff6b6b;
}
</style>