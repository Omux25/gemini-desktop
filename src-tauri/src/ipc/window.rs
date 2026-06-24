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
        let _ = window.show_and_focus();
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
        let _ = webview.eval(WEBVIEW_BACK_SCRIPT);
    }
}

#[tauri::command]
pub fn hide_webview(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        let _ = webview.hide();
    }
}

#[tauri::command]
pub fn show_webview(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        let _ = webview.show();
    }
}

#[tauri::command]
pub fn webview_reload(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        let _ = webview.eval(WEBVIEW_RELOAD_SCRIPT);
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
            let _ = window.show_and_focus();
            let _ = window.emit("request-show", ());
        }
        state.window_visible.store(true, Ordering::Release);
        state.window_focused.store(true, Ordering::Release);
    } else {
        let should_hide = is_focused || is_pinned;
        
        if should_hide {
            for window in &main_windows {
                let _ = window.hide();
            }
            state.window_visible.store(false, Ordering::Release);
            state.window_focused.store(false, Ordering::Release);
            optimize_memory(app.clone());
        } else {
            for window in &main_windows {
                let _ = window.show_and_focus();
                let _ = app.emit("tray_state_changed", true);
                let _ = window.emit("request-show", ());
            }
        }
    }
}
