//! Minimal reproduction of macOS keyboard shortcuts (Cmd+C/V/X/A/Z) not
//! working in a Tauri v2 webview even though the equivalent menu items work.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // The default menu includes the Edit submenu (Undo/Redo/Cut/Copy/Paste/
        // Select All). Clicking those items works; their keyboard equivalents
        // do not.
        .menu(|handle| tauri::menu::Menu::default(handle))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
