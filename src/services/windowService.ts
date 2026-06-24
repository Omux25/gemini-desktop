import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();

export const windowService = {
  async hideAndTrim() {
    await invoke('sync_visibility', { visible: false });
    await invoke('optimize_memory');
    await appWindow.hide();
  },

  async showWebview() {
    await invoke('show_webview');
  },

  async hideWebview() {
    await invoke('hide_webview');
  },

  async webviewBack() {
    await invoke('webview_back');
  },

  async webviewReload() {
    await invoke('webview_reload');
  },

  async minimize() {
    await appWindow.minimize();
    await this.hideAndTrim();
  },

  async maximize() {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  },

  async close() {
    await this.hideAndTrim();
  }
};
