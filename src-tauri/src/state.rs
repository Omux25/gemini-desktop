use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

pub struct AppState {
    pub window_visible: AtomicBool,
    pub window_pinned: AtomicBool,
    pub window_focused: AtomicBool,
    pub is_trimming_loop_running: AtomicBool,
    pub smooth_mode: AtomicBool,
    pub auto_hide: AtomicBool,
    pub last_shown_time: std::sync::atomic::AtomicU64,
    pub startup_time: std::sync::atomic::AtomicU64,
    pub hotkey_toggle: Mutex<String>,
    pub hotkey_copy: Mutex<String>,
    pub hotkey_snip: Mutex<String>,
    pub custom_prompt: Mutex<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            window_visible: AtomicBool::new(false),
            window_pinned: AtomicBool::new(false),
            window_focused: AtomicBool::new(false),
            is_trimming_loop_running: AtomicBool::new(false),
            smooth_mode: AtomicBool::new(false),
            auto_hide: AtomicBool::new(false),
            last_shown_time: std::sync::atomic::AtomicU64::new(0),
            startup_time: std::sync::atomic::AtomicU64::new(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64),
            hotkey_toggle: Mutex::new(String::new()),
            hotkey_copy: Mutex::new(String::new()),
            hotkey_snip: Mutex::new(String::new()),
            custom_prompt: Mutex::new(String::new()),
        }
    }
}
