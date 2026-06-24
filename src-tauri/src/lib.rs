pub mod state;
pub mod process;
pub mod ipc;
pub mod config;
pub mod tray;

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
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::all() 
                    & !tauri_plugin_window_state::StateFlags::VISIBLE
                )
                .build()
        )
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ipc::system::greet,
            ipc::window::webview_back,
            ipc::window::webview_reload,
            ipc::window::hide_webview,
            ipc::window::show_webview,
            ipc::window::sync_visibility,
            ipc::window::sync_pinned,
            process::optimize_memory,
            ipc::settings::change_hotkey,
            ipc::system::restart_app,
            ipc::settings::set_smooth_mode,
            ipc::settings::load_settings,
            ipc::settings::save_settings,
            ipc::window::frontend_ready
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
                window.state::<crate::state::AppState>().window_focused.store(*focused, Ordering::Release);
            }
            _ => {}
        })
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState, Shortcut};
            
            let default_hotkey = "Alt+Space";
            if let Ok(key) = default_hotkey.parse::<Shortcut>() {
                if !app.global_shortcut().is_registered(key) {
                    let _ = app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                        if event.state() == ShortcutState::Released {
                            crate::ipc::window::toggle_window(app_handle);
                        }
                    });
                }
            }
            
            use tauri::Manager;
            if let Ok(mut locked) = app.state::<crate::state::AppState>().global_hotkey.lock() {
                *locked = default_hotkey.to_string();
            } else {
                log::error!("Failed to acquire lock for global hotkey state");
            }

            tray::create_tray(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, _event| {});
}
