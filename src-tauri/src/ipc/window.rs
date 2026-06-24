use tauri::{Manager, Emitter};
use std::sync::atomic::Ordering;
use crate::state::AppState;
use crate::process::optimize_memory;

trait WindowExt {
    fn show_and_focus(&self) -> tauri::Result<()>;
}

impl WindowExt for tauri::Window {
    fn show_and_focus(&self) -> tauri::Result<()> {
        self.show()?;
        self.unminimize()?;
        self.set_focus()
    }
}

#[tauri::command]
pub fn frontend_ready(app: tauri::AppHandle) {
    if let Some(window) = app.get_window("main") {
        if let Err(e) = window.show_and_focus() { eprintln!("Failed to show window: {:?}", e); }
        let state = app.state::<crate::state::AppState>();
        state.window_visible.store(true, Ordering::Release);
        state.window_focused.store(true, Ordering::Release);
    }
}

#[tauri::command]
pub fn sync_visibility(state: tauri::State<'_, AppState>, visible: bool) {
    state.window_visible.store(visible, Ordering::Relaxed);
}

#[tauri::command]
pub fn sync_pinned(state: tauri::State<'_, AppState>, pinned: bool) {
    state.window_pinned.store(pinned, Ordering::Relaxed);
}

const WEBVIEW_BACK_SCRIPT: &str = include_str!("../scripts/webview_back.js");
const WEBVIEW_RELOAD_SCRIPT: &str = include_str!("../scripts/webview_reload.js");

#[tauri::command]
pub fn webview_back(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.eval(WEBVIEW_BACK_SCRIPT) { eprintln!("Failed to eval: {:?}", e); }
    }
}

#[tauri::command]
pub fn hide_webview(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.hide() { eprintln!("Failed to hide webview: {:?}", e); }
    }
}

#[tauri::command]
pub fn show_webview(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.show() { eprintln!("Failed to show webview: {:?}", e); }
    }
}

#[tauri::command]
pub fn webview_reload(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.eval(WEBVIEW_RELOAD_SCRIPT) { eprintln!("Failed to eval: {:?}", e); }
    }
}

pub fn toggle_window(app: &tauri::AppHandle) {
    let state = app.state::<AppState>();
    let is_visible = state.window_visible.load(Ordering::Acquire);
    let is_pinned = state.window_pinned.load(Ordering::Relaxed);
    let is_focused = state.window_focused.load(Ordering::Relaxed);
    
    let main_windows: Vec<_> = app.windows().into_iter()
        .filter(|(label, _)| !label.starts_with("plugin:"))
        .map(|(_, window)| window)
        .collect();

    if !is_visible {
        for window in &main_windows {
            if let Err(e) = window.show_and_focus() { eprintln!("Failed to show window: {:?}", e); }
            if let Err(e) = window.emit("request-show", ()) { eprintln!("Failed to emit event: {:?}", e); }
        }
        state.window_visible.store(true, Ordering::Release);
        state.window_focused.store(true, Ordering::Release);
    } else {
        let should_hide = is_focused;
        
        if should_hide {
            for window in &main_windows {
                if let Err(e) = window.hide() { eprintln!("Failed to hide window: {:?}", e); }
            }
            state.window_visible.store(false, Ordering::Release);
            state.window_focused.store(false, Ordering::Release);
            optimize_memory(app.clone());
        } else {
            for window in &main_windows {
                if let Err(e) = window.show_and_focus() { eprintln!("Failed to show window: {:?}", e); }
                if let Err(e) = app.emit("tray_state_changed", true) { eprintln!("Failed to emit event: {:?}", e); }
                if let Err(e) = window.emit("request-show", ()) { eprintln!("Failed to emit event: {:?}", e); }
            }
        }
    }
}
