import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { enable as enableAutostart, disable as disableAutostart } from '@tauri-apps/plugin-autostart';
import { CONSTANTS, type WindowSizeKey, hotkeyToggle, hotkeyCopy, hotkeySnip, customPrompt, windowSize, isPinned, isLocked, isStartup, isSmoothMode, isAutoHide } from '../store';
import { get } from 'svelte/store';

const appWindow = getCurrentWindow();

async function saveAllSettings() {
    const settings = {
        hotkeyToggle: get(hotkeyToggle),
        hotkeyCopy: get(hotkeyCopy),
        hotkeySnip: get(hotkeySnip),
        customPrompt: get(customPrompt),
        windowSize: get(windowSize),
        isPinned: get(isPinned),
        isLocked: get(isLocked),
        isStartup: get(isStartup),
        isSmoothMode: get(isSmoothMode),
        isAutoHide: get(isAutoHide)
    };
    await invoke('save_settings', { settingsJson: JSON.stringify(settings) }).catch(console.error);
}

export const settingsService = {
  async setHotkey(action: string, oldHotkey: string, newHotkey: string) {
    await invoke('change_hotkey', { action, oldHotkey, newHotkey });
    await saveAllSettings();
  },

  async setCustomPrompt(prompt: string) {
    await invoke('set_custom_prompt', { prompt });
    await saveAllSettings();
  },

  async setWindowSize(sizeId: WindowSizeKey, isLockedState: boolean) {
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

  async setAutoHide(enabled: boolean) {
    await invoke('set_auto_hide', { enabled });
    await saveAllSettings();
  },

  async restartApp() {
    await invoke('restart_app');
  }
};
