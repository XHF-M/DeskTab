import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import type { DiskState } from '../types/disk';

export const useDiskStore = defineStore('disk', {
  state: (): DiskState => ({
    drives: [],
    selectedDrive: null,
    loading: false,
    error: null,
  }),

  actions: {
    async loadDrives() {
      this.loading = true;
      this.error = null;

      try {
        const drives = await invoke<string[]>('get_disk_drives');
        this.drives = drives;
      } catch (error) {
        this.error = error instanceof Error ? error.message : 'Failed to load drives';
        console.error('Failed to load disk drives:', error);
      } finally {
        this.loading = false;
      }
    },

    selectDrive(drive: string) {
      this.selectedDrive = drive;
    },

    clearSelection() {
      this.selectedDrive = null;
    },
  },
});