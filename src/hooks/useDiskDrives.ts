import { onMounted, ref, computed } from 'vue';
import { useDiskStore } from '../stores/diskStore';

export function useDiskDrives() {
  const diskStore = useDiskStore();
  const mounted = ref(false);

  onMounted(async () => {
    mounted.value = true;
    await diskStore.loadDrives();
  });

  const drives = computed(() => diskStore.drives);
  const loading = computed(() => diskStore.loading);
  const error = computed(() => diskStore.error);
  const selectedDrive = computed(() => diskStore.selectedDrive);

  return {
    drives,
    loading,
    error,
    selectedDrive,
    selectDrive: (drive: string) => diskStore.selectDrive(drive),
    clearSelection: () => diskStore.clearSelection(),
    refreshDrives: () => diskStore.loadDrives(),
  };
}