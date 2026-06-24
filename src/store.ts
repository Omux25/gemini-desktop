import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { Store } from '@tauri-apps/plugin-store';

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

export const settingsStore = new Store('settings.json');

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
    await settingsStore.load();
    
    const savedHotkey = await settingsStore.get<string>('globalHotkey');
    if (savedHotkey) {
        globalHotkey.set(savedHotkey);
        await invoke('change_hotkey', { oldHotkey: CONSTANTS.HOTKEYS.DEFAULT, newHotkey: savedHotkey }).catch(console.error);
    }

    const savedSize = await settingsStore.get<string>('windowSize');
    if (savedSize) windowSize.set(savedSize);

    const savedPinned = await settingsStore.get<boolean>('isPinned');
    if (savedPinned !== null) {
        isPinned.set(savedPinned);
        await invoke('sync_pinned', { pinned: savedPinned }).catch(console.error);
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
