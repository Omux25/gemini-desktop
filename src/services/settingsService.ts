import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart';
import { CONSTANTS, settingsStore } from '../store';

const appWindow = getCurrentWindow();

export const settingsService = {
  async setHotkey(oldHotkey: string, newHotkey: string) {
    await invoke('change_hotkey', { oldHotkey, newHotkey });
    await settingsStore.set('globalHotkey', newHotkey);
    await settingsStore.save();
  },

  async setWindowSize(sizeId: keyof typeof CONSTANTS.WINDOW_SIZES, isLocked: boolean) {
    const preset = CONSTANTS.WINDOW_SIZES[sizeId];
    if (preset) {
      await appWindow.setResizable(true);
      await appWindow.setSize(new LogicalSize(preset.width, preset.height));
      if (isLocked) {
        await appWindow.setResizable(false);
      }
      await settingsStore.set('windowSize', sizeId);
      await settingsStore.save();
    }
  },

  async setPinned(pinned: boolean) {
    await appWindow.setAlwaysOnTop(pinned);
    await invoke('sync_pinned', { pinned });
    await settingsStore.set('isPinned', pinned);
    await settingsStore.save();
  },

  async setLocked(locked: boolean) {
    await appWindow.setResizable(!locked);
    await settingsStore.set('isLocked', locked);
    await settingsStore.save();
  },

  async setStartup(startup: boolean) {
    if (startup) {
      await enableAutostart();
    } else {
      await disableAutostart();
    }
    await settingsStore.set('isStartup', startup);
    await settingsStore.save();
  },

  async setSmoothMode(enabled: boolean) {
    await invoke('set_smooth_mode', { enabled });
    await settingsStore.set('isSmoothMode', enabled);
    await settingsStore.save();
  },

  async restartApp() {
    await invoke('restart_app');
  }
};
