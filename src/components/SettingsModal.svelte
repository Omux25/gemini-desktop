<script lang="ts">
  import { 
    isSettingsVisible, globalHotkey, windowSize, 
    isPinned, isLocked, isStartup, isSmoothMode, CONSTANTS 
  } from '../store';
  import { settingsService } from '../services/settingsService';
  import { windowService } from '../services/windowService';
  
  import ToggleSetting from './settings/ToggleSetting.svelte';
  import HotkeySetting from './settings/HotkeySetting.svelte';
  import WindowSizeSetting from './settings/WindowSizeSetting.svelte';

  let showDialog = false;

  // We still hide the webview when settings are visible so it doesn't overlap weirdly
  $: {
    if ($isSettingsVisible) {
      windowService.hideWebview().catch(console.error);
    } else {
      windowService.showWebview().catch(console.error);
    }
  }

  function handleHotkeyChange(newHotkey: string) {
    settingsService.setHotkey($globalHotkey, newHotkey).catch(console.error);
    globalHotkey.set(newHotkey);
  }

  function handleSizeChange(sizeId: string) {
    settingsService.setWindowSize(sizeId as any, $isLocked).catch(console.error);
    windowSize.set(sizeId);
  }

  function handlePinnedChange(pinned: boolean) {
    settingsService.setPinned(pinned).catch(console.error);
    isPinned.set(pinned);
  }

  function handleLockedChange(locked: boolean) {
    settingsService.setLocked(locked).catch(console.error);
    isLocked.set(locked);
  }

  function handleStartupChange(startup: boolean) {
    settingsService.setStartup(startup).catch(console.error);
    isStartup.set(startup);
  }

  function handleSmoothModeChange(enabled: boolean) {
    settingsService.setSmoothMode(enabled).catch(console.error);
    isSmoothMode.set(enabled);
    showDialog = true;
  }

  const handleDone = () => isSettingsVisible.set(false);
  const handleDialogCancel = () => showDialog = false;
  const handleDialogRestart = () => settingsService.restartApp();
</script>

{#if $isSettingsVisible}
<div class="settings-modal">
  <div class="settings-header">
      <h2>Settings</h2>
      <span class="version-text">v0.1.0</span>
  </div>
  
  <div class="settings-content">
      <HotkeySetting 
        currentHotkey={$globalHotkey} 
        onChange={handleHotkeyChange} 
      />

      <WindowSizeSetting 
        currentSize={$windowSize} 
        onChange={handleSizeChange} 
      />

      <ToggleSetting 
        id="alwaysOnTopToggle"
        title="Always on Top"
        description="Keep window floating above other apps"
        checked={$isPinned}
        onChange={handlePinnedChange}
      />

      <ToggleSetting 
        id="lockSizeToggle"
        title="Lock Window Size"
        description="Prevent accidental manual resizing"
        checked={$isLocked}
        onChange={handleLockedChange}
      />

      <ToggleSetting 
        id="startupToggle"
        title="Launch on Startup"
        description="Start quietly in tray when PC boots"
        checked={$isStartup}
        onChange={handleStartupChange}
      />

      <ToggleSetting 
        id="smoothModeToggle"
        title="Smooth Mode (High RAM)"
        description="Allow Chromium to consume ~500MB+ for maximum performance. Requires restart."
        checked={$isSmoothMode}
        onChange={handleSmoothModeChange}
      />
  </div>

  <div class="settings-footer">
      <button class="primary-btn" onclick={handleDone}>Done</button>
  </div>
</div>
{/if}

{#if showDialog}
<div class="settings-modal custom-dialog-overlay">
  <div class="custom-dialog-box">
      <h3>Restart Required</h3>
      <p>Smooth Mode changes require a restart to apply the new memory parameters. Restart now?</p>
      <div class="custom-dialog-actions">
          <button class="pill" onclick={handleDialogCancel}>Later</button>
          <button class="pill active" onclick={handleDialogRestart}>Restart</button>
      </div>
  </div>
</div>
{/if}

<style>
/* CSS styles are scoped nicely here now that child components handle their own layout */
.settings-modal {
    position: absolute;
    top: 38px;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(19, 19, 20, 0.95);
    backdrop-filter: blur(10px);
    z-index: 100;
    display: flex;
    flex-direction: column;
    padding: 24px;
    color: #e3e3e3;
    overflow-y: auto;
    font-family: 'Inter', sans-serif;
    animation: fadeIn 0.2s ease-out;
}

.settings-modal::-webkit-scrollbar { width: 6px; }
.settings-modal::-webkit-scrollbar-track { background: transparent; }
.settings-modal::-webkit-scrollbar-thumb { background-color: #3c4043; border-radius: 10px; }
.settings-modal::-webkit-scrollbar-thumb:hover { background-color: #5f6368; }

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
}

.settings-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.settings-header h2 { margin: 0; font-size: 24px; font-weight: 600; }
.version-text { font-size: 13px; color: #8ab4f8; }

.settings-content { flex: 1; display: flex; flex-direction: column; gap: 16px; }

:global(.settings-section) { background-color: #1e1e1e; border-radius: 12px; padding: 16px 20px; display: flex; flex-direction: column; gap: 12px; border: 1px solid rgba(255, 255, 255, 0.05); }
:global(.pill-group) { display: flex; gap: 8px; flex-wrap: wrap; }
:global(.pill) { background: transparent; border: 1px solid #3c4043; color: #e3e3e3; padding: 8px 16px; border-radius: 20px; font-size: 13px; cursor: pointer; transition: all 0.2s; }
:global(.pill:hover) { background-color: rgba(255, 255, 255, 0.05); }
:global(.pill.active) { background-color: rgba(66, 133, 244, 0.15); border-color: #4285f4; color: #8ab4f8; }

.settings-footer { margin-top: 24px; display: flex; justify-content: center; }
.primary-btn { background-color: #1a73e8; color: white; border: none; border-radius: 8px; padding: 12px 0; width: 100%; font-size: 15px; font-weight: 600; cursor: pointer; transition: background-color 0.2s; }
.primary-btn:hover { background-color: #1b66c9; }

.custom-dialog-overlay { z-index: 1000; align-items: center; justify-content: center; background-color: rgba(0, 0, 0, 0.7); display: flex; }
.custom-dialog-box { background-color: #1e1e1e; padding: 24px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); width: 80%; max-width: 320px; box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5); }
.custom-dialog-box h3 { margin: 0 0 12px 0; font-size: 16px; font-weight: 600; }
.custom-dialog-box p { margin: 0 0 20px 0; font-size: 13px; color: #9aa0a6; line-height: 1.5; }
.custom-dialog-actions { display: flex; gap: 12px; justify-content: flex-end; }
</style>
