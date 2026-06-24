<div align="center">
  <img src="./public/icon.png" alt="Gemini Desktop Icon" width="128" />

  <h1>Gemini Desktop</h1>

  <p><strong>A highly optimized, enterprise-grade native desktop wrapper for Google Gemini.</strong></p>

  <a href="https://github.com/Omux25/gemini-desktop/releases/latest">
    <img src="https://img.shields.io/github/v/release/Omux25/gemini-desktop?style=for-the-badge&color=007AFF&label=Latest%20Release" alt="Latest Release" />
  </a>
  <br>
  <br>
  <a href="https://github.com/Omux25/gemini-desktop/releases/latest">
    <img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="Windows" />
  </a>
  <a href="https://github.com/Omux25/gemini-desktop/releases/latest">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Backend" />
  </a>
  <a href="https://github.com/Omux25/gemini-desktop/releases/latest">
    <img src="https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" alt="Svelte Frontend" />
  </a>
</div>

<br>

Welcome to the next generation of **Gemini Desktop**. Completely re-architected from the ground up, moving away from Electron to a blazingly fast, lightweight **Rust & Tauri** core. It brings Google Gemini out of your browser tabs and turns it into a native, powerful desktop assistant.

## ✨ Features

- **Smart Global Hotkey:** Instantly summon or hide the chat window from anywhere without interrupting your workflow.
  - Global Default: `Alt + Space`
- **Always on Top (Pinned):** Pin the window so you can easily reference Gemini while gaming, coding, or working. 
- **Smooth Mode (Memory Optimization):** Advanced dynamic Webview2 flag injection (`--enable-low-end-device-mode`, `--renderer-process-limit=1`) alongside a native OS memory flushing algorithm (`EmptyWorkingSet`) to achieve extremely low RAM footprints when idling.
- **Native System Tray:** Runs quietly in the background, completely out of your way until summoned.
- **Enterprise-Grade Architecture:** Features a modular Svelte 5 frontend with strict service-layer isolation and a pure Rust IPC architecture. 

## 📥 Download & Install

Download the latest version for your operating system from the **[Releases Page](https://github.com/Omux25/gemini-desktop/releases)**.

## 🛠️ Building from Source

This project uses an automated Vite + Tauri build pipeline.

1. Ensure you have [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:
   ```bash
   git clone https://github.com/Omux25/gemini-desktop.git
   cd gemini-desktop
   ```
3. Install the dependencies:
   ```bash
   npm install
   ```
4. Run the development server:
   ```bash
   npm run tauri dev
   ```
5. Build the application for production:
   ```bash
   npm run tauri build
   ```

## 🏗 Architecture & IPC

This application enforces a strict separation of concerns:
* **Frontend:** Built with vanilla Svelte 5. State management and IPC invocations are isolated into dedicated service singletons (`settingsService`, `windowService`) to ensure UI components remain pure and stateless.
* **Backend:** Built with Rust. The monolithic command structure is broken down into domain-specific modules (`ipc::window`, `ipc::settings`, `ipc::system`).

## 📜 License

This project is licensed under the **PolyForm Noncommercial License 1.0.0**.
You are completely free to use, modify, distribute, and build upon this code for non-commercial purposes only.
