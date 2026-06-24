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
