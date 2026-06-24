# Gemini Desktop v1.0.0

**The official Rust/Tauri V1 Architecture Release.**

We are thrilled to announce the official `1.0.0` release of Gemini Desktop! This version marks the transition from our legacy Node.js/Electron architecture to a pure, highly optimized Rust and Tauri backend. 

✨ **What's New in 1.0.0**

- **Native Rust & Tauri Architecture:** Transitioning to Windows Webview2 removes the heavy overhead of bundling an entire browser, resulting in a significantly leaner desktop application.
- **Global Hotkey Integration:** Summon or dismiss Gemini instantly with the new default hotkey: `Alt + Space`. It intercepts inputs globally and smoothly overlays your workflow.
- **Modern Svelte 5 UI:** The frontend has been completely rewritten with Svelte 5 to offer a beautiful, instantly reactive interface with smooth animations and dynamic resizing.
- **Rock-Solid Settings Persistence:** A newly written native Rust IPC pipeline guarantees that your customized window sizes, pinned states, and hardware acceleration preferences perfectly restore exactly the way you left them.

📦 **Downloads**

Choose the version that fits your needs:

- `Gemini-Desktop-Setup.exe` - The standard Windows Installer.
- `Gemini-Desktop-Portable.exe` - A standalone, portable executable that requires no installation.

*Note to Windows Users: As an indie open-source project without a paid EV Code Signing certificate, Microsoft Defender SmartScreen will initially flag these executables. Click "More Info -> Run anyway" to proceed.*
