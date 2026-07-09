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

<div class="titlebar" id="titlebar" data-tauri-drag-region>
  <div class="titlebar-left-controls">
      <div class="nav-controls">
          <button class="app-btn" title="Go Back" onclick={handleBack}>
              <BackIcon />
          </button>
          <button class="app-btn" title="Reload Page" onclick={handleRefresh}>
              <RefreshIcon />
          </button>
      </div>
      
      <div class="divider"></div>
      
      <div class="app-controls">
          <button class="app-btn" title="Open Settings" onclick={toggleSettings}>
              <SettingsIcon />
          </button>
          <button class="app-btn pin-btn" class:active={$isPinned} title="Toggle Always on Top" onclick={handlePin}>
              <PinIcon />
          </button>
      </div>
  </div>
  
  <div class="titlebar-title">
      <img src={iconPath} class="titlebar-icon" alt="Icon">
      <span class="titlebar-text">Gemini Desktop</span>
  </div>
  
  <div class="window-controls">
      <button class="win-btn" title="Minimize" onclick={handleMinimize}>
          <MinimizeIcon />
      </button>
      <button class="win-btn" title="Maximize" onclick={handleMaximize}>
          <MaximizeIcon />
      </button>
      <button class="win-btn close" title="Close" onclick={handleClose}>
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
      align-items: stretch;
      border-bottom: 1px solid rgba(255, 255, 255, 0.04);
      color: #e3e3e3;
      user-select: none;
      position: relative;
      z-index: 10;
  }

  /* Absolute Centered Title */
  .titlebar-title {
      position: absolute;
      left: 50%;
      top: 0;
      bottom: 0;
      transform: translateX(-50%);
      font-size: 14.5px;
      font-family: 'Segoe UI', system-ui, sans-serif;
      font-weight: 500;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
      letter-spacing: 0.4px;
      pointer-events: none; /* Let drag events pass through to the parent bar */
      white-space: nowrap; /* Prevent wrapping */
  }

  .titlebar-text {
      color: #e3e3e3;
  }

  /* Responsive Titlebar */
  @media (max-width: 450px) {
      .titlebar-text {
          display: none; /* Only show the logo icon on narrow screens to avoid smashing */
      }
  }

  .titlebar-icon {
      width: 18px;
      height: 18px;
      object-fit: contain;
  }

  .titlebar-left-controls {
      display: flex;
      align-items: center;
      padding-left: 8px;
      z-index: 2;
  }

  .nav-controls, .app-controls {
      display: flex;
      align-items: center;
      gap: 4px;
  }

  /* Vertical Divider */
  .divider {
      width: 1px;
      height: 14px;
      background-color: rgba(255, 255, 255, 0.15);
      margin: 0 8px;
      border-radius: 1px;
  }

  .window-controls {
      display: flex;
      align-items: stretch;
      height: 100%;
      z-index: 2;
  }

  /* Premium App Buttons (Rounded, Soft Hover) */
  .app-btn {
      width: 30px;
      height: 30px;
      display: flex;
      justify-content: center;
      align-items: center;
      cursor: pointer;
      border-radius: 6px;
      border: none;
      background: transparent;
      padding: 0;
      transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .app-btn:hover {
      background-color: rgba(255, 255, 255, 0.08);
  }

  .app-btn:active {
      transform: scale(0.92);
      background-color: rgba(255, 255, 255, 0.12);
  }

  /* Refined Glassmorphic Glow for Active Pin */
  .app-btn.pin-btn.active {
      background-color: rgba(255, 255, 255, 0.1);
      box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.05);
  }

  :global(.app-btn.pin-btn.active svg) {
      fill: #ffffff;
      filter: drop-shadow(0 0 4px rgba(162, 89, 255, 0.8));
  }

  /* Native Windows 11 Window Controls */
  .win-btn {
      width: 46px;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      cursor: pointer;
      border-radius: 0; /* Flush edges */
      border: none;
      background: transparent;
      padding: 0;
      transition: background-color 0.1s ease;
  }

  .win-btn:hover {
      background-color: rgba(255, 255, 255, 0.1);
  }

  .win-btn.close:hover {
      background-color: #e81123;
  }

  /* SVG Styling */
  :global(.app-btn svg) {
      width: 14px;
      height: 14px;
      fill: #9aa0a6;
      transition: fill 0.2s;
  }

  :global(.app-btn:hover svg) {
      fill: #ffffff;
  }

  :global(.win-btn svg) {
      width: 14px; /* Correct Windows 11 proportion */
      height: 14px;
      fill: #9aa0a6;
      transition: fill 0.1s;
  }

  :global(.win-btn:hover svg) {
      fill: #ffffff;
  }

  :global(.win-btn.close:hover svg) {
      fill: #ffffff;
  }
</style>
