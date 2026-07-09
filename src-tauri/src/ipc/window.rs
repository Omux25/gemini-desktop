use tauri::{Manager, Emitter};
use std::sync::atomic::Ordering;
use crate::state::AppState;
use crate::process::optimize_memory;

trait WindowExt {
    fn show_and_focus(&self) -> tauri::Result<()>;
}

impl WindowExt for tauri::Window {
    fn show_and_focus(&self) -> tauri::Result<()> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        self.app_handle().state::<crate::state::AppState>().last_shown_time.store(now, std::sync::atomic::Ordering::Release);

        self.show()?;
        self.unminimize()?;
        let _ = self.set_always_on_top(true);
        let is_pinned = self.app_handle().state::<crate::state::AppState>().window_pinned.load(std::sync::atomic::Ordering::Acquire);
        
        // Use WinAPI to forcefully steal focus from the previous foreground app
        if let Ok(hwnd) = self.hwnd() {
            #[cfg(target_os = "windows")]
            unsafe {
                use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId, AttachThreadInput, SetForegroundWindow, BringWindowToTop};
                use winapi::um::processthreadsapi::GetCurrentThreadId;
                use winapi::shared::windef::HWND;
                
                let hwnd = hwnd.0 as HWND;
                let foreground_window = GetForegroundWindow();
                let foreground_thread = GetWindowThreadProcessId(foreground_window, std::ptr::null_mut());
                let current_thread = GetCurrentThreadId();
                
                if foreground_thread != current_thread {
                    AttachThreadInput(current_thread, foreground_thread, 1);
                    SetForegroundWindow(hwnd);
                    BringWindowToTop(hwnd);
                    AttachThreadInput(current_thread, foreground_thread, 0);
                } else {
                    SetForegroundWindow(hwnd);
                    BringWindowToTop(hwnd);
                }
            }
        }
        
        // Now that the window is forcefully in the foreground, ask Tauri to focus it 
        // (this properly routes keyboard focus to the inner WebView)
        let _ = self.set_focus();
        
        let window_clone = self.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            if !is_pinned {
                let _ = window_clone.set_always_on_top(false);
            }
        });

        Ok(())
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
const WEBVIEW_FOCUS_SCRIPT: &str = include_str!("../scripts/webview_focus.js");

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
    if let Some(window) = app.get_window("main") {
        let _ = window.set_focus();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        app.state::<crate::state::AppState>().last_shown_time.store(now, std::sync::atomic::Ordering::Release);
    }
}

#[tauri::command]
pub fn show_webview(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.show() { eprintln!("Failed to show webview: {:?}", e); }
        let _ = webview.set_focus();
        if let Err(e) = webview.eval(WEBVIEW_FOCUS_SCRIPT) { eprintln!("Failed to eval focus script: {:?}", e); }
        
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        app.state::<crate::state::AppState>().last_shown_time.store(now, std::sync::atomic::Ordering::Release);
    }
}

#[tauri::command]
pub fn webview_reload(app: tauri::AppHandle) {
    if let Some(webview) = app.get_webview("gemini") {
        if let Err(e) = webview.eval(WEBVIEW_RELOAD_SCRIPT) { eprintln!("Failed to eval: {:?}", e); }
    }
}

#[tauri::command]
pub fn inject_text(app: tauri::AppHandle, text: String) {
    if let Some(webview) = app.get_webview("gemini") {
        let script = format!(r#"
            (function() {{
                function findFocusableInput(root) {{
                    let input = root.querySelector('rich-textarea div[contenteditable="true"]') 
                             || root.querySelector('rich-textarea p')
                             || root.querySelector('rich-textarea [contenteditable="true"]')
                             || root.querySelector('textarea') 
                             || root.querySelector('[role="textbox"]')
                             || root.querySelector('[contenteditable="true"]');
                    if (input) return input;
                    const allElements = root.querySelectorAll('*');
                    for (const el of allElements) {{
                        if (el.shadowRoot) {{
                            const found = findFocusableInput(el.shadowRoot);
                            if (found) return found;
                        }}
                    }}
                    return null;
                }}

                const promptElement = findFocusableInput(document);
                if (promptElement) {{
                    const textToInject = `\n\n[Selected Text: "${{`{}`}}"]`;
                    
                    promptElement.focus();
                    try {{ promptElement.click(); }} catch (e) {{}}
                    
                    // We use execCommand because it flawlessly simulates user input 
                    // and triggers all React/Angular framework events automatically
                    const success = document.execCommand('insertText', false, textToInject);
                    
                    // Fallback if execCommand fails
                    if (!success) {{
                        if (promptElement.value !== undefined) {{
                            promptElement.value += textToInject;
                        }} else {{
                            promptElement.textContent += textToInject;
                        }}
                        const inputEvent = new Event('input', {{ bubbles: true }});
                        promptElement.dispatchEvent(inputEvent);
                    }}
                    
                    // Move cursor to the beginning so the user can type their prompt
                    try {{
                        if (promptElement.setSelectionRange) {{
                            // For textarea or input
                            promptElement.setSelectionRange(0, 0);
                        }} else {{
                            // For contenteditable div/p
                            const range = document.createRange();
                            const sel = window.getSelection();
                            range.setStart(promptElement, 0);
                            range.collapse(true);
                            sel.removeAllRanges();
                            sel.addRange(range);
                        }}
                    }} catch (e) {{}}
                }}
            }})();
        "#, text.replace("`", "\\`").replace("$", "\\$"));
        
        if let Err(e) = webview.eval(&script) { eprintln!("Failed to eval inject_text: {:?}", e); }
    }
}

pub fn toggle_window(app: &tauri::AppHandle) {
    let state = app.state::<AppState>();
    let is_visible = state.window_visible.load(Ordering::Acquire);
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

pub fn grab_text_and_toggle_window(app: &tauri::AppHandle) {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    
    let old_text = app.clipboard().read_text().unwrap_or_default();
    
    use enigo::{Enigo, Key, Keyboard, Settings, Direction};
    if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
        // Force release modifiers that might be held down from the global hotkey
        let _ = enigo.key(Key::Alt, Direction::Release);
        let _ = enigo.key(Key::Shift, Direction::Release);
        let _ = enigo.key(Key::Meta, Direction::Release);

        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::Unicode('c'), Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);
    }
    
    // Give the OS time to process the copy
    std::thread::sleep(std::time::Duration::from_millis(150));
    
    let new_text = app.clipboard().read_text().unwrap_or_default();
    
    if old_text != new_text && !new_text.is_empty() {
        let _ = app.emit("text-selected", new_text.clone());
        let _ = app.clipboard().write_text(old_text);
    }
    
    toggle_window(app);
}
