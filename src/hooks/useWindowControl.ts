import { ref, onMounted, onUnmounted } from 'vue';

/**
 * 窗口控制 Hook（简化版）
 * 仅保留窗口拖拽功能，移除展开/收缩逻辑
 */
export function useWindowControl() {
  const mousePosition = ref({ x: 0, y: 0 });

  // 保留鼠标位置追踪（可能用于其他功能）
  const handleMouseMove = (event: MouseEvent) => {
    mousePosition.value = { x: event.clientX, y: event.clientY };
  };

  onMounted(() => {
    // 保留全局鼠标移动监听（可能用于其他功能）
    document.addEventListener('mousemove', handleMouseMove);
  });

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleMouseMove);
  });

  return {
    mousePosition,
  };
}