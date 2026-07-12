use tauri::{Manager, Emitter};
use std::sync::atomic::Ordering;
use crate::state::AppState;


pub trait WindowExt {
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
            tokio::time::sleep(std::time::Duration::from_millis(400)).await;
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

const FIND_INPUT_SCRIPT: &str = include_str!("../scripts/find_input.js");
const WEBVIEW_BACK_SCRIPT: &str = include_str!("../scripts/webview_back.js");
const WEBVIEW_RELOAD_SCRIPT: &str = include_str!("../scripts/webview_reload.js");
const WEBVIEW_FOCUS_SCRIPT: &str = include_str!("../scripts/webview_focus.js");
const WEBVIEW_INJECT_SCRIPT: &str = include_str!("../scripts/webview_inject.js");
const WEBVIEW_SNIP_SCRIPT: &str = include_str!("../scripts/webview_snip.js");

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
        let script = format!("{}\n{}", FIND_INPUT_SCRIPT, WEBVIEW_FOCUS_SCRIPT);
        if let Err(e) = webview.eval(&script) { eprintln!("Failed to eval focus script: {:?}", e); }
        
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
        let mut template = app.state::<crate::state::AppState>().custom_prompt.lock().unwrap().clone();
        if template.is_empty() {
            template = "[Selected Text: \"{text}\"]".to_string();
        }
        let injected_text = format!("\n\n{}", template.replace("{text}", &text));
        
        let escaped_text = serde_json::to_string(&injected_text).unwrap_or_else(|_| "\"\"".to_string());
        let script = format!(
            "window.__gemini_injected_text = {};\n{}\n{}",
            escaped_text, FIND_INPUT_SCRIPT, WEBVIEW_INJECT_SCRIPT
        );
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
            let app_clone = app.clone();
            let main_windows_clone = main_windows.clone();
            
            tauri::async_runtime::spawn(async move {
                #[cfg(target_os = "windows")]
                {
                    use winapi::um::winuser::{GetAsyncKeyState, VK_MENU, VK_CONTROL, VK_SHIFT, VK_LWIN, VK_RWIN};
                    // Await physical modifier release before window hiding to prevent OS hotkey leakage.
                    for _ in 0..20 {
                        unsafe {
                            let mods_down = GetAsyncKeyState(VK_MENU) < 0
                                || GetAsyncKeyState(VK_CONTROL) < 0
                                || GetAsyncKeyState(VK_SHIFT) < 0
                                || GetAsyncKeyState(VK_LWIN) < 0
                                || GetAsyncKeyState(VK_RWIN) < 0;
                            if !mods_down {
                                break;
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                    }
                }
                
                let state = app_clone.state::<AppState>();
                for window in &main_windows_clone {
                    if let Err(e) = window.hide() { eprintln!("Failed to hide window: {:?}", e); }
                }
                state.window_visible.store(false, Ordering::Release);
                state.window_focused.store(false, Ordering::Release);
                crate::process::optimize_memory(app_clone);
            });
        } else {
            for window in &main_windows {
                if let Err(e) = window.show_and_focus() { eprintln!("Failed to show window: {:?}", e); }
                if let Err(e) = app.emit("tray_state_changed", true) { eprintln!("Failed to emit event: {:?}", e); }
                if let Err(e) = window.emit("request-show", ()) { eprintln!("Failed to emit event: {:?}", e); }
            }
            state.window_visible.store(true, Ordering::Release);
            state.window_focused.store(true, Ordering::Release);
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

pub fn start_snip_mode(app: &tauri::AppHandle) {
    use tauri::Manager;
    let main_windows: Vec<tauri::Window> = app.windows().values().cloned().collect();
    for window in &main_windows {
        let _ = window.hide();
    }
    let state = app.state::<AppState>();
    state.window_visible.store(false, Ordering::Release);
    state.window_focused.store(false, Ordering::Release);
    std::thread::sleep(std::time::Duration::from_millis(50));

    #[cfg(target_os = "windows")]
    let old_sequence = unsafe { winapi::um::winuser::GetClipboardSequenceNumber() };
    #[cfg(not(target_os = "windows"))]
    let old_sequence = 0; // Fallback
    
    // Win + Shift + S
    #[cfg(target_os = "windows")]
    unsafe {
        use winapi::um::winuser::{keybd_event, VK_LWIN, VK_SHIFT, VK_MENU, KEYEVENTF_KEYUP};
        
        // Force release Alt just in case
        keybd_event(VK_MENU as u8, 0, KEYEVENTF_KEYUP, 0);
        
        keybd_event(VK_LWIN as u8, 0, 0, 0);
        keybd_event(VK_SHIFT as u8, 0, 0, 0);
        keybd_event(0x53, 0, 0, 0); // S key
        keybd_event(0x53, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_SHIFT as u8, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_LWIN as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        // Poll clipboard sequence within a bounded 15-second capture window while Windows Snipping Tool is active.
        for _ in 0..100 {
            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
            
            #[cfg(target_os = "windows")]
            let new_sequence = unsafe { winapi::um::winuser::GetClipboardSequenceNumber() };
            #[cfg(not(target_os = "windows"))]
            let new_sequence = 1; // Fallback
            
            if new_sequence != old_sequence {
                // Clipboard changed, meaning the snip session ended.
                // Immediately restore the window so it never stays hidden.
                crate::ipc::window::toggle_window(&app_clone);
                
                // Retry reading the clipboard up to 10 times in case Snipping Tool is still holding the lock.
                for _ in 0..10 {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if let Ok(new_image) = clipboard.get_image() {
                            use image::{ImageBuffer, RgbaImage};
                            let image_buffer: Option<RgbaImage> = ImageBuffer::from_raw(
                                new_image.width as u32,
                                new_image.height as u32,
                                new_image.bytes.into_owned(),
                            );
                            
                            if let Some(img) = image_buffer {
                                let mut cursor = std::io::Cursor::new(Vec::new());
                                if img.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
                                    use base64::{Engine as _, engine::general_purpose};
                                    let b64 = general_purpose::STANDARD.encode(cursor.into_inner());
                                    
                                    use tauri::Manager;
                                    if let Some(webview) = app_clone.get_webview("gemini") {
                                        let escaped_b64 = serde_json::to_string(&b64).unwrap_or_else(|_| "\"\"".to_string());
                                        let script = format!(
                                            "window.__gemini_snip_b64 = {};\n{}\n{}",
                                            escaped_b64, FIND_INPUT_SCRIPT, WEBVIEW_SNIP_SCRIPT
                                        );
                                        let _ = webview.eval(&script);
                                    }
                                }
                            }
                            break;
                        }
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                }
                break;
            }
        }
    });
}
