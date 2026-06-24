<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  
  import Titlebar from './components/Titlebar.svelte';
  import SettingsModal from './components/SettingsModal.svelte';
  import WebviewContainer from './components/WebviewContainer.svelte';
  import { isSettingsVisible, hydrateSettings, windowSize, isLocked, globalHotkey, CONSTANTS } from './store';
  import { settingsService } from './services/settingsService';

  const appWindow = getCurrentWindow();

  onMount(async () => {
    // Load persisted settings
    await hydrateSettings();
    
    // Tell Rust backend we are ready to show the window gracefully
    await invoke('frontend_ready');

    // Global event listeners
    const unlistenHide = await listen('request-hide', async () => {
        await appWindow.hide();
        await invoke('optimize_memory');
    });
    
    const unlistenShow = await listen('request-show', async () => {
        // If settings are not visible, show the webview
        import('svelte/store').then(({ get }) => {
            if (!get(isSettingsVisible)) {
                invoke('show_webview').catch(console.error);
            }
        });
    });

    return () => {
      unlistenHide();
      unlistenShow();
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
