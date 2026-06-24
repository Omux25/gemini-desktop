import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart';
import { CONSTANTS, globalHotkey, windowSize, isPinned, isLocked, isStartup, isSmoothMode } from '../store';
import { get } from 'svelte/store';

const appWindow = getCurrentWindow();

async function saveAllSettings() {
    const settings = {
        globalHotkey: get(globalHotkey),
        windowSize: get(windowSize),
        isPinned: get(isPinned),
        isLocked: get(isLocked),
        isStartup: get(isStartup),
        isSmoothMode: get(isSmoothMode)
    };
    await invoke('save_settings', { settingsJson: JSON.stringify(settings) }).catch(console.error);
}

export const settingsService = {
  async setHotkey(oldHotkey: string, newHotkey: string) {
    await invoke('change_hotkey', { oldHotkey, newHotkey });
    await saveAllSettings();
  },

  async setWindowSize(sizeId: keyof typeof CONSTANTS.WINDOW_SIZES, isLockedState: boolean) {
    const preset = CONSTANTS.WINDOW_SIZES[sizeId];
    if (preset) {
      await appWindow.setResizable(true);
      await appWindow.setSize(new LogicalSize(preset.width, preset.height));
      if (isLockedState) {
        await appWindow.setResizable(false);
      }
      await saveAllSettings();
    }
  },

  async setPinned(pinned: boolean) {
    await appWindow.setAlwaysOnTop(pinned);
    await invoke('sync_pinned', { pinned });
    await saveAllSettings();
  },

  async setLocked(locked: boolean) {
    await appWindow.setResizable(!locked);
    await saveAllSettings();
  },

  async setStartup(startup: boolean) {
    if (startup) {
      await enableAutostart();
    } else {
      await disableAutostart();
    }
    await saveAllSettings();
  },

  async setSmoothMode(enabled: boolean) {
    await invoke('set_smooth_mode', { enabled });
    await saveAllSettings();
  },

  async restartApp() {
    await invoke('restart_app');
  }
};
