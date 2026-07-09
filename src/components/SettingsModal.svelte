<script lang="ts">
  import { 
    isSettingsVisible, hotkeyToggle, hotkeyCopy, hotkeySnip, customPrompt, windowSize, 
    isPinned, isLocked, isStartup, isSmoothMode, isAutoHide, CONSTANTS 
  } from '../store';
  import { updaterService, updateState, updateProgress, updateVersion, updateNotes, updateErrorMsg } from '../services/updaterService';
  import { settingsService } from '../services/settingsService';
  import { windowService } from '../services/windowService';
  import { getVersion } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  
  import SettingGroup from './settings/SettingGroup.svelte';
  import ToggleSetting from './settings/ToggleSetting.svelte';
  import HotkeySetting from './settings/HotkeySetting.svelte';
  import WindowSizeSetting from './settings/WindowSizeSetting.svelte';
  import UpdaterBar from './settings/UpdaterBar.svelte';

  let showDialog = false;
  let appVersion = "Unknown";

  onMount(async () => {
      appVersion = await getVersion();
  });

  $: {
    if ($isSettingsVisible) {
      windowService.hideWebview().catch(console.error);
    } else {
      windowService.showWebview().catch(console.error);
    }
  }

  function handleHotkeyToggleChange(newHotkey: string) {
    const oldHotkey = $hotkeyToggle;
    hotkeyToggle.set(newHotkey);
    settingsService.setHotkey('toggle', oldHotkey, newHotkey).catch(console.error);
  }

  function handleHotkeyCopyChange(newHotkey: string) {
    const oldHotkey = $hotkeyCopy;
    hotkeyCopy.set(newHotkey);
    settingsService.setHotkey('copy', oldHotkey, newHotkey).catch(console.error);
  }

  function handleHotkeySnipChange(newHotkey: string) {
    const oldHotkey = $hotkeySnip;
    hotkeySnip.set(newHotkey);
    settingsService.setHotkey('snip', oldHotkey, newHotkey).catch(console.error);
  }

  function handlePromptChange(newPrompt: string) {
    customPrompt.set(newPrompt);
    settingsService.setCustomPrompt(newPrompt).catch(console.error);
  }

  function handleSizeChange(sizeId: string) {
    windowSize.set(sizeId);
    settingsService.setWindowSize(sizeId as any, $isLocked).catch(console.error);
  }

  function handlePinnedChange(pinned: boolean) {
    isPinned.set(pinned);
    settingsService.setPinned(pinned).catch(console.error);
  }

  function handleLockedChange(locked: boolean) {
    isLocked.set(locked);
    settingsService.setLocked(locked).catch(console.error);
  }

  function handleStartupChange(startup: boolean) {
    isStartup.set(startup);
    settingsService.setStartup(startup).catch(console.error);
  }

  function handleSmoothModeChange(enabled: boolean) {
    isSmoothMode.set(enabled);
    settingsService.setSmoothMode(enabled).catch(console.error);
    showDialog = true;
  }

  function handleAutoHideChange(enabled: boolean) {
    isAutoHide.set(enabled);
    settingsService.setAutoHide(enabled).catch(console.error);
  }

  const handleDone = () => isSettingsVisible.set(false);
  const handleDialogCancel = () => showDialog = false;
  const handleDialogRestart = () => settingsService.restartApp();
</script>

{#if $isSettingsVisible}
<div class="settings-modal">
  <div class="settings-modal-inner">
      <div class="settings-header">
          <h2>Settings</h2>
          <span class="version-text">v{appVersion}</span>
      </div>
      
      <div class="settings-content">
          {#if $updateErrorMsg !== ''}
            <div class="update-banner error">
              <div class="update-info">
                <div class="update-title">Updater Error</div>
                <div class="update-notes">{$updateErrorMsg}</div>
              </div>
            </div>
          {/if}

          <UpdaterBar />

          <HotkeySetting 
            currentToggle={$hotkeyToggle} 
            currentCopy={$hotkeyCopy}
            currentSnip={$hotkeySnip}
            currentPrompt={$customPrompt}
            onToggleChange={handleHotkeyToggleChange} 
            onCopyChange={handleHotkeyCopyChange}
            onSnipChange={handleHotkeySnipChange}
            onPromptChange={handlePromptChange}
          />

          <WindowSizeSetting 
            currentSize={$windowSize} 
            onChange={handleSizeChange} 
          />

          <SettingGroup title="Application">
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

              <ToggleSetting 
                id="autoHideToggle"
                title="Auto-Hide"
                description="Automatically hide window when clicking away"
                checked={$isAutoHide}
                onChange={handleAutoHideChange}
              />
          </SettingGroup>
      </div>

      <div class="settings-footer">
          <button class="primary-btn" onclick={handleDone}>Done</button>
      </div>
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
.settings-modal {
    position: absolute;
    top: 38px; /* Below titlebar */
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(19, 19, 20, 0.95);
    backdrop-filter: blur(16px);
    z-index: 100;
    display: flex;
    justify-content: center; /* Center horizontally */
    align-items: flex-start; /* Crucial: Prevents flexbox from clipping bottom padding when scrolling */
    color: #e3e3e3;
    overflow-y: auto;
    overflow-x: hidden; /* CRITICAL FIX: Guarantee no horizontal scrollbars */
    font-family: 'Segoe UI', system-ui, sans-serif;
    animation: fadeIn 0.2s ease-out;
}

.settings-modal-inner {
    width: 100%;
    max-width: 600px; /* Luxurious breathing room */
    padding: 32px 24px 24px 24px; /* Perfectly balanced bottom padding */
    display: flex;
    flex-direction: column;
    box-sizing: border-box; /* CRITICAL FIX: prevents horizontal scrollbar on narrow window */
}

.settings-modal::-webkit-scrollbar { width: 6px; }
.settings-modal::-webkit-scrollbar-track { background: transparent; }
.settings-modal::-webkit-scrollbar-thumb { background-color: rgba(255,255,255,0.2); border-radius: 10px; }
.settings-modal::-webkit-scrollbar-thumb:hover { background-color: rgba(255,255,255,0.3); }

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}

.settings-header { 
    display: flex; 
    justify-content: space-between; 
    align-items: center; 
    margin-bottom: 32px; 
}

.settings-header h2 { 
    margin: 0; 
    font-size: 28px; 
    font-weight: 600; 
    letter-spacing: -0.5px;
}

.version-text { 
    font-size: 13px; 
    color: #8ab4f8; 
    background-color: rgba(138, 180, 248, 0.1);
    padding: 4px 8px;
    border-radius: 6px;
    font-weight: 500;
}

.settings-content { 
    flex: 1; 
    display: flex; 
    flex-direction: column; 
    gap: 0; /* Gaps are handled by group margins now */
}

.settings-footer { 
    margin-top: 16px; /* Reduced gap between Application section and Done button */
    margin-bottom: 0;
    display: flex; 
    width: 100%;
}

.primary-btn { 
    background: #e3e3e3;
    color: #131314; 
    border: none; 
    border-radius: 12px; 
    padding: 16px 24px; 
    width: 100%;
    font-size: 16px; 
    font-weight: 600; 
    cursor: pointer; 
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 12px rgba(255, 255, 255, 0.05);
}

.primary-btn:hover { 
    background: #ffffff;
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(255, 255, 255, 0.1);
}

.primary-btn:active {
    transform: scale(0.96);
}

.custom-dialog-overlay { 
    z-index: 1000; 
    align-items: center; 
    justify-content: center; 
    background-color: rgba(0, 0, 0, 0.8); 
    display: flex; 
}

.custom-dialog-box { 
    background-color: #1e1e1e; 
    padding: 24px; 
    border-radius: 16px; 
    border: 1px solid rgba(255, 255, 255, 0.1); 
    width: 80%; 
    max-width: 320px; 
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6); 
}

.custom-dialog-box h3 { 
    margin: 0 0 12px 0; 
    font-size: 18px; 
    font-weight: 600; 
}

.custom-dialog-box p { 
    margin: 0 0 24px 0; 
    font-size: 14px; 
    color: #9aa0a6; 
    line-height: 1.5; 
}

.custom-dialog-actions { 
    display: flex; 
    gap: 12px; 
    justify-content: flex-end; 
}

.pill { 
    background: transparent; 
    border: 1px solid rgba(255,255,255,0.2); 
    color: #e3e3e3; 
    padding: 8px 16px; 
    border-radius: 8px; 
    font-size: 14px; 
    cursor: pointer; 
    transition: all 0.2s; 
}

.pill:hover { background-color: rgba(255, 255, 255, 0.05); }

.pill.active { 
    background-color: rgba(66, 133, 244, 0.15); 
    border-color: #4285f4; 
    color: #8ab4f8; 
}
</style>
