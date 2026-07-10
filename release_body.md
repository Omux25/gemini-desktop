# Gemini Desktop v2.2.0

**The 100% Pure Enterprise Architecture & Hardened IPC Release.**

We are excited to announce **Gemini Desktop v2.2.0**! This major architecture release completes our thorough engineering audit (`7/7 PASS` across all forensic software rigor benchmarks), delivering zero-overhead event-driven responsiveness, strict TypeScript type safety across every Svelte 5 service, and enhanced OS integration.

✨ **What's New & Hardened in v2.2.0**

- **Event-Driven DOM Focus Engine (`MutationObserver`):** Replaced legacy interval polling inside injected Webview scripts with native, highly reactive DOM mutation observing. Text inputs focus immediately and zero polling timers run in the background.
- **100% Type-Safe Svelte 5 IPC Bridge:** Completely eradicated all `any` and `@ts-ignore` type escape hatches across our frontend services and state stores (`updaterService.ts`, `settingsService.ts`, `SettingsModal.svelte`). All IPC payloads strictly conform to `@tauri-apps/plugin-updater` and internal union definitions.
- **Single Source of Truth Configuration (`config.rs`):** Centralized app bundle identifiers (`com.omux25.geminidesktop`) and storage directory derivation under a unified Rust configuration layer to guarantee zero filesystem path drift.
- **Optimized Keyboard & Clipboard Synchronization:** Replaced global 248-key WinAPI scanning loops with bounded, targeted modifier release checks (`VK_MENU`, `VK_CONTROL`, `VK_SHIFT`, `VK_LWIN`, `VK_RWIN`), eliminating CPU polling overhead before window transitions.
- **Clean Root Hygiene:** Purged multi-megabyte testing and bootstrap binaries from the project structure, delivering a ultra-lean repository and release package.

📦 **Downloads & Portable Version**

Choose the package tailored to your workflow:

- `Gemini.Desktop_2.2.0_x64-setup.exe` - The standard Windows Installer (`NSIS`), recommended for automatic startup and background tray support.
- `Gemini-Desktop-Portable.exe` - A standalone, zero-install portable executable that runs instantly anywhere from a folder or USB drive.

*Note to Windows Users: As an indie open-source project without a paid EV Code Signing certificate, Microsoft Defender SmartScreen will initially prompt on new release builds. Click "More Info -> Run anyway" to proceed.*
