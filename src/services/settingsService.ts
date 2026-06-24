import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart';
import { CONSTANTS } from '../store';

const appWindow = getCurrentWindow();

export const settingsService = {
  async setHotkey(oldHotkey: string, newHotkey: string) {
    await invoke('change_hotkey', { oldHotkey, newHotkey });
  },

  async setWindowSize(sizeId: keyof typeof CONSTANTS.WINDOW_SIZES, isLocked: boolean) {
    const preset = CONSTANTS.WINDOW_SIZES[sizeId];
    if (preset) {
      await appWindow.setResizable(true);
      await appWindow.setSize(new LogicalSize(preset.width, preset.height));
      if (isLocked) {
        await appWindow.setResizable(false);
      }
    }
  },

  async setPinned(pinned: boolean) {
    await appWindow.setAlwaysOnTop(pinned);
    await invoke('sync_pinned', { pinned });
  },

  async setLocked(locked: boolean) {
    await appWindow.setResizable(!locked);
  },

  async setStartup(startup: boolean) {
    if (startup) {
      await enableAutostart();
    } else {
      await disableAutostart();
    }
  },

  async setSmoothMode(enabled: boolean) {
    await invoke('set_smooth_mode', { enabled });
  },

  async restartApp() {
    await invoke('restart_app');
  }
};
