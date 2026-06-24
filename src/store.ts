import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export const CONSTANTS = {
    UI: {
        TITLEBAR_HEIGHT: 38,
    },
    WINDOW_SIZES: {
        compact: { width: 400, height: 600 },
        standard: { width: 800, height: 600 },
        tall: { width: 600, height: 800 },
        large: { width: 1024, height: 800 },
    },
    HOTKEYS: {
        DEFAULT: 'Alt+Space'
    }
};

// State stores
export const isSettingsVisible = writable(false);
export const globalHotkey = writable(CONSTANTS.HOTKEYS.DEFAULT);
export const windowSize = writable('tall');
export const isPinned = writable(false);
export const isLocked = writable(false);
export const isStartup = writable(false);
export const isSmoothMode = writable(false);

export async function hydrateSettings() {
    try {
        const jsonStr = await invoke<string>('load_settings');
        const settings = JSON.parse(jsonStr || '{}');

        if (settings.globalHotkey) {
            globalHotkey.set(settings.globalHotkey);
            await invoke('change_hotkey', { oldHotkey: CONSTANTS.HOTKEYS.DEFAULT, newHotkey: settings.globalHotkey }).catch(console.error);
        }

        if (settings.windowSize) {
            windowSize.set(settings.windowSize);
            const preset = CONSTANTS.WINDOW_SIZES[settings.windowSize as keyof typeof CONSTANTS.WINDOW_SIZES];
            if (preset) {
                const { getCurrentWindow, LogicalSize } = await import('@tauri-apps/api/window');
                await getCurrentWindow().setSize(new LogicalSize(preset.width, preset.height));
            }
        }

        if (settings.isPinned !== undefined) {
            isPinned.set(settings.isPinned);
            await invoke('sync_pinned', { pinned: settings.isPinned }).catch(console.error);
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().setAlwaysOnTop(settings.isPinned);
        }

        if (settings.isLocked !== undefined) {
            isLocked.set(settings.isLocked);
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().setResizable(!settings.isLocked);
        }

        if (settings.isStartup !== undefined) isStartup.set(settings.isStartup);
        if (settings.isSmoothMode !== undefined) isSmoothMode.set(settings.isSmoothMode);

    } catch (e) {
        console.error("Critical error loading settings:", e);
    }
}
