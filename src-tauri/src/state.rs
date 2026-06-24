use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    pub window_visible: AtomicBool,
    pub window_pinned: AtomicBool,
    pub window_focused: AtomicBool,
    pub is_trimming_loop_running: AtomicBool,
    pub smooth_mode: AtomicBool,
    pub global_hotkey: Mutex<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            window_visible: AtomicBool::new(false),
            window_pinned: AtomicBool::new(false),
            window_focused: AtomicBool::new(false),
            is_trimming_loop_running: AtomicBool::new(false),
            smooth_mode: AtomicBool::new(false),
            global_hotkey: Mutex::new(String::new()),
        }
    }
}
