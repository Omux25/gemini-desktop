<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  import Titlebar from './components/Titlebar.svelte';
  import SettingsModal from './components/SettingsModal.svelte';
  import WebviewContainer from './components/WebviewContainer.svelte';
  import { isSettingsVisible, hydrateSettings, windowSize, isLocked, CONSTANTS } from './store';
  import { settingsService } from './services/settingsService';
  import { updaterService } from './services/updaterService';

  const appWindow = getCurrentWindow();

  onMount(async () => {
    // Load persisted settings
    await hydrateSettings();
    
    // Tell Rust backend we are ready to show the window gracefully
    await invoke('frontend_ready');

    // Silently check for updates in the background on boot
    updaterService.checkForUpdates().catch(console.error);

    // Global event listeners
    const unlistenHide = await listen('request-hide', async () => {
        await appWindow.hide();
        await invoke('optimize_memory');
    });
    
    const unlistenShow = await listen('request-show', async () => {
        // Always reset to the chat view when the window is summoned
        isSettingsVisible.set(false);
        invoke('show_webview').catch(console.error);
    });

    const unlistenTextSelected = await listen<string>('text-selected', async (event) => {
        await invoke('inject_text', { text: event.payload });
    });

    return () => {
      unlistenHide();
      unlistenShow();
      unlistenTextSelected();
    };
  });
</script>

<Titlebar />

<SettingsModal />
<WebviewContainer />

<style>
  :global(body) {
      margin: 0; 
      padding: 0; 
      background-color: #131314; /* Matches Gemini Dark Mode */
      overflow: hidden; 
      font-family: 'Inter', sans-serif;
  }
</style>
