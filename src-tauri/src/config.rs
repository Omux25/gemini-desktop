pub const APP_ID: &str = "com.omux25.geminidesktop";

pub fn get_settings_file_path() -> Option<std::path::PathBuf> {
    dirs::data_dir().map(|mut path| {
        path.push(APP_ID);
        path.push("settings.json");
        path
    })
}

pub fn load_smooth_mode() -> bool {
    let mut is_smooth_mode = false;
    #[cfg(target_os = "windows")]
    {
        if let Some(file_path) = get_settings_file_path() {
            if let Ok(contents) = std::fs::read_to_string(file_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) {
                    if let Some(val) = json.get("isSmoothMode") {
                        if val.as_bool() == Some(true) || val.as_str() == Some("true") {
                            is_smooth_mode = true;
                        }
                    }
                }
            }
        }
    }
    is_smooth_mode
}

pub fn apply_chromium_flags(is_smooth_mode: bool) {
    #[cfg(target_os = "windows")]
    {
        if !is_smooth_mode {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                "--enable-low-end-device-mode --disable-site-isolation-trials --renderer-process-limit=1 --js-flags=\"--max-old-space-size=128\"",
            );
        }
    }
}
