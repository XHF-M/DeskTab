import { ref, onMounted, onUnmounted } from 'vue';

export function useMousePosition() {
  const x = ref(0);
  const y = ref(0);

  const updatePosition = (event: MouseEvent) => {
    x.value = event.clientX;
    y.value = event.clientY;
  };

  onMounted(() => {
    document.addEventListener('mousemove', updatePosition);
  });

  onUnmounted(() => {
    document.removeEventListener('mousemove', updatePosition);
  });

  return { x, y };
}
