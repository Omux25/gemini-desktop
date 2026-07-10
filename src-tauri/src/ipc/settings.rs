use std::sync::atomic::Ordering;
use tauri_plugin_global_shortcut::ShortcutState;
use crate::state::AppState;

#[tauri::command]
pub fn set_smooth_mode(state: tauri::State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state.smooth_mode.store(enabled, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_auto_hide(state: tauri::State<'_, AppState>, enabled: bool) -> Result<(), String> {
    state.auto_hide.store(enabled, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_custom_prompt(state: tauri::State<'_, AppState>, prompt: String) -> Result<(), String> {
    if let Ok(mut locked) = state.custom_prompt.lock() {
        *locked = prompt;
    }
    Ok(())
}

#[tauri::command]
pub fn change_hotkey(app: tauri::AppHandle, state: tauri::State<'_, AppState>, action: String, old_hotkey: String, new_hotkey: String) -> Result<(), String> {
    use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
    
    if old_hotkey != new_hotkey && !old_hotkey.is_empty() {
        if let Ok(old_key) = old_hotkey.parse::<Shortcut>() {
            let _ = app.global_shortcut().unregister(old_key);
        }
    }

    if !new_hotkey.is_empty() {
        let key = new_hotkey.parse::<Shortcut>().map_err(|e| e.to_string())?;
        if !app.global_shortcut().is_registered(key) {
            let action_clone = action.clone();
            app.global_shortcut().on_shortcut(key, move |app_handle, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    match action_clone.as_str() {
                        "toggle" => crate::ipc::window::toggle_window(app_handle),
                        "copy" => crate::ipc::window::grab_text_and_toggle_window(app_handle),
                        "snip" => crate::ipc::window::start_snip_mode(app_handle),
                        _ => (),
                    }
                }
            }).map_err(|e| e.to_string())?;
        }
    }

    match action.as_str() {
        "toggle" => { if let Ok(mut locked) = state.hotkey_toggle.lock() { *locked = new_hotkey; } },
        "copy" => { if let Ok(mut locked) = state.hotkey_copy.lock() { *locked = new_hotkey; } },
        "snip" => { if let Ok(mut locked) = state.hotkey_snip.lock() { *locked = new_hotkey; } },
        _ => return Err("Invalid hotkey action".to_string()),
    }
    
    Ok(())
}

#[tauri::command]
pub fn load_settings() -> Result<String, String> {
    if let Some(file_path) = crate::config::get_settings_file_path() {
        if let Ok(contents) = std::fs::read_to_string(file_path) {
            return Ok(contents);
        }
    }
    Ok("{}".to_string())
}

#[tauri::command]
pub fn save_settings(settings_json: String) -> Result<(), String> {
    if let Some(file_path) = crate::config::get_settings_file_path() {
        if let Some(parent) = file_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(file_path, settings_json);
    }
    Ok(())
}
