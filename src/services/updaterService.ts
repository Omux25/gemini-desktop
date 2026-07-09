import { writable } from 'svelte/store';
import { check } from '@tauri-apps/plugin-updater';
import { invoke } from '@tauri-apps/api/core';

export type UpdateState = 'idle' | 'checking' | 'available' | 'downloading' | 'ready';

export const updateState = writable<UpdateState>('idle');
export const updateProgress = writable({ downloaded: 0, total: 0 });
export const updateVersion = writable<string>('');
export const updateNotes = writable<string>('');

let updateInstance: any = null;

export const updaterService = {
    async checkForUpdates() {
        try {
            updateState.set('checking');
            const update = await check();
            if (update) {
                updateInstance = update;
                updateVersion.set(update.version || 'Unknown');
                updateNotes.set(update.body || 'No release notes provided.');
                updateState.set('available');
                return true;
            } else {
                updateState.set('idle');
                return false;
            }
        } catch (error) {
            console.error('Failed to check for updates:', error);
            updateState.set('idle');
            return false;
        }
    },

    async download() {
        if (!updateInstance) return;
        
        updateState.set('downloading');
        let downloaded = 0;
        let contentLength = 0;

        try {
            await updateInstance.download((event: any) => {
                switch (event.event) {
                    case 'Started':
                        contentLength = event.data.contentLength || 0;
                        updateProgress.set({ downloaded: 0, total: contentLength });
                        break;
                    case 'Progress':
                        downloaded += event.data.chunkLength;
                        updateProgress.set({ downloaded, total: contentLength });
                        break;
                    case 'Finished':
                        updateState.set('ready');
                        break;
                }
            });
            updateState.set('ready');
        } catch (error) {
            alert('Updater Error: ' + String(error));
            console.error('Failed to download update:', error);
            updateState.set('available');
        }
    },

    async installUpdate() {
        if (!updateInstance) return;
        try {
            await updateInstance.install();
        } catch (error) {
            alert('Install Error: ' + String(error));
            console.error('Failed to install update:', error);
        }
    }
};
