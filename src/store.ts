import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { LazyStore } from '@tauri-apps/plugin-store';

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

export const settingsStore = new LazyStore('settings.json');

// State stores
export const isSettingsVisible = writable(false);
export const globalHotkey = writable(CONSTANTS.HOTKEYS.DEFAULT);
export const windowSize = writable('tall');
export const isPinned = writable(false);
export const isLocked = writable(false);
export const isStartup = writable(false);
export const isSmoothMode = writable(false);

export async function hydrateSettings() {
    // Await the store to be loaded
    await settingsStore.init();
    
    const savedHotkey = await settingsStore.get<string>('globalHotkey');
    if (savedHotkey) {
        globalHotkey.set(savedHotkey);
        await invoke('change_hotkey', { oldHotkey: CONSTANTS.HOTKEYS.DEFAULT, newHotkey: savedHotkey }).catch(console.error);
    }

    const savedSize = await settingsStore.get<string>('windowSize');
    if (savedSize) {
        windowSize.set(savedSize);
        const preset = CONSTANTS.WINDOW_SIZES[savedSize as keyof typeof CONSTANTS.WINDOW_SIZES];
        if (preset) {
            const { getCurrentWindow, LogicalSize } = await import('@tauri-apps/api/window');
            await getCurrentWindow().setSize(new LogicalSize(preset.width, preset.height));
        }
    }

    const savedPinned = await settingsStore.get<boolean>('isPinned');
    if (savedPinned !== null) {
        isPinned.set(savedPinned);
        await invoke('sync_pinned', { pinned: savedPinned }).catch(console.error);
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().setAlwaysOnTop(savedPinned);
    }

    const savedLocked = await settingsStore.get<boolean>('isLocked');
    if (savedLocked !== null) {
        isLocked.set(savedLocked);
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        await getCurrentWindow().setResizable(!savedLocked);
    }

    const savedStartup = await settingsStore.get<boolean>('isStartup');
    if (savedStartup !== null) isStartup.set(savedStartup);

    const savedSmoothMode = await settingsStore.get<boolean>('isSmoothMode');
    if (savedSmoothMode !== null) isSmoothMode.set(savedSmoothMode);
}
