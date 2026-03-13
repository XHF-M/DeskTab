import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import type { WindowState } from '../types/window';

export const useWindowStore = defineStore('window', {
  state: (): WindowState => ({
    isExpanded: false,
    isHoveringEdge: false,
    windowHeight: 8,
    isDocked: false,
    isMinimized: false,
    previousHeight: 350,
  }),

  actions: {
    expand() {
      const heightToRestore = this.previousHeight || 350;
      invoke('expand_window', { height: heightToRestore }).catch((error) => {
        console.error('Failed to expand window:', error);
      });
      this.isExpanded = true;
      this.isMinimized = false;
      this.windowHeight = heightToRestore;
    },

    collapse() {
      if (this.isMinimized) {
        return;
      }
      this.previousHeight = this.windowHeight;
      invoke('collapse_window').catch((error) => {
        console.error('Failed to collapse window:', error);
      });
      this.isExpanded = false;
      this.windowHeight = 80;
    },

    toggle() {
      if (this.isExpanded) {
        this.collapse();
      } else {
        this.expand();
      }
    },

    setHovering(isHovering: boolean) {
      this.isHoveringEdge = isHovering;
    },

    toggleMinimize() {
      if (this.isMinimized) {
        this.expand();
      } else {
        this.previousHeight = this.windowHeight;
        invoke('minimize_to_fab').catch((error) => {
          console.error('Failed to minimize to fab:', error);
        });
        this.isMinimized = true;
        this.isExpanded = false;
        this.windowHeight = 60;
      }
    },
  },
});
