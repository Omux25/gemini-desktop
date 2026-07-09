pub mod state;
pub mod process;
pub mod ipc;
pub mod config;
pub mod tray;
pub mod updater;

use std::sync::atomic::Ordering;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    let is_smooth_mode = config::load_smooth_mode();
    config::apply_chromium_flags(is_smooth_mode);

    let app_state = AppState::default();
    app_state.smooth_mode.store(is_smooth_mode, Ordering::Relaxed);

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::all() 
                    & !tauri_plugin_window_state::StateFlags::VISIBLE
                )
                .build()
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ipc::system::greet,
            ipc::window::webview_back,
            ipc::window::webview_reload,
            ipc::window::hide_webview,
            ipc::window::show_webview,
            ipc::window::sync_visibility,
            ipc::window::sync_pinned,
            ipc::window::inject_text,
            process::optimize_memory,
            ipc::settings::change_hotkey,
            ipc::system::restart_app,
            ipc::settings::set_smooth_mode,
            ipc::settings::set_auto_hide,
            ipc::settings::set_custom_prompt,
            ipc::settings::load_settings,
            ipc::settings::save_settings,
            ipc::window::frontend_ready,
            updater::is_portable,
            updater::download_portable_update,
            updater::install_portable_update
        ])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
                use tauri::Manager;
                let app = window.app_handle();
                let state = app.state::<crate::state::AppState>();
                state.window_visible.store(false, Ordering::Release);
                crate::process::optimize_memory(app.clone());
            }
            tauri::WindowEvent::Focused(focused) => {
                use tauri::Manager;
                let app = window.app_handle();
                let state = app.state::<crate::state::AppState>();
                state.window_focused.store(*focused, Ordering::Release);
                
                if !focused {
                    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
                    let last_shown = state.last_shown_time.load(Ordering::Acquire);
                    let startup = state.startup_time.load(Ordering::Acquire);
                    
                    // Do not auto-hide for the first 3 seconds after the app launches.
                    // This prevents terminal windows or other background startup processes from instantly stealing focus and hiding the app.
                    if now.saturating_sub(startup) < 3000 {
                        return;
                    }
                    
                    // Reduced grace period to 150ms. 
                    // This prevents the bug where clicking away quickly traps the window in an unfocused state.
                    if now.saturating_sub(last_shown) > 150 {
                        let is_pinned = state.window_pinned.load(Ordering::Acquire);
                        
                        if !is_pinned {
                            let _ = window.set_always_on_top(false);
                        }
                        
                        let is_auto_hide = state.auto_hide.load(Ordering::Acquire);
                        if is_auto_hide && !is_pinned {
                            let _ = window.hide();
                            state.window_visible.store(false, Ordering::Release);
                            crate::process::optimize_memory(app.clone());
                        }
                    } else {
                        // OS Glitch: It lost focus immediately.
                        // We must reclaim focus so winit doesn't permanently trap the window in an unfocused state!
                        // Calling window.set_focus() instead of WinAPI SetFocus ensures the WebView child gets the focus
                        let _ = window.set_focus();
                    }
                }
            }
            _ => {}
        })
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState, Shortcut};
            
            let default_toggle = "Alt+Space";
            let default_copy = "Alt+C";
            let default_snip = "Alt+S";

            if let Ok(key) = default_toggle.parse::<Shortcut>() {
                if !app.global_shortcut().is_registered(key) {
                    let _ = app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            crate::ipc::window::toggle_window(app_handle);
                        }
                    });
                }
            }
            
            if let Ok(key) = default_copy.parse::<Shortcut>() {
                if !app.global_shortcut().is_registered(key) {
                    let _ = app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            crate::ipc::window::grab_text_and_toggle_window(app_handle);
                        }
                    });
                }
            }
            
            if let Ok(key) = default_snip.parse::<Shortcut>() {
                if !app.global_shortcut().is_registered(key) {
                    let _ = app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            crate::ipc::window::start_snip_mode(app_handle);
                        }
                    });
                }
            }
            
            use tauri::Manager;
            let state = app.state::<crate::state::AppState>();
            if let Ok(mut locked) = state.hotkey_toggle.lock() { *locked = default_toggle.to_string(); }
            if let Ok(mut locked) = state.hotkey_copy.lock() { *locked = default_copy.to_string(); }
            if let Ok(mut locked) = state.hotkey_snip.lock() { *locked = default_snip.to_string(); }

            tray::create_tray(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, _event| {});
}
