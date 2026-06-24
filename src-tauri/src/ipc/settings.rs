use std::sync::atomic::Ordering;
use tauri_plugin_global_shortcut::ShortcutState;
use crate::state::AppState;
use crate::ipc::window::toggle_window;

#[tauri::command]
pub fn set_smooth_mode(state: tauri::State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state.smooth_mode.store(enabled, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn change_hotkey(app: tauri::AppHandle, state: tauri::State<'_, AppState>, old_hotkey: String, new_hotkey: String) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
    
    if old_hotkey != new_hotkey && !old_hotkey.is_empty() {
        if let Ok(old_key) = old_hotkey.parse::<Shortcut>() {
            let _ = app.global_shortcut().unregister(old_key);
        }
    }

    if !new_hotkey.is_empty() {
        let key = new_hotkey.parse::<Shortcut>().map_err(|e| e.to_string())?;
        if !app.global_shortcut().is_registered(key) {
            app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                if event.state() == ShortcutState::Released {
                    toggle_window(app_handle);
                }
            }).map_err(|e| e.to_string())?;
        }
    }

    if let Ok(mut locked) = state.global_hotkey.lock() {
        *locked = new_hotkey;
    }
    
    Ok(())
}
