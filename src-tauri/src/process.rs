use std::sync::atomic::Ordering;
use tauri::Manager;
use crate::state::AppState;

/// The `process` module provides low-level OS process management and optimization utilities.
/// 
/// This is particularly useful for Tauri applications running Chromium (Webview2),
/// which can consume significant memory while idling in the background. 
/// 
/// The functions here interact directly with the OS APIs (e.g., Windows API) to 
/// trim the process working set, moving inactive memory pages to the system pagefile
/// when the application is minimized or hidden.

#[cfg(target_os = "windows")]
mod windows_api {
    use std::collections::{HashMap, HashSet};
    use sysinfo::System;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::EmptyWorkingSet;
    use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA};
    use winapi::um::handleapi::CloseHandle;

    /// Builds a set of process IDs representing the process tree starting from a given root.
    /// This is used to capture the main application and all spawned Webview2 renderers.
    pub fn get_process_tree(root_pid: u32) -> HashSet<u32> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let mut pids = HashSet::new();
        pids.insert(root_pid);

        // Webview2 processes are sometimes detached from the main process tree by the OS.
        // We aggressively search for any msedgewebview2.exe processes that have our app's identifier in their command line.
        for (pid, process) in sys.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains("msedgewebview2") {
                let cmd = process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect::<Vec<_>>().join(" ").to_lowercase();
                if cmd.contains("com.omux2.geminidesktop") || cmd.contains("gemini") || cmd.contains("tauri-app") {
                    pids.insert(pid.as_u32());
                }
            }
        }
        
        let mut parent_map: HashMap<u32, Vec<u32>> = HashMap::new();
        for (pid, process) in sys.processes() {
            if let Some(parent) = process.parent() {
                parent_map.entry(parent.as_u32()).or_default().push(pid.as_u32());
            }
        }
        
        let mut to_visit: Vec<u32> = pids.iter().copied().collect();
        
        while let Some(current_pid) = to_visit.pop() {
            if let Some(children) = parent_map.get(&current_pid) {
                for child in children {
                    if pids.insert(*child) {
                        to_visit.push(*child);
                    }
                }
            }
        }
        
        pids
    }

    /// Trims the memory working set of the current process and all its children.
    /// Returns the number of processes successfully trimmed.
    pub fn optimize_working_set() -> Result<usize, String> {
        let my_pid = sysinfo::get_current_pid()
            .map_err(|e| format!("Failed to get current process ID: {}", e))?;
            
        let pids = get_process_tree(my_pid.as_u32());
        let mut success_count = 0;
        
        for pid in pids {
            // SAFETY: OpenProcess and EmptyWorkingSet are safe if the handle is valid.
            // We check for null before using the handle and ensure it is properly closed.
            unsafe {
                let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_SET_QUOTA, 0, pid);
                if !handle.is_null() {
                    let result = EmptyWorkingSet(handle);
                    if result != 0 {
                        success_count += 1;
                    }
                    CloseHandle(handle);
                }
            }
        }
        
        Ok(success_count)
    }
}

/// Command to asynchronously optimize the application's memory usage footprint.
/// Spawns a background task to allow the UI to finish its current render frame
/// before performing the OS-level memory optimizations.
#[cfg(target_os = "windows")]
#[tauri::command]
pub fn optimize_memory(app: tauri::AppHandle) {
    let state = app.state::<AppState>();
    
    // Prevent concurrent optimization loops
    if state.is_trimming_loop_running.swap(true, Ordering::Acquire) {
        return; 
    }

    tauri::async_runtime::spawn(async move {
        // Wait briefly for UI transitions to complete before pausing memory
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        let state = app.state::<AppState>();
        if !state.window_visible.load(Ordering::Acquire) {
            if let Err(e) = windows_api::optimize_working_set() {
                log::warn!("Background memory optimization failed: {}", e);
            } else {
                log::debug!("Background memory optimization completed successfully.");
            }
        }
        
        state.is_trimming_loop_running.store(false, Ordering::Release);
    });
}

/// Stub for non-Windows platforms. Memory optimization is less aggressive on macOS/Linux
/// where the OS manages application memory compression more effectively out-of-the-box.
#[cfg(not(target_os = "windows"))]
#[tauri::command]
pub fn optimize_memory(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let state = app.state::<crate::state::AppState>();
        state.is_trimming_loop_running.store(false, Ordering::Release);
    });
}
