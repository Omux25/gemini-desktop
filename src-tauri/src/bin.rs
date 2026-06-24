use tauri_plugin_global_shortcut::Shortcut;
fn main() {
    let s1: Result<Shortcut, _> = "ctrl+shift+g".parse();
    let s2: Result<Shortcut, _> = "f12".parse();
    println!("s1: {:?}", s1.is_ok());
    println!("s2: {:?}", s2.is_ok());
}
