<script lang="ts">
  import { isSettingsVisible, isPinned } from '../store';
  import { windowService } from '../services/windowService';
  import { settingsService } from '../services/settingsService';
  import iconPath from '../../src-tauri/icons/32x32.png';

  import BackIcon from './icons/BackIcon.svelte';
  import RefreshIcon from './icons/RefreshIcon.svelte';
  import SettingsIcon from './icons/SettingsIcon.svelte';
  import PinIcon from './icons/PinIcon.svelte';
  import MinimizeIcon from './icons/MinimizeIcon.svelte';
  import MaximizeIcon from './icons/MaximizeIcon.svelte';
  import CloseIcon from './icons/CloseIcon.svelte';

  const handleBack = async () => {
    if ($isSettingsVisible) {
      isSettingsVisible.set(false);
    } else {
      await windowService.webviewBack();
    }
  };

  const handleRefresh = async () => windowService.webviewReload();
  const toggleSettings = () => isSettingsVisible.update(v => !v);
  const handleMinimize = async () => windowService.minimize();
  const handleMaximize = async () => windowService.maximize();
  const handleClose = async () => windowService.close();
  
  const handlePin = () => {
    isPinned.update(v => {
      const newState = !v;
      settingsService.setPinned(newState).catch(console.error);
      return newState;
    });
  };
</script>

<div class="titlebar" id="titlebar">
  <div class="titlebar-left-controls">
      <button class="control-btn" title="Go Back" onclick={handleBack}>
          <BackIcon />
      </button>
      <button class="control-btn" title="Reload Page" onclick={handleRefresh}>
          <RefreshIcon />
      </button>
  </div>
  <div class="titlebar-title" data-tauri-drag-region>
      <img src={iconPath} class="titlebar-icon" alt="Icon">
      <span class="titlebar-text">Gemini Desktop</span>
  </div>
  <div class="titlebar-controls">
      <button class="control-btn" title="Open Settings" onclick={toggleSettings}>
          <SettingsIcon />
      </button>
      <button class="control-btn" class:active={$isPinned} title="Toggle Always on Top" onclick={handlePin}>
          <PinIcon />
      </button>
      <button class="control-btn" title="Minimize" onclick={handleMinimize}>
          <MinimizeIcon />
      </button>
      <button class="control-btn" title="Maximize" onclick={handleMaximize}>
          <MaximizeIcon />
      </button>
      <button class="control-btn close" title="Close" onclick={handleClose}>
          <CloseIcon />
      </button>
  </div>
</div>

<style>
  .titlebar {
      height: 38px;
      background-color: #131314;
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-bottom: 1px solid rgba(255, 255, 255, 0.04);
      color: #e3e3e3;
      user-select: none;
      position: relative;
      z-index: 10;
  }

  .titlebar-title {
      padding-left: 10px;
      font-size: 13px;
      font-weight: 500;
      display: flex;
      align-items: center;
      gap: 10px;
      letter-spacing: 0.3px;
      height: 100%;
      flex: 1;
      justify-content: flex-start;
  }

  .titlebar-text {
      pointer-events: none;
  }

  .titlebar-icon {
      width: 16px;
      height: 16px;
      object-fit: contain;
      margin-right: 6px;
      pointer-events: none;
  }

  .titlebar-controls {
      display: flex;
      align-items: center;
      height: 100%;
      padding-right: 6px;
      gap: 4px;
      z-index: 2;
      position: relative;
  }

  .titlebar-left-controls {
      display: flex;
      gap: 2px;
      padding-left: 6px;
      z-index: 2;
      position: relative;
  }

  .control-btn {
      width: 32px;
      height: 28px;
      display: flex;
      justify-content: center;
      align-items: center;
      cursor: pointer;
      border-radius: 6px;
      border: none;
      transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
      background: transparent;
      padding: 0;
  }

  .control-btn:hover {
      background-color: rgba(255, 255, 255, 0.08);
      transform: scale(1.05);
  }

  .control-btn:active {
      transform: scale(0.95);
  }

  .control-btn.close:hover {
      background-color: #e81123;
      color: white;
      box-shadow: 0 0 8px rgba(232, 17, 35, 0.5);
  }

  :global(.control-btn svg) {
      width: 14px;
      height: 14px;
      fill: #9aa0a6;
      stroke: none;
      transition: fill 0.2s;
  }

  :global(.titlebar-icon) {
      stroke: #9aa0a6;
      fill: none;
  }

  :global(.control-btn:hover svg) {
      fill: #ffffff;
  }

  :global(.control-btn.close:hover svg) {
      fill: #ffffff;
  }

  .control-btn.active {
      background-color: rgba(66, 133, 244, 0.15);
  }

  :global(.control-btn.active svg) {
      fill: #669df6; 
      filter: drop-shadow(0 0 3px rgba(102, 157, 246, 0.5));
  }
</style>
