use std::env;
use std::fs;
use std::process::Command;
use tauri::{AppHandle, Emitter};
use reqwest::Client;
use futures_util::StreamExt;
use std::io::Write;

#[derive(Clone, serde::Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
}

#[tauri::command]
pub fn is_portable() -> bool {
    let exe = env::current_exe().unwrap_or_default();
    let exe_str = exe.to_string_lossy().to_lowercase();
    
    // If the path contains AppData\Local\Programs or Program Files, it's installed.
    if exe_str.contains("appdata\\local\\programs") || exe_str.contains("program files") {
        return false;
    }
    
    // If it has an uninstall.exe next to it, it's definitely installed
    let uninstall_path = exe.with_file_name("Uninstall Gemini Desktop.exe");
    if uninstall_path.exists() {
        return false;
    }
    
    true
}

#[tauri::command]
pub async fn download_portable_update(app: AppHandle, url: String) -> Result<String, String> {
    let current_exe = env::current_exe().map_err(|e| e.to_string())?;
    let mut update_exe = current_exe.clone();
    update_exe.set_extension("exe.new");

    let client = Client::builder()
        .user_agent("GeminiDesktopPortableUpdater")
        .build()
        .map_err(|e| e.to_string())?;

    // Enable redirects
    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
    
    let total_size = res.content_length().unwrap_or(0);
    
    app.emit("updater-download-started", DownloadProgress {
        downloaded: 0,
        total: total_size,
    }).map_err(|e| e.to_string())?;

    let mut file = fs::File::create(&update_exe).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        
        app.emit("updater-download-progress", DownloadProgress {
            downloaded,
            total: total_size,
        }).unwrap_or(());
    }

    app.emit("updater-download-finished", ()).unwrap_or(());

    Ok(update_exe.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn install_portable_update(_app: AppHandle, downloaded_path: String) -> Result<(), String> {
    let current_exe = env::current_exe().map_err(|e| e.to_string())?;
    let current_dir = current_exe.parent().unwrap_or(std::path::Path::new(""));
    let bat_path = current_dir.join("update_gemini.bat");

    let bat_content = format!(
        "@echo off\n\
        timeout /t 2 /nobreak > NUL\n\
        del \"{}\"\n\
        move /y \"{}\" \"{}\"\n\
        start \"\" \"{}\"\n\
        (goto) 2>nul & del \"%~f0\"",
        current_exe.to_string_lossy(),
        downloaded_path,
        current_exe.to_string_lossy(),
        current_exe.to_string_lossy()
    );

    fs::write(&bat_path, bat_content).map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        Command::new("cmd")
            .args(["/C", bat_path.to_string_lossy().as_ref()])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("sh")
            .args(["-c", bat_path.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    // Exit the app immediately
    std::process::exit(0);
}
