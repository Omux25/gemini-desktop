use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use std::sync::atomic::Ordering;
use crate::state::AppState;
use crate::process::optimize_memory;
use crate::ipc::window::toggle_window;

pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

    let tray_builder = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                for (label, window) in app.windows() {
                    if !label.starts_with("plugin:") {
                        if let Err(e) = window.show() { log::error!("Failed to show window from tray: {}", e); }
                        if let Err(e) = window.unminimize() { log::error!("Failed to unminimize window from tray: {}", e); }
                        if let Err(e) = window.set_focus() { log::error!("Failed to focus window from tray: {}", e); }
                    }
                }
                let state = app.state::<AppState>();
                state.window_visible.store(true, Ordering::SeqCst);
            }
            "hide" => {
                for (label, window) in app.windows() {
                    if !label.starts_with("plugin:") {
                        if let Err(e) = window.hide() {
                            log::error!("Failed to hide window from tray: {}", e);
                        }
                    }
                }
                let state = app.state::<AppState>();
                state.window_visible.store(false, Ordering::Release);
                optimize_memory(app.clone());
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                toggle_window(&tray.app_handle());
            }
            _ => {}
        });

    if let Err(e) = tray_builder.build(app) {
        log::error!("Failed to build tray icon: {}", e);
    }

    Ok(())
}
