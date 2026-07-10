#[tauri::command]
pub fn restart_app(app: tauri::AppHandle) {
    use tauri_plugin_window_state::AppHandleExt;
    let _ = app.save_window_state(tauri_plugin_window_state::StateFlags::all());
    app.restart();
}
