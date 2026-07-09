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
        DEFAULT_TOGGLE: 'Alt+Space',
        DEFAULT_COPY: 'Alt+C',
        DEFAULT_SNIP: 'Alt+S'
    }
};

// State stores
export const isSettingsVisible = writable(false);
export const hotkeyToggle = writable(CONSTANTS.HOTKEYS.DEFAULT_TOGGLE);
export const hotkeyCopy = writable(CONSTANTS.HOTKEYS.DEFAULT_COPY);
export const hotkeySnip = writable(CONSTANTS.HOTKEYS.DEFAULT_SNIP);
export const windowSize = writable('tall');
export const isPinned = writable(false);
export const isLocked = writable(false);
export const isStartup = writable(false);
export const isSmoothMode = writable(false);
export const isAutoHide = writable(false);

export async function hydrateSettings() {
    try {
        const jsonStr = await invoke<string>('load_settings');
        const settings = JSON.parse(jsonStr || '{}');

        if (settings.hotkeyToggle) {
            hotkeyToggle.set(settings.hotkeyToggle);
            await invoke('change_hotkey', { action: 'toggle', oldHotkey: CONSTANTS.HOTKEYS.DEFAULT_TOGGLE, newHotkey: settings.hotkeyToggle }).catch(console.error);
        }
        if (settings.hotkeyCopy) {
            hotkeyCopy.set(settings.hotkeyCopy);
            await invoke('change_hotkey', { action: 'copy', oldHotkey: CONSTANTS.HOTKEYS.DEFAULT_COPY, newHotkey: settings.hotkeyCopy }).catch(console.error);
        }
        if (settings.hotkeySnip) {
            hotkeySnip.set(settings.hotkeySnip);
            await invoke('change_hotkey', { action: 'snip', oldHotkey: CONSTANTS.HOTKEYS.DEFAULT_SNIP, newHotkey: settings.hotkeySnip }).catch(console.error);
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
        
        if (settings.isAutoHide !== undefined) {
            isAutoHide.set(settings.isAutoHide);
            await invoke('set_auto_hide', { enabled: settings.isAutoHide }).catch(console.error);
        }

    } catch (e) {
        console.error("Critical error loading settings:", e);
    }
}
