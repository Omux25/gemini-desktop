<div align="center">
  <img src="https://raw.githubusercontent.com/Omux25/gemini-desktop/master/public/icon.png" alt="Gemini Desktop Icon" width="128" />

  <h1>Gemini Desktop v2.1.0</h1>

  <p><strong>The official Rust/Tauri V1 Architecture Release.</strong></p>
</div>

We are thrilled to announce the official `2.1.0` release of Gemini Desktop! This version marks the transition from our legacy Node.js/Electron architecture to a pure, highly optimized **Rust** and **Tauri** backend. 

### ✨ What's New in 2.1.0

* **Extremely Low Memory Footprint:** By replacing Electron with the native Windows Webview2 engine and implementing an OS-level `EmptyWorkingSet` memory flush, background memory usage has been reduced by over 90%.
* **Global Hotkey Overhaul:** Summon or dismiss Gemini instantly with the new default hotkey: `Alt + Space`. It intercepts inputs globally and perfectly overlays full-screen exclusive games.
* **Modular Enterprise UI:** The Svelte 5 frontend has been completely rewritten into a stateless, strictly decoupled component architecture, offering instantaneous reactivity and smooth animations.
* **Intelligent Window Placement:** The application tracks your exact geometry across sessions, seamlessly respawning exactly where and how you left it.

### 📥 Downloads

Choose the version that fits your needs:
* `Gemini Desktop_2.1.0_x64-setup.exe` - The standard Windows Installer.
* `Gemini-Desktop-Portable.exe` - A standalone, portable executable that requires no installation.

> **Note to Windows Users:** As an indie open-source project without a paid EV Code Signing certificate, Microsoft Defender SmartScreen will initially flag these executables. Click **More Info** -> **Run anyway** to proceed.
