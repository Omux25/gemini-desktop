pub fn load_smooth_mode() -> bool {
    let mut is_smooth_mode = false;
    #[cfg(target_os = "windows")]
    {
        if let Some(mut file_path) = dirs::data_local_dir() {
            file_path.push("com.gemini.desktop");
            file_path.push("settings.json");
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
